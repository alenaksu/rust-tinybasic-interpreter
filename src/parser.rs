use std::vec;

use crate::ast::*;
use crate::errors::SyntaxError;
use crate::lexer::{Lexer, TokenKind, TokenValue};

pub type ParseResult<T> = Result<T, SyntaxError>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    source: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            lexer: Lexer::new(source),
            source,
        }
    }

    fn expect_token(&mut self, kinds: &[TokenKind], value: Option<TokenValue>) -> ParseResult<()> {
        let next_token = self.lexer.peek()?;

        if !kinds.contains(&next_token.kind) {
            return Err(SyntaxError::UnexpectedToken(next_token));
        } else if value.is_some() {
            let expected_value = value.unwrap().clone();
            if next_token.value != expected_value {
                return Err(SyntaxError::UnexpectedToken(next_token));
            }
        }
        Ok(())
    }

    fn parse_unary_expression(&mut self) -> ParseResult<Expression> {
        let next_token = self.lexer.peek()?;
        let operator = match next_token.kind {
            TokenKind::Add => Some(UnaryOperator::Plus),
            TokenKind::Subtract => Some(UnaryOperator::Minus),
            _ => None,
        };

        if operator.is_some() {
            self.lexer.next()?;
        }

        self.expect_token(
            &[
                TokenKind::StringLiteral,
                TokenKind::NumberLiteral,
                TokenKind::Identifier,
                TokenKind::LeftParen,
            ],
            None,
        )?;

        let next_token = self.lexer.next()?;
        let argument: Expression = match next_token.kind {
            TokenKind::LeftParen => {
                let expression = self.parse_expression()?;

                self.expect_token(&[TokenKind::RightParen], None)?;
                self.lexer.next()?;

                expression
            }
            TokenKind::Identifier => Expression::Identifier(Identifier {
                name: match next_token.value {
                    TokenValue::String(s) => s.clone(),
                    _ => unreachable!(),
                },
            }),
            TokenKind::NumberLiteral => Expression::Literal(Literal::Number {
                value: match next_token.value {
                    TokenValue::Digit(d) => d,
                    _ => unreachable!(),
                },
            }),
            TokenKind::StringLiteral => Expression::Literal(Literal::String {
                value: match next_token.value {
                    TokenValue::String(s) => s.clone(),
                    _ => unreachable!(),
                },
            }),
            _ => unreachable!(),
        };

        if operator.is_none() {
            return Ok(argument);
        }

        Ok(Expression::UnaryExpression(UnaryExpression {
            operator,
            argument: Box::new(argument),
        }))
    }

    fn get_expression_precedence(&self, operator: &ArithmeticOperator) -> usize {
        match operator {
            ArithmeticOperator::Add | ArithmeticOperator::Subtract => 1,
            ArithmeticOperator::Multiply | ArithmeticOperator::Divide => 2,
        }
    }

    pub fn parse_expression(&mut self) -> ParseResult<Expression> {
        let left = self.parse_unary_expression()?;

        let operator = match self.lexer.peek()?.kind {
            TokenKind::Add => Some(ArithmeticOperator::Add),
            TokenKind::Subtract => Some(ArithmeticOperator::Subtract),
            TokenKind::Multiply => Some(ArithmeticOperator::Multiply),
            TokenKind::Divide => Some(ArithmeticOperator::Divide),
            _ => None,
        };

        if operator.is_none() {
            return Ok(left);
        }

        // skip operator
        self.lexer.next()?;

        let right = self.parse_expression()?;
        let operator = operator.unwrap();

        match &right {
            Expression::BinaryExpression(right_expression) => {
                let left_precedence = self.get_expression_precedence(&operator);
                let right_precedence = self.get_expression_precedence(&right_expression.operator);

                if left_precedence < right_precedence {
                    return Ok(Expression::BinaryExpression(BinaryExpression {
                        operator,
                        left: Box::new(right),
                        right: Box::new(left),
                    }));
                }
            }
            _ => {}
        };

        Ok(Expression::BinaryExpression(BinaryExpression {
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }))
    }

    fn parse_print_statement(&mut self) -> ParseResult<Statement> {
        let mut expressions: Vec<Expression> = vec![];
        loop {
            let expression = self.parse_expression()?;

            expressions.push(expression);
            if self.lexer.peek()?.kind != TokenKind::Comma {
                return Ok(Statement::PrintStatement { expressions });
            }

            self.lexer.next()?;
        }
    }

    fn parse_input_statement(&mut self) -> ParseResult<Statement> {
        let mut variables: Vec<Identifier> = vec![];
        loop {
            self.expect_token(&[TokenKind::Identifier], None)?;
            variables.push(Identifier {
                name: match self.lexer.next()?.value {
                    TokenValue::String(s) => s.clone(),
                    _ => unreachable!(),
                },
            });

            if self.lexer.peek()?.kind != TokenKind::Comma {
                return Ok(Statement::InputStatement { variables });
            }

            self.lexer.next()?;
        }
    }

    fn parse_if_statement(&mut self) -> ParseResult<Statement> {
        let left = self.parse_expression()?;

        self.expect_token(
            &[
                TokenKind::Equal,
                TokenKind::NotEqual,
                TokenKind::LessThan,
                TokenKind::GreaterThan,
                TokenKind::LessThanOrEqual,
                TokenKind::GreaterThanOrEqual,
            ],
            None,
        )?;

        let next_token = self.lexer.next()?;
        let relation_operator = match next_token.kind {
            TokenKind::Equal => RelationOperator::Equal,
            TokenKind::NotEqual => RelationOperator::NotEqual,
            TokenKind::LessThan => RelationOperator::LessThan,
            TokenKind::LessThanOrEqual => RelationOperator::LessThanOrEqual,
            TokenKind::GreaterThan => RelationOperator::GreaterThan,
            TokenKind::GreaterThanOrEqual => RelationOperator::GreaterThanOrEqual,
            _ => unreachable!(),
        };

        let right = self.parse_expression()?;

        self.expect_token(
            &[TokenKind::Identifier],
            Some(TokenValue::String(String::from("THEN"))),
        )?;

        // skip THEN
        self.lexer.next()?;

        let then = self.parse_statement()?;

        return Ok(Statement::IfStatement {
            condition: IfCondition {
                operator: relation_operator,
                left,
                right,
            },
            then: Box::new(then),
        });
    }

    fn parse_var_statement(&mut self) -> ParseResult<Statement> {
        self.expect_token(&[TokenKind::Identifier], None)?;

        let next_token = self.lexer.next()?;
        let name = next_token.value;

        match name.clone() {
            TokenValue::String(name) => {
                if name.len() > 1 {
                    return Err(SyntaxError::InvalidVariableName(
                        name,
                        next_token.span.start,
                    ));
                }
            }
            _ => {}
        };

        self.expect_token(&[TokenKind::Equal], None)?;
        self.lexer.next()?;

        let value = self.parse_expression()?;

        return Ok(Statement::VarStatement {
            declaration: VarDeclaration {
                name: match name {
                    TokenValue::String(s) => s.clone(),
                    _ => unreachable!(),
                },
                value,
            },
        });
    }

    fn parse_goto_statement(&mut self) -> ParseResult<Statement> {
        let location = self.parse_expression()?;

        return Ok(Statement::GoToStatement { location });
    }

    fn parse_gosub_statement(&mut self) -> ParseResult<Statement> {
        let location = self.parse_expression()?;

        return Ok(Statement::GoSubStatement { location });
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        let next_token = self.lexer.next()?;
        let statement = match next_token.value.clone() {
            TokenValue::String(s) => match s.as_str() {
                "PRINT" => self.parse_print_statement(),
                "INPUT" => self.parse_input_statement(),
                "IF" => self.parse_if_statement(),
                "LET" => self.parse_var_statement(),
                "GOTO" => self.parse_goto_statement(),
                "GOSUB" => self.parse_gosub_statement(),
                "RUN" => Ok(Statement::RunStatement),
                "LIST" => Ok(Statement::ListStatement),
                "CLEAR" => Ok(Statement::ClearStatement),
                "RETURN" => Ok(Statement::ReturnStatement),
                "END" => Ok(Statement::EndStatement),
                "HELP" => Ok(Statement::HelpStatement),
                _ => Err(SyntaxError::UnexpectedIdentifier(s, next_token.span.start)),
            },
            _ => Err(SyntaxError::UnexpectedToken(next_token)),
        };

        statement
    }

    pub fn parse(&mut self) -> ParseResult<Line> {
        let next_token = self.lexer.peek()?;

        self.expect_token(
            &[
                TokenKind::Eol,
                TokenKind::Identifier,
                TokenKind::NumberLiteral,
            ],
            None,
        )?;

        match next_token.kind {
            TokenKind::Eol => Ok(Line {
                number: None,
                statement: Statement::Empty,
                source: String::from(self.source),
            }),
            TokenKind::Identifier => Ok(Line {
                number: None,
                statement: self.parse_statement()?,
                source: String::from(self.source),
            }),
            TokenKind::NumberLiteral => {
                let line_number = match next_token.value {
                    TokenValue::Digit(number) => Some(number),
                    _ => None,
                };
                self.lexer.next()?;

                Ok(Line {
                    number: line_number,
                    statement: self.parse_statement()?,
                    source: String::from(self.source),
                })
            }
            _ => unreachable!(),
        }
    }
}
