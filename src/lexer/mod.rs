#![allow(dead_code)]

use crate::token::{Token, TokenType, Tokens};

pub struct Lexer {
    input: String,
    position: usize,
    read_postion: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_postion: 0,
            ch: None,
        };
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        if self.read_postion >= self.input.len() {
            self.ch = None
        } else {
            self.ch = self.input.chars().nth(self.read_postion);
        }
        self.position = self.read_postion;
        self.read_postion += 1;
    }

    fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while let Some(ch) = self.ch {
            if is_letter(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_position..self.position].to_string()
    }


    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            Some('=') => new_token(Tokens::ASSIGN.as_str().to_string(), self.ch.unwrap()),
            Some(';') => new_token(Tokens::SEMICOLON.as_str().to_string(), self.ch.unwrap()),
            Some('(') => new_token(Tokens::LPAREN.as_str().to_string(), self.ch.unwrap()),
            Some(')') => new_token(Tokens::RPAREN.as_str().to_string(), self.ch.unwrap()),
            Some(',') => new_token(Tokens::COMMA.as_str().to_string(), self.ch.unwrap()),
            Some('+') => new_token(Tokens::PLUS.as_str().to_string(), self.ch.unwrap()),
            Some('{') => new_token(Tokens::LBRACE.as_str().to_string(), self.ch.unwrap()),
            Some('}') => new_token(Tokens::RBRACE.as_str().to_string(), self.ch.unwrap()),
            None => Token {
                r#type: Tokens::EOF.as_str().to_string(),
                literal: "".to_string(),
            },
            Some(ch) if is_letter(ch) => {
                let literal = self.read_identifier();
                Token {
                    r#type: Tokens::IDENT.as_str().to_string(), // Assuming IDENT represents identifiers
                    literal,
                }
            }
            _ => Token {
                r#type: Tokens::ILLEGAL.as_str().to_string(),
                literal: self.ch.map(|ch| ch.to_string()).unwrap_or_default(),
            },
        };
        self.read_char();
        tok
    }
}

fn new_token(token_type: TokenType, ch: char) -> Token {
    Token {
        r#type: token_type,
        literal: ch.to_string(),
    }
}

fn is_letter(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}
