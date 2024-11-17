use crate::ast::{
    ArithmeticOperator, Expression, Identifier, IfCondition, Line, Literal, RelationOperator,
    Statement, VarDeclaration,
};
use crate::errors::RuntimeError;
use crate::parser::{self, Parser};
use std::collections::HashMap;
use std::io::Write;
use std::io::{stdin, stdout};

#[derive(Debug, Clone)]
enum Value {
    Number(usize),
    String(String),
    Boolean(bool),
    Error(RuntimeError),
    None,
}

pub struct ExecutionContext {
    variables: HashMap<String, Value>,
    program: [Option<Line>; 255],
    current_line: usize,
}

pub struct Interpreter {
    context: ExecutionContext,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            context: ExecutionContext {
                variables: HashMap::new(),
                program: [const { None }; 255],
                current_line: 0,
            },
        }
    }

    fn visit_expression(&self, expression: &Expression) -> Value {
        match expression {
            Expression::Identifier(identifier) => {
                let value = self.context.variables.get(&identifier.name);
                match value {
                    Some(value) => value.clone(),
                    None => Value::None,
                }
            }
            Expression::Literal(literal) => match literal {
                Literal::Number { value } => Value::Number(*value),
                Literal::String { value } => Value::String(value.clone()),
            },
            Expression::BinaryExpression(binary) => {
                let left = self.visit_expression(&binary.left);
                let right = self.visit_expression(&binary.right);

                match (left, right) {
                    (Value::Number(left), Value::Number(right)) => {
                        let result = match binary.operator {
                            ArithmeticOperator::Add => left + right,
                            ArithmeticOperator::Subtract => left - right,
                            ArithmeticOperator::Multiply => left * right,
                            ArithmeticOperator::Divide => left / right,
                        };

                        Value::Number(result)
                    }
                    (Value::String(left), Value::String(right)) => {
                        let result = match binary.operator {
                            ArithmeticOperator::Add => left + &right,
                            _ => {
                                return Value::Error(RuntimeError::InvalidOperation(
                                    self.context.current_line,
                                ))
                            }
                        };

                        Value::String(result)
                    }
                    _ => {
                        return Value::Error(RuntimeError::InvalidOperation(
                            self.context.current_line,
                        ))
                    }
                }
            }
            _ => return Value::Error(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    fn visit_print_statement(&self, expressions: &Vec<Expression>) {
        for expression in expressions {
            let value = self.visit_expression(expression);
            match value {
                Value::Number(value) => println!("{}", value),
                Value::String(value) => println!("{}", value),
                _ => {}
            }
        }
    }

    fn visit_if_statement(&mut self, condition: &IfCondition, then: &Box<Statement>) {
        let left = self.visit_expression(&condition.left);
        let right = self.visit_expression(&condition.right);

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
                    self.visit_statement(&then);
                }
            }
            _ => {}
        }
    }

    fn visit_run_statement(&mut self) {
        self.context.current_line = 0;

        while self.context.current_line < 255 {
            let line = self.context.program[self.context.current_line].clone();
            self.context.current_line += 1;

            match line {
                Some(line) => {
                    let statement = &line.statement;
                    self.visit_statement(statement);
                }
                None => {}
            }
        }
    }

    fn visit_var_statement(&mut self, declaration: &VarDeclaration) {
        let value = self.visit_expression(&declaration.value);
        self.context
            .variables
            .insert(declaration.name.to_string(), value);
    }

    fn visit_input_statement(&mut self, variables: &Vec<Identifier>) {
        for variable in variables {
            print!("{}? ", variable.name);
            stdout().flush().unwrap();

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();

            let mut parser = parser::Parser::new(&buffer);
            let expression = parser.parse_expression();

            match expression {
                Ok(expression) => {
                    let value = self.visit_expression(&expression);
                    self.context
                        .variables
                        .insert(variable.name.to_string(), value);
                }
                Err(error) => {
                    println!("Invalid input: {}", error);
                    return;
                }
            };
        }
    }

    fn visit_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::IfStatement { condition, then } => {
                self.visit_if_statement(&condition, &then)
            }
            Statement::PrintStatement { expressions } => self.visit_print_statement(expressions),
            Statement::VarStatement { declaration } => self.visit_var_statement(&declaration),
            Statement::InputStatement { variables } => self.visit_input_statement(variables),
            Statement::RunStatement => self.visit_run_statement(),
            _ => {}
        }
    }

    fn eval(&mut self, ast: Line) {
        if ast.number.is_some() {
            let line_number = ast.number.unwrap();
            self.context.program[line_number] = Some(ast);
        } else {
            self.visit_statement(&ast.statement);
        }
    }

    pub fn execute(&mut self, source: &str) {
        let mut parser = Parser::new(source);
        let ast = parser.parse();

        match ast {
            Ok(ast) => self.eval(ast),
            Err(error) => println!("{}", error),
        }
    }
}
