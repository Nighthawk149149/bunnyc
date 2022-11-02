use crate::token::Token;
use crate::token::TokenKind;

pub struct Lexer<'a> {
    program: &'a str,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(program: &'a str) -> Lexer<'a> {
        Lexer {
            program,
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        while self.pos < self.program.len() {
            let c = self.program.chars().nth(self.pos).unwrap();

            if c == '\n' {
                self.pos += 1;
                self.line += 1;
                self.column = 1;
                continue;
            } else if c.is_whitespace() {
                self.pos += 1;
                self.column += 1;
                continue;
            } else if c.is_alphabetic() {
                self.lex_identifier();
                continue;
            } else if c.is_numeric() {
                self.lex_number();
                continue;
            } else if c == '"' {
                self.lex_string();
                continue;
            }

            self.pos += 1;
            self.column += 1;
        }

        self.tokens.clone()
    }

    fn lex_identifier(&mut self) {
        let mut identifier = String::new();

        while self.pos < self.program.len() {
            let c = self.program.chars().nth(self.pos).unwrap();

            if c.is_alphanumeric() {
                identifier.push(c);
                self.pos += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        self.tokens
            .push(Token::new_identifier(identifier, self.line, self.column));
    }

    fn lex_number(&mut self) {
        let mut number = String::new();
        let mut is_float = false;

        while self.pos < self.program.len() {
            let c = self.program.chars().nth(self.pos).unwrap();

            if c.is_numeric() {
                number.push(c);
                self.pos += 1;
                self.column += 1;
            } else if c == '.' {
                is_float = true;
                number.push('.');
                self.pos += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        if is_float {
            self.tokens.push(Token::new(TokenKind::Float, number));
        } else {
            self.tokens.push(Token::new(TokenKind::Number, number));
        }
    }

    fn lex_string(&mut self) {
        let mut string = String::new();

        self.pos += 1;
        self.column += 1;

        while self.pos < self.program.len() {
            let c = self.program.chars().nth(self.pos).unwrap();

            if c != '"' {
                string.push(c);
                self.pos += 1;
                self.column += 1;
            } else {
                break;
            }
        }

        self.pos += 1;
        self.column += 1;

        self.tokens
            .push(Token::new(TokenKind::StringLiteral, string));
    }
}
