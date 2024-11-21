use crate::ast::{
    ArithmeticOperator, Expression, Identifier, IfCondition, Line, Literal, RelationOperator,
    Statement, VarDeclaration,
};
use crate::errors::RuntimeError;
use crate::parser::Parser;
use std::collections::HashMap;
use std::fmt;

use wasm_bindgen::prelude::*;

use crate::io::{clear, read_line, set_prompt, write, write_line};

#[derive(Debug, Clone, PartialEq)]
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
    stack: Vec<usize>,
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
                stack: vec![0],
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

    async fn visit_if_statement(
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
                    Box::pin(self.visit_statement(&then)).await
                } else {
                    Ok(Value::None)
                }
            }
            _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    async fn visit_run_statement(&mut self) -> InterpreterResult {
        self.context.current_line = 0;
        self.context.stack.clear();

        let mut output: Vec<Value> = vec![];
        while self.context.current_line < 255 {
            let line = self.context.program[self.context.current_line].clone();
            self.context.current_line += 1;

            match line {
                Some(line) => {
                    let statement = &line.statement;
                    output.push(self.visit_statement(statement).await?);
                }
                None => {}
            }
        }

        Ok(Value::String(
            output
                .iter()
                .filter(|v| **v != Value::None)
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

    async fn visit_input_statement(&mut self, variables: &Vec<Identifier>) -> InterpreterResult {
        for variable in variables {
            set_prompt(format!("{}? ", variable.name).as_str());

            let input = read_line().await;
            write_line(format!("{}? {}", variable.name, input).as_str());

            let mut parser = Parser::new(input.as_str());
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

    fn visit_list_statement(&self) -> InterpreterResult {
        let mut output: Vec<String> = vec![];
        for line in self.context.program.iter() {
            match line {
                Some(line) => {
                    output.push(format!("{}", line.source.trim()));
                }
                None => {}
            }
        }

        Ok(Value::String(output.join("\n")))
    }

    fn visit_clear_statement(&self) -> InterpreterResult {
        clear();
        Ok(Value::None)
    }

    fn visit_goto_statement(&mut self, location: &Expression) -> InterpreterResult {
        let location = match self.visit_expression(location)? {
            Value::Number(number) => number,
            value => return Err(RuntimeError::IllegalLineNumber(format!("{}", value))),
        };

        self.context.current_line = location;


        Ok(Value::None)
    }

    fn visit_gosub_statement(&mut self, location: &Expression) -> InterpreterResult {
        self.context.stack.push(self.context.current_line);
        self.visit_goto_statement(location)?;

        Ok(Value::None)
    }

    fn visit_return_statement(&mut self) -> InterpreterResult {
        match self.context.stack.pop() {
            Some(location) => {
                self.context.current_line = location;
                Ok(Value::None)
            }
            None => Err(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    fn visit_end_statement(&mut self) -> InterpreterResult {
        self.context.current_line = self.context.program.len();
        Ok(Value::None)
    }

    async fn visit_statement(&mut self, statement: &Statement) -> InterpreterResult {
        match statement {
            Statement::IfStatement { condition, then } => {
                return self.visit_if_statement(&condition, &then).await;
            }
            Statement::PrintStatement { expressions } => {
                return self.visit_print_statement(expressions);
            }
            Statement::VarStatement { declaration } => {
                return self.visit_var_statement(&declaration);
            }
            Statement::InputStatement { variables } => {
                return self.visit_input_statement(variables).await;
            }
            Statement::GoToStatement { location } => {
                return self.visit_goto_statement(location);
            }
            Statement::GoSubStatement { location } => {
                return self.visit_gosub_statement(location);
            }
            Statement::EndStatement => {
                return self.visit_end_statement()
            }
            Statement::ListStatement => {
                return self.visit_list_statement();
            }
            Statement::RunStatement => {
                return Box::pin(self.visit_run_statement()).await;
            }
            Statement::ReturnStatement => {
                return self.visit_return_statement();
            }
            Statement::Empty => {
                return Ok(Value::None);
            }
            Statement::ClearStatement => {
                return self.visit_clear_statement();
            }
        }
    }

    async fn eval(&mut self, ast: Line) -> InterpreterResult {
        if ast.number.is_some() {
            let line_number = ast.number.unwrap();
            self.context.program[line_number] = Some(ast);
        } else {
            return self.visit_statement(&ast.statement).await;
        }

        Ok(Value::None)
    }

    pub async fn execute(&mut self) {
        write_line("Ready!");

        loop {
            set_prompt("> ");

            let source = read_line().await;
            write_line(format!(":{}", source).as_str());

            let mut parser = Parser::new(source.as_str());
            let ast = parser.parse();

            if ast.is_err() {
                write_line(format!("{}", ast.err().unwrap()).as_str());
                continue;
            }

            let value = self.eval(ast.ok().unwrap()).await;
            match value {
                Ok(value) => {
                    if value != Value::None {
                        write_line(format!("{}", value).as_str());
                    }
                }
                Err(error) => {
                    write_line(format!("{}", error).as_str());
                }
            }
        }
    }
}
