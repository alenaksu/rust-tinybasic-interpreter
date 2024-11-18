use crate::ast::{
    ArithmeticOperator, Expression, Identifier, IfCondition, Line, Literal, RelationOperator,
    Statement, VarDeclaration,
};
use crate::errors::RuntimeError;
use crate::parser::Parser;
use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, stdout, Write};

use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
enum Value {
    Number(usize),
    String(String),
    Boolean(bool),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "{}", string),
            Self::Boolean(boolean) => write!(f, "{}", if *boolean { "True" } else { "False" }),
            Self::None => write!(f, ""),
        }
    }
}

type InterpreterResult = std::result::Result<Value, RuntimeError>;

pub struct ExecutionContext {
    variables: HashMap<String, Value>,
    program: [Option<Line>; 255],
    current_line: usize,
}

#[wasm_bindgen]
pub struct Interpreter {
    context: ExecutionContext,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Interpreter {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Interpreter {
        Interpreter {
            context: ExecutionContext {
                variables: HashMap::new(),
                program: [const { None }; 255],
                current_line: 0,
            },
        }
    }

    fn visit_expression(&self, expression: &Expression) -> InterpreterResult {
        match expression {
            Expression::Identifier(identifier) => {
                let value = self.context.variables.get(&identifier.name);
                match value {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Value::None),
                }
            }
            Expression::Literal(literal) => match literal {
                Literal::Number { value } => Ok(Value::Number(*value)),
                Literal::String { value } => Ok(Value::String(value.clone())),
            },
            Expression::BinaryExpression(binary) => {
                let left = self.visit_expression(&binary.left)?;
                let right = self.visit_expression(&binary.right)?;

                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        let result = match binary.operator {
                            ArithmeticOperator::Add => left + right,
                            ArithmeticOperator::Subtract => left - right,
                            ArithmeticOperator::Multiply => left * right,
                            ArithmeticOperator::Divide => left / right,
                        };

                        Ok(Value::Number(result))
                    }
                    (Value::String(left), Value::String(right)) => {
                        let result = match binary.operator {
                            ArithmeticOperator::Add => left + &right,
                            _ => {
                                return Err(RuntimeError::InvalidOperation(
                                    self.context.current_line,
                                ))
                            }
                        };

                        Ok(Value::String(result))
                    }
                    _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
                }
            }
            _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    fn visit_print_statement(&self, expressions: &Vec<Expression>) -> InterpreterResult {
        let mut results: Vec<String> = vec![];
        for expression in expressions {
            let value = self.visit_expression(expression)?;

            results.push(match value {
                Value::Number(number) => number.to_string(),
                Value::String(string) => string,
                _ => String::from(""),
            });
        }

        Ok(Value::String(results.join(", ")))
    }

    fn visit_if_statement(
        &mut self,
        condition: &IfCondition,
        then: &Box<Statement>,
    ) -> InterpreterResult {
        let left = self.visit_expression(&condition.left)?;
        let right = self.visit_expression(&condition.right)?;

        match (left, right) {
            (Value::Number(left), Value::Number(right)) => {
                let result = match condition.operator {
                    RelationOperator::Equal => left == right,
                    RelationOperator::NotEqual => left != right,
                    RelationOperator::LessThan => left < right,
                    RelationOperator::LessThanOrEqual => left <= right,
                    RelationOperator::GreaterThan => left > right,
                    RelationOperator::GreaterThanOrEqual => left >= right,
                };

                if result {
                    self.visit_statement(&then)
                } else {
                    Ok(Value::None)
                }
            }
            _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    fn visit_run_statement(&mut self) -> InterpreterResult {
        self.context.current_line = 0;

        let mut output: Vec<Value> = vec![];
        while self.context.current_line < 255 {
            let line = self.context.program[self.context.current_line].clone();
            self.context.current_line += 1;

            match line {
                Some(line) => {
                    let statement = &line.statement;
                    output.push(self.visit_statement(statement)?);
                }
                None => {}
            }
        }

        Ok(Value::String(
            output
                .iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join("\n"),
        ))
    }

    fn visit_var_statement(&mut self, declaration: &VarDeclaration) -> InterpreterResult {
        let value = self.visit_expression(&declaration.value)?;
        self.context
            .variables
            .insert(declaration.name.to_string(), value);

        Ok(Value::None)
    }

    fn visit_input_statement(&mut self, variables: &Vec<Identifier>) -> InterpreterResult {
        for variable in variables {
            print!("{}? ", variable.name);
            stdout().flush().unwrap();

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();

            let mut parser = Parser::new(&buffer);
            let expression = parser.parse_expression();

            match expression {
                Ok(expression) => {
                    let value = self.visit_expression(&expression)?;
                    self.context
                        .variables
                        .insert(variable.name.to_string(), value);
                }
                Err(error) => return Err(RuntimeError::SyntaxError(error)),
            };
        }

        Ok(Value::None)
    }

    fn visit_statement(&mut self, statement: &Statement) -> InterpreterResult {
        match statement {
            Statement::IfStatement { condition, then } => {
                return self.visit_if_statement(&condition, &then);
            }
            Statement::PrintStatement { expressions } => {
                return self.visit_print_statement(expressions)
            }
            Statement::VarStatement { declaration } => {
                return self.visit_var_statement(&declaration);
            }
            Statement::InputStatement { variables } => {
                return self.visit_input_statement(variables);
            }
            Statement::GoToStatement { location } => {
                return Err(RuntimeError::NotImplemented(String::from("GOTO")));
            }
            Statement::GoSubStatement { location } => {
                return Err(RuntimeError::NotImplemented(String::from("GOSUB")));
            }
            Statement::EndStatement => {
                return Err(RuntimeError::NotImplemented(String::from("END")));
            }
            Statement::RunStatement => {
                return self.visit_run_statement();
            }
            Statement::ReturnStatement => {
                return Err(RuntimeError::NotImplemented(String::from("RETURN")));
            }
            Statement::Empty => {
                return Ok(Value::None);
            },
            Statement::ClearStatement => {
                Ok(Value::String("\x0C".to_string()))
            }
        }
    }

    fn eval(&mut self, ast: Line) -> InterpreterResult {
        if ast.number.is_some() {
            let line_number = ast.number.unwrap();
            self.context.program[line_number] = Some(ast);
        } else {
            return self.visit_statement(&ast.statement);
        }

        Ok(Value::None)
    }

    fn internal_execute(&mut self, source: &str) -> Result<String, RuntimeError> {
        let mut parser = Parser::new(source);
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                let value = self.eval(ast)?;
                Ok(format!("{}", value))
            }
            Err(error) => Err(RuntimeError::SyntaxError(error)),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn execute(&mut self, source: &str) -> Result<String, JsError> {
        let result = self.internal_execute(source);

        match result {
            Ok(value) => Ok(value),
            Err(error) => Err(JsError::new(&format!("{}", error))),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn execute(&mut self, source: &str) -> Result<String, RuntimeError> {
        let result = self.internal_execute(source)?;

        Ok(result)
    }
}
