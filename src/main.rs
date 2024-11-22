mod ast;
mod errors;
mod interpreter;
mod io;
mod lexer;
mod parser;
mod program;

use interpreter::Interpreter;

#[tokio::main]
async fn main() {
    let mut interpreter = Interpreter::new();

    let buffer: Vec<&str> = vec![
        "10 PRINT \"HELLO, WORLD!\"",
        "20 PRINT \"HELLO, WORLD!\"",
        "30 PRINT \"HELLO, WORLD!\"",
        "40 PRINT \"HELLO, WORLD!\"",
    ];

    interpreter.load_program(buffer.join("\n"));

    interpreter.execute().await;
}
