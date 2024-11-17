mod ast;
mod lexer;
mod parser;
mod interpreter;

use lexer::Lexer;
use lexer::TokenKind;
use parser::Parser;
use interpreter::Interpreter;

use std::io::Write;
use std::io::{stdin, stdout};

fn main() {
    let source = "PRINT \"Hello, World!\"";
    // let mut parser = Parser::new(source);
    // let mut lex = Lexer::new(source);
    // while let token = lex.next() {
    //     if token.kind == TokenKind::Eol {
    //         break;
    //     }

    //     println!("{:?}", token);
    // }

    // let parsed = parser.parse();
    // println!("{:#?}", parsed);
    let mut interpreter = Interpreter::new();
    interpreter.execute(source);

    loop {
        let mut buffer = String::new();
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut buffer).unwrap();
        interpreter.execute(&buffer);
    }
}
