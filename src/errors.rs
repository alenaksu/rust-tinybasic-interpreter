use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedCharacter(char, usize),
    UnterminatedStringLiteral(usize),
    UnexpectedToken(Token),
    UnexpectedIdentifier(String, usize),
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
                write!(f, "Invalid variable name '{}' at position {}", name, pos)
            }
            Self::UnexpectedIdentifier(name, pos) => {
                write!(f, "Unexpected identifier '{}' at position {}", name, pos)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeError {
    Generic(String),
    NotImplemented(String),
    InvalidOperation(usize),
    SyntaxError(SyntaxError),
}

impl RuntimeError {
    pub fn new(message: &str) -> Self {
        Self::Generic(message.to_string())
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(message) => write!(f, "{}", message),
            Self::NotImplemented(name) => write!(f, "Not implemented: {}", name),
            Self::InvalidOperation(pos) => write!(f, "Invalid operation at line {}", pos),
            Self::SyntaxError(name) => write!(f, "Syntax error: {}", name),
        }
    }
}
