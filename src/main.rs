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

mod lexer;
mod token;

use lexer::Lexer;
use token::{Token, TokenKind};

fn main() {
    let mut lexer = Lexer::new("input.txt");

    loop {
        let token = lexer.next();

        if Token::equals_kind(token.kind(), &TokenKind::EOF) {
            break;
        }
    }
}

//? Old code, will be removed soon
// mod error;
// mod lexer;
// mod token;

// use lexer::Lexer;

// fn main() {
//     Lexer::new(&std::fs::read_to_string("input.txt").unwrap())
//         .lex()
//         .iter()
//         .for_each(|t| println!("{:?}", t));
// }
