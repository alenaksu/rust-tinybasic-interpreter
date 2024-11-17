use crate::ast::{Expression, Line, Literal, Statement};
use crate::parser::Parser;

enum Value {
    Number(usize),
    String(String),
    None,
}

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    fn visit_expression(&mut self, expression: Expression) -> Value {
        match expression {
            Expression::Literal(literal) => match literal {
                Literal::Number { value } => Value::Number(value),
                Literal::String { value } => Value::String(value),
            },
            _ => Value::None,
        }
    }

    fn visit_print_statement(&mut self, expressions: Vec<Expression>) {
        for expression in expressions {
            let value = self.visit_expression(expression);
            match value {
                Value::Number(value) => println!("{}", value),
                Value::String(value) => println!("{}", value),
                _ => {}
            }
        }
    }

    fn eval(&mut self, ast: Line) {
        match ast.statement {
            Statement::PrintStatement { expressions } => self.visit_print_statement(expressions),
            _ => {}
        }
    }

    pub fn execute(&mut self, source: &str) {
        let mut parser = Parser::new(source);
        let ast = parser.parse();

        self.eval(ast);
    }
}
