mod ast;
mod errors;
mod interpreter;
mod io;
mod lexer;
mod parser;

use interpreter::Interpreter;

#[tokio::main]
async fn main() {
    let mut interpreter = Interpreter::new();

    interpreter.execute().await;
}
