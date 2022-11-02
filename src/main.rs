mod error;
mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    Lexer::new(&std::fs::read_to_string("input.txt").unwrap())
        .lex()
        .iter()
        .for_each(|t| println!("{:?}", t));
}
