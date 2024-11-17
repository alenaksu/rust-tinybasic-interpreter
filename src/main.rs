mod ast;
mod errors;
mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;

use std::io::Write;
use std::io::{stdin, stdout};

fn main() {
    let mut interpreter = Interpreter::new();

    println!("Welcome to TINY BASIC!");
    loop {
        let mut buffer = String::new();
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut buffer).unwrap();
        interpreter.execute(&buffer);
    }
}
