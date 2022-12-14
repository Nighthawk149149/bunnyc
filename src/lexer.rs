/*
bunnyc - A compiler for the bunny langauge
Copyright (C) 2022  Nicholas Stienz

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::token::{Token, TokenKind};
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub struct Lexer {
    program: BufReader<File>,
    line: String,
    tockens: Vec<Token>,
    ln: u32,
    col: u32,
}

/*
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("log.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    let len = reader.read_line(&mut line)?;
    println!("First line is {len} bytes long");
    Ok(())
}
*/

impl Lexer {
    pub fn new(program: &str) -> Lexer {
        Lexer {
            program: BufReader::new(File::open(program).unwrap()), // TODO: Error handling
            line: String::new(),
            tockens: Vec::new(),
            ln: 1,
            col: 1,
        }
    }

    // TODO: Pain in the ass function
    pub fn next(&mut self) -> Token {
        Token::new(TokenKind::EOF, "".to_string())
    }
}

//? Old code, will be removed soon
// use crate::token::Token;
// use crate::token::TokenKind;

// pub struct Lexer<'a> {
//     program: &'a str,
//     pos: usize,
//     line: usize,
//     column: usize,
//     tokens: Vec<Token>,
// }

// impl<'a> Lexer<'a> {
//     pub fn new(program: &'a str) -> Lexer<'a> {
//         Lexer {
//             program,
//             pos: 0,
//             line: 1,
//             column: 1,
//             tokens: Vec::new(),
//         }
//     }

//     pub fn lex(&mut self) -> Vec<Token> {
//         while self.pos < self.program.len() {
//             let c = self.program.chars().nth(self.pos).unwrap();

//             if c == '\n' {
//                 self.pos += 1;
//                 self.line += 1;
//                 self.column = 1;
//                 continue;
//             } else if c.is_whitespace() {
//                 self.pos += 1;
//                 self.column += 1;
//                 continue;
//             } else if c.is_alphabetic() {
//                 self.lex_identifier();
//                 continue;
//             } else if c.is_numeric() {
//                 self.lex_number();
//                 continue;
//             } else if c == '"' {
//                 self.lex_string();
//                 continue;
//             }

//             self.pos += 1;
//             self.column += 1;
//         }

//         self.tokens.clone()
//     }

//     fn lex_identifier(&mut self) {
//         let mut identifier = String::new();

//         while self.pos < self.program.len() {
//             let c = self.program.chars().nth(self.pos).unwrap();

//             if c.is_alphanumeric() {
//                 identifier.push(c);
//                 self.pos += 1;
//                 self.column += 1;
//             } else {
//                 break;
//             }
//         }

//         self.tokens
//             .push(Token::new_identifier(identifier, self.line, self.column));
//     }

//     fn lex_number(&mut self) {
//         let mut number = String::new();
//         let mut is_float = false;

//         while self.pos < self.program.len() {
//             let c = self.program.chars().nth(self.pos).unwrap();

//             if c.is_numeric() {
//                 number.push(c);
//                 self.pos += 1;
//                 self.column += 1;
//             } else if c == '.' {
//                 is_float = true;
//                 number.push('.');
//                 self.pos += 1;
//                 self.column += 1;
//             } else {
//                 break;
//             }
//         }

//         if is_float {
//             self.tokens.push(Token::new(TokenKind::Float, number));
//         } else {
//             self.tokens.push(Token::new(TokenKind::Number, number));
//         }
//     }

//     fn lex_string(&mut self) {
//         let mut string = String::new();

//         self.pos += 1;
//         self.column += 1;

//         while self.pos < self.program.len() {
//             let c = self.program.chars().nth(self.pos).unwrap();

//             if c != '"' {
//                 string.push(c);
//                 self.pos += 1;
//                 self.column += 1;
//             } else {
//                 break;
//             }
//         }

//         self.pos += 1;
//         self.column += 1;

//         self.tokens
//             .push(Token::new(TokenKind::StringLiteral, string));
//     }
// }
