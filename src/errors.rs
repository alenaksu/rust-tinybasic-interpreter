use std::fmt;

use crate::ast::ArithmeticOperator;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedCharacter(char, usize),
    UnterminatedStringLiteral(usize),
    UnexpectedToken(Token),
    InvalidVariableName(String, usize),
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexpected character '{}' at position {}", c, pos)
            }
            Self::UnterminatedStringLiteral(pos) => {
                write!(f, "Unterminated string literal at position {}", pos)
            }
            Self::UnexpectedToken(token) => {
                write!(
                    f,
                    "Unexpected token {:?} at position {}",
                    token.kind, token.span.start
                )
            }
            Self::InvalidVariableName(name, pos) => {
                write!(f, "Invalid variable name {} at position {}", name, pos)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeError {
    UndefinedVariable(String),
    InvalidOperation(usize),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UndefinedVariable(name) => write!(f, "Undefined variable '{}'", name),
            Self::InvalidOperation(pos) => write!(f, "Invalid operation at line {}", pos),
        }
    }
}
