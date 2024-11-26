use crate::ast::{
    ArithmeticOperator, BinaryExpression, Expression, Identifier, IfCondition, Line, Literal,
    RelationOperator, Statement, UnaryExpression, UnaryOperator, VarDeclaration,
};
use crate::errors::RuntimeError;
use crate::parser::Parser;
use crate::program::Program;
use std::collections::HashMap;
use std::fmt;

use wasm_bindgen::prelude::*;

use crate::io::{clear, load_file, read_line, save_file, set_prompt, write_line};

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Number(f32),
    String(String),
    // Boolean(bool),
    None,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::String(string) => write!(f, "{}", string),
            // Self::Boolean(boolean) => write!(f, "{}", if *boolean { "True" } else { "False" }),
            Self::None => write!(f, ""),
        }
    }
}

type InterpreterResult = std::result::Result<Value, RuntimeError>;

pub struct RuntimeContext {
    variables: HashMap<String, Value>,
    program: Program,
    stack: Vec<usize>,
    current_line: usize,
}

pub enum InterpreterState {
    Running,
    Stopped,
}

#[wasm_bindgen]
pub struct Interpreter {
    context: RuntimeContext,
    state: InterpreterState,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl Interpreter {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(constructor))]
    pub fn new() -> Interpreter {
        Interpreter {
            context: RuntimeContext {
                variables: HashMap::new(),
                program: Program::new(),
                stack: vec![0],
                current_line: 0,
            },
            state: InterpreterState::Stopped,
        }
    }

    fn reset(&mut self) {
        self.context.current_line = 0;
        self.context.stack.clear();
        self.context.variables.clear();
    }

    fn new_program(&mut self) {
        self.context.program.clear();
        self.reset();
    }

    fn visit_expression(&self, expression: &Expression) -> InterpreterResult {
        match expression {
            Expression::Identifier(identifier) => {
                let value = self.context.variables.get(&identifier.name);
                match value {
                    Some(value) => Ok(value.clone()),
                    None => Err(RuntimeError::UndefinedVariable(
                        format!("{}", identifier.name).to_string(),
                    )),
                }
            }
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::UnaryExpression(unary) => self.visit_unary_expression(unary),
            Expression::BinaryExpression(binary) => self.visit_binary_expression(binary),
            _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    fn visit_literal(&self, literal: &Literal) -> InterpreterResult {
        match literal {
            Literal::Number { value } => Ok(Value::Number(*value)),
            Literal::String { value } => Ok(Value::String(value.clone())),
        }
    }

    fn visit_binary_expression(&self, binary: &BinaryExpression) -> InterpreterResult {
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
                    _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
                };

                Ok(Value::String(result))
            }
            _ => return Err(RuntimeError::InvalidOperation(self.context.current_line)),
        }
    }

    fn visit_unary_expression(&self, unary: &UnaryExpression) -> InterpreterResult {
        let value = self.visit_expression(&unary.argument)?;

        match value {
            Value::Number(number) => match unary.operator {
                Some(UnaryOperator::Plus) => Ok(Value::Number(number)),
                Some(UnaryOperator::Minus) => Ok(Value::Number(-number)),
                None => Ok(Value::Number(number)),
            },
            _ => Err(RuntimeError::InvalidOperation(self.context.current_line)),
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

        Ok(Value::String(results.join(" ")))
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
        self.state = InterpreterState::Running;

        self.reset();

        while self.context.current_line < self.context.program.lines.len() {
            let line = self.context.program.get(self.context.current_line);
            self.context.current_line += 1;
            match line {
                Some(line) => {
                    let value = self.visit_statement(&line.statement).await?;

                    if value != Value::None {
                        write_line(format!("{}", value).as_str());
                    }
                }
                None => {}
            };
        }

        self.state = InterpreterState::Stopped;

        Ok(Value::None)
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
        Ok(Value::String(self.context.program.print()))
    }

    fn visit_cls_statement(&self) -> InterpreterResult {
        clear();
        Ok(Value::None)
    }

    fn visit_goto_statement(&mut self, location: &Expression) -> InterpreterResult {
        let location = match self.visit_expression(location)? {
            Value::Number(number) => number,
            value => return Err(RuntimeError::IllegalLineNumber(format!("{}", value))),
        };

        self.context.current_line = location as usize;

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
            None => Ok(Value::None),
        }
    }

    fn visit_end_statement(&mut self) -> InterpreterResult {
        self.context.current_line = self.context.program.lines.len();
        Ok(Value::None)
    }

    fn visit_help_statement(&self) -> InterpreterResult {
        Ok(Value::String(
            vec![
                "PRINT <expression>[, <expression>...]",
                "INPUT <variable>[, <variable>...]",
                "IF <condition> THEN <statement>",
                "LET <variable> = <expression>",
                "GOTO <line>",
                "GOSUB <line>",
                "REM <comment>",
                "RETURN",
                "END",
                "CLS",
                "LIST",
                "RUN",
                "NEW",
            ]
            .join("\n"),
        ))
    }

    fn visit_new_statement(&mut self) -> InterpreterResult {
        self.new_program();
        Ok(Value::None)
    }

    async fn visit_load_statement(&mut self) -> InterpreterResult {
        let source = load_file().await;
        match source {
            None => return Ok(Value::None),
            Some(source) => self.load_program(source),
        }

        Ok(Value::None)
    }

    async fn visit_save_statement(&mut self) -> InterpreterResult {
        save_file(self.context.program.print().as_str());

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
            Statement::EndStatement => return self.visit_end_statement(),
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
            Statement::ClsStatement => {
                return self.visit_cls_statement();
            }
            Statement::HelpStatement => {
                return self.visit_help_statement();
            }
            Statement::NewStatement => {
                return self.visit_new_statement();
            }
            Statement::RemStatement => {
                return Ok(Value::None);
            }
            Statement::LoadStatement => {
                return self.visit_load_statement().await;
            }
            Statement::SaveStatement => {
                return self.visit_save_statement().await;
            }
        }
    }

    async fn eval(&mut self, ast: Line) -> InterpreterResult {
        if ast.number.is_some() {
            self.context.program.set(ast);
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

            for line in ast.ok().unwrap() {
                let value = self.eval(line).await;
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

    pub fn load_program(&mut self, source: String) {
        self.new_program();

        let mut parser = Parser::new(source.as_str());
        let ast = parser.parse();

        match ast {
            Ok(ast) => {
                for line in ast {
                    self.context.program.set(line);
                }
            }
            Err(error) => {
                write_line(format!("{}", error).as_str());
            }
        }

        write_line("program loaded");
    }
}
