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
