mod error;
mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    println!("=== Program 1 ===");
    let program1 = "ATTACKMODE HID\nDELAY 500\nSTRING \"Hello World\"";
    println!("{}", program1);

    println!("=== Tokens 1 ===");
    let tokens1 = Lexer::new(program1).lex();
    tokens1.iter().for_each(|token| {
        println!("{:?}", token);
    });
}
