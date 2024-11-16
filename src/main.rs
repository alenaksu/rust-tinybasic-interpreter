mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use lexer::TokenKind;
use parser::Parser;

fn main() {
    let source = "10 IF X > Y THEN GOTO X+10";
    let mut parser = Parser::new(source);
    let mut lex = Lexer::new(source);
    while let token = lex.next() {
        if token.kind == TokenKind::Eol {
            break;
        }

        println!("{:?}", token);
    }

    let parsed = parser.parse();
    println!("{:#?}", parsed);
}
