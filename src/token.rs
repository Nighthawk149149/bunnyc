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

pub struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Token {
        Token { kind, literal }
    }

    //                                              I
    // You're not allowed to mutate my token kind nnInn
    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    // I know very little of what this does exept that it works... Magic code // TODO: WTF is really going on here?
    pub fn equals_kind(lhs: &TokenKind, rhs: &TokenKind) -> bool {
        // This could and probably will cause a bug in the future! (Implement more than 255 tokens)
        *lhs as u8 == *rhs as u8 // Dereference and campare the values as unsigned 8-bit integers.
    }
}

// Doesn't need Copy or Clone except to fix possible bugs with the equals_kind function as of 12/4/2022.
#[derive(Copy, Clone)]
pub enum TokenKind {
    EOF,
}

//? Old code, will be removed soon
// use crate::error::Error;

// #[derive(Clone, Debug)]
// pub struct Token {
//     pub kind: TokenKind,
//     pub literal: String,
// }

// impl Token {
//     pub fn new(kind: TokenKind, literal: String) -> Token {
//         Token { kind, literal }
//     }

//     pub fn new_identifier(identifier: String, line: usize, coloum: usize) -> Token {
//         match identifier.as_str() {
//             "ATTACKMODE" => Token::new(TokenKind::BunnyAttackmode, identifier),
//             "HID" => Token::new(TokenKind::BunnyAttackType, identifier),
//             "STRING" => Token::new(TokenKind::BunnyString, identifier),
//             "DELAY" => Token::new(TokenKind::BunnyDelay, identifier),
//             //_ => panic!("Unknown identifier: {}", identifier),
//             _ => {
//                 Error::write(
//                     format!("Unknown identifier: {}", identifier).as_str(),
//                     line,
//                     coloum - identifier.len(),
//                 );
//                 Token::new(TokenKind::Unknown, identifier)
//             }
//         }
//     }
// }

// #[derive(Clone, Debug)]
// pub enum TokenKind {
//     BunnyAttackmode,
//     BunnyString,
//     BunnyAttackType,
//     BunnyDelay,

//     StringLiteral,
//     Number,
//     Float,
//     Unknown,
// }
