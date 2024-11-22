use std::str::Chars;

use crate::errors::SyntaxError;

type LexerResult<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    StringLiteral,
    NumberLiteral,
    Identifier,
    LeftParen,
    RightParen,
    Comma,
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Eol,
    Eof
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    None,
    Digit(usize),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub value: TokenValue,
}

impl Token {
    pub fn default() -> Token {
        Token {
            kind: TokenKind::Eof,
            span: Span { start: 0, end: 0 },
            value: TokenValue::None,
        }
    }
}

// Why 'a? https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax
pub struct Lexer<'a> {
    source: &'a str,
    chars: Chars<'a>,
    next_token: LexerResult<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut instance = Self {
            source,
            chars: source.chars(),
            next_token: Ok(Token::default()),
        };

        instance.next_token = instance.consume_token();

        return instance;
    }

    fn consume_number_literal(&mut self) -> LexerResult<Token> {
        let start = self.offset();
        while let Some(c) = self.peek_char() {
            match c {
                '0'..='9' => {
                    self.next_char();
                }
                _ => break,
            }
        }

        let value = self.source[start..self.offset()].to_string();

        Ok(Token {
            kind: TokenKind::NumberLiteral,
            span: Span {
                start,
                end: self.offset(),
            },
            value: TokenValue::Digit(value.parse().unwrap()),
        })
    }

    fn consume_string_literal(&mut self) -> LexerResult<Token> {
        self.next_char();

        let start = self.offset();
        while let Some(c) = self.peek_char() {
            match c {
                '"' => {
                    let end = self.offset();
                    self.next_char();

                    return Ok(Token {
                        kind: TokenKind::StringLiteral,
                        span: Span { start, end },
                        value: self.get_value(start, end),
                    });
                }
                _ => {
                    self.next_char();
                }
            }
        }

        Err(SyntaxError::UnterminatedStringLiteral(self.offset()))
    }

    fn consume_identifier(&mut self) -> LexerResult<Token> {
        let start = self.offset();
        while let Some(c) = self.peek_char() {
            match c {
                'A'..='Z' => {
                    self.next_char();
                }
                _ => break,
            }
        }
        let end = self.offset();

        Ok(Token {
            kind: TokenKind::Identifier,
            span: Span { start, end },
            value: self.get_value(start, end),
        })
    }

    fn consume_new_line(&mut self) -> LexerResult<Token> {
        self.next_char();

        Ok(Token {
            kind: TokenKind::Eol,
            span: Span {
                start: self.offset(),
                end: self.offset(),
            },
            value: TokenValue::None,
        })
    }

    fn consume_token(&mut self) -> LexerResult<Token> {
        while let Some(c) = self.peek_char() {
            match c {
                '\n' => return self.consume_new_line(),
                '"' => return self.consume_string_literal(),
                '+' | '-' | '*' | '/' | '(' | ')' | ',' | '=' | '<' | '>' => {
                    self.next_char();

                    let kind = match c {
                        '+' => TokenKind::Add,
                        '-' => TokenKind::Subtract,
                        '*' => TokenKind::Multiply,
                        '/' => TokenKind::Divide,
                        '(' => TokenKind::LeftParen,
                        ')' => TokenKind::RightParen,
                        ',' => TokenKind::Comma,
                        '=' => TokenKind::Equal,
                        '>' => match self.peek_char() {
                            Some('=') => {
                                self.next_char();
                                TokenKind::GreaterThanOrEqual
                            }
                            _ => TokenKind::GreaterThan,
                        },
                        '<' => match self.peek_char() {
                            Some('=') => {
                                self.next_char();
                                TokenKind::LessThanOrEqual
                            }
                            Some('>') => {
                                self.next_char();
                                TokenKind::NotEqual
                            }
                            _ => TokenKind::LessThan,
                        },
                        _ => unreachable!(),
                    };

                    let start = self.offset();
                    let end = self.offset();

                    return Ok(Token {
                        kind,
                        span: Span { start, end },
                        value: self.get_value(start, end),
                    });
                }
                '0'..='9' => return self.consume_number_literal(),
                'A'..='Z' => return self.consume_identifier(),
                ' ' => {
                    self.next_char();
                }
                _ => {
                    return Err(SyntaxError::UnexpectedCharacter(c, self.offset()));
                }
            };
        }

        Ok(Token {
            kind: TokenKind::Eof,
            span: Span {
                start: self.offset(),
                end: self.offset(),
            },
            value: TokenValue::None,
        })
    }

    pub fn next(&mut self) -> LexerResult<Token> {
        let next_token = self.consume_token();
        let token = std::mem::replace(&mut self.next_token, next_token);

        token
    }

    pub fn peek(&self) -> LexerResult<Token> {
        match &self.next_token {
            Ok(token) => Ok(token.clone()),
            Err(err) => Err(err.clone()),
        }
    }

    fn get_value(&self, start: usize, end: usize) -> TokenValue {
        match &self.source[start..end] {
            "" => TokenValue::None,
            "\n" => TokenValue::None,
            value => {
                if let Ok(value) = value.parse() {
                    TokenValue::Digit(value)
                } else {
                    TokenValue::String(value.to_string())
                }
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn offset(&self) -> usize {
        self.source.len() - self.chars.as_str().len()
    }
}
