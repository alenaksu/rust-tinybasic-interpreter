use core::panic;
use std::vec;

use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind, TokenValue};

pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            lexer: Lexer::new(source),
        }
    }

    fn expect_token(&mut self, kinds: &[TokenKind], value: Option<TokenValue>) {
        let next_token = self.lexer.peek();
        let offeset = self.lexer.offset();
        if !kinds.contains(&next_token.kind) {
            panic!(
                "Error at position {:?}: expected token kind {:?}, but found {:?}",
                offeset, kinds, next_token.kind
            );
        } else if value.is_some() {
            let expected_value = value.unwrap().clone();
            if next_token.value != expected_value {
                panic!(
                    "Error at position {:?}: expected token value {:?}, but found {:?}",
                    offeset, expected_value, next_token.value
                );
            }
        }
    }

    fn parse_unary_expression(&mut self) -> Expression {
        let next_token = self.lexer.peek();
        let operator = match next_token.kind {
            TokenKind::Add => Some(UnaryOperator::Plus),
            TokenKind::Subtract => Some(UnaryOperator::Minus),
            _ => None,
        };

        if operator.is_some() {
            self.lexer.next();
        }

        self.expect_token(
            &[
                TokenKind::NumberLiteral,
                TokenKind::Identifier,
                TokenKind::LeftParen,
            ],
            None,
        );

        let next_token = self.lexer.next();
        let argument = match next_token.kind {
            TokenKind::LeftParen => {
                self.lexer.next();
                let expression = self.parse_expression();

                self.expect_token(&[TokenKind::RightParen], None);
                self.lexer.next();

                expression
            }
            TokenKind::Identifier => Expression::Identifier {
                name: match next_token.value {
                    TokenValue::String(s) => s.clone(),
                    _ => unreachable!(),
                },
            },
            TokenKind::NumberLiteral => Expression::Number {
                value: match next_token.value {
                    TokenValue::Digit(d) => d,
                    _ => unreachable!(),
                },
            },
            _ => unreachable!(),
        };

        Expression::UnaryExpression {
            expression: UnaryExpression {
                operator: operator,
                argument: Box::new(argument),
            },
        }
    }

    fn parse_expression(&mut self) -> Expression {
        let left = self.parse_unary_expression();

        let next_token = self.lexer.peek();
        let operator = match next_token.kind {
            TokenKind::Add => Some(ArithmeticOperator::Add),
            TokenKind::Subtract => Some(ArithmeticOperator::Subtract),
            TokenKind::Multiply => Some(ArithmeticOperator::Multiply),
            TokenKind::Divide => Some(ArithmeticOperator::Divide),
            _ => None,
        };

        if operator.is_none() {
            return left;
        }

        // skip operator
        self.lexer.next();

        let right = self.parse_unary_expression();

        Expression::BinaryExpression {
            expression: BinaryExpression {
                operator: operator.unwrap(),
                left: Box::new(left),
                right: Box::new(right),
            },
        }
    }

    fn parse_print_statement(&mut self) -> Statement {
        let mut expressions: Vec<Expression> = vec![];
        while self.lexer.peek().kind != TokenKind::Eol {
            expressions.push(self.parse_expression());
        }

        Statement::PrintStatement { expressions }
    }

    fn parse_if_statement(&mut self) -> Statement {
        let left = self.parse_expression();

        self.expect_token(
            &[
                TokenKind::Equal,
                TokenKind::LessThan,
                TokenKind::GreaterThan,
                TokenKind::LessThanOrEqual,
                TokenKind::GreaterThanOrEqual,
            ],
            None,
        );

        let next_token = self.lexer.next();
        let relation_operator = match next_token.kind {
            TokenKind::Equal => RelationOperator::Equal,
            TokenKind::LessThan => RelationOperator::LessThan,
            TokenKind::LessThanOrEqual => RelationOperator::LessThanOrEqual,
            TokenKind::GreaterThan => RelationOperator::GreaterThan,
            TokenKind::GreaterThanOrEqual => RelationOperator::GreaterThanOrEqual,
            _ => unreachable!(),
        };

        let right = self.parse_expression();

        self.expect_token(
            &[TokenKind::Identifier],
            Some(TokenValue::String(String::from("THEN"))),
        );

        // skip THEN
        self.lexer.next();

        let then = self.parse_statement();

        return Statement::IfStatement {
            condition: IfCondition {
                operator: relation_operator,
                left,
                right,
            },
            then: Box::new(then),
        };
    }

    fn parse_var_statement(&mut self) -> Statement {
        self.expect_token(&[TokenKind::Identifier], None);

        let next_token = self.lexer.next();
        let name = next_token.value;
        let value = self.parse_expression();

        return Statement::VarStatement {
            declaration: VarDeclaration {
                name: match name {
                    TokenValue::String(s) => s.clone(),
                    _ => unreachable!(),
                },
                value,
            },
        };
    }

    fn parse_goto_statement(&mut self) -> Statement {
        let location = self.parse_expression();

        return Statement::GoToStatement { location };
    }

    fn parse_gosub_statement(&mut self) -> Statement {
        let location = self.parse_expression();

        return Statement::GoSubStatement { location };
    }

    fn parse_statement(&mut self) -> Statement {
        let next_token = self.lexer.next();
        let statement = match next_token.value.clone() {
            TokenValue::String(s) => match s.as_str() {
                "PRINT" => self.parse_print_statement(),
                "IF" => self.parse_if_statement(),
                "LET" => self.parse_var_statement(),
                "GOTO" => self.parse_goto_statement(),
                "GOSUB" => self.parse_gosub_statement(),
                "RETURN" => Statement::ReturnStatement,
                "END" => Statement::EndStatement,
                _ => Statement::Empty,
            },
            _ => Statement::Empty,
        };

        statement
    }

    pub fn parse(&mut self) -> Line {
        let next_token = self.lexer.peek();

        self.expect_token(
            &[
                TokenKind::Eol,
                TokenKind::Identifier,
                TokenKind::NumberLiteral,
            ],
            None,
        );

        match next_token.kind {
            TokenKind::Eol => Line {
                number: None,
                statement: Statement::Empty,
            },
            TokenKind::Identifier => Line {
                number: None,
                statement: self.parse_statement(),
            },
            TokenKind::NumberLiteral => {
                let line_number = match next_token.value {
                    TokenValue::Digit(number) => Some(number),
                    _ => None,
                };
                self.lexer.next();

                Line {
                    number: line_number,
                    statement: self.parse_statement(),
                }
            }
            _ => unreachable!(),
        }
    }
}
