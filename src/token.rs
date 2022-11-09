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
use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Token {
        Token { kind, literal }
    }

    pub fn new_identifier(identifier: String, line: usize, coloum: usize) -> Token {
        match identifier.as_str() {
            "ATTACKMODE" => Token::new(TokenKind::BunnyAttackmode, identifier),
            "HID" => Token::new(TokenKind::BunnyAttackType, identifier),
            "STRING" => Token::new(TokenKind::BunnyString, identifier),
            "DELAY" => Token::new(TokenKind::BunnyDelay, identifier),
            //_ => panic!("Unknown identifier: {}", identifier),
            _ => {
                Error::write(
                    format!("Unknown identifier: {}", identifier).as_str(),
                    line,
                    coloum - identifier.len(),
                );
                Token::new(TokenKind::Unknown, identifier)
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    BunnyAttackmode,
    BunnyString,
    BunnyAttackType,
    BunnyDelay,

    StringLiteral,
    Number,
    Float,
    Unknown,
}
