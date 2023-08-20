use crate::token::{look_up_ident, Token, TokenType, Tokens};

#[derive(Debug)]
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
            self.ch = None;
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
    // TODO Support floats, and numbers in hex
    fn read_number(&mut self) -> String {
        let start_position = self.position;
        while let Some(ch) = self.ch {
            if is_digit(ch) {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[start_position..self.position].to_string()
    }
    fn skip_white_space(&mut self) {
        while let Some(ch) = self.ch {
            if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                self.read_char()
            } else {
                break;
            }
        }
    }
    fn peek_char(&self) -> Option<char> {
        if self.read_postion >= self.input.len() {
            None
        } else {
            self.input.chars().nth(self.read_postion)
        }
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_white_space();
        let tok = match self.ch {
            Some(ch) if is_digit(ch) => {
                let token_type = Tokens::INT.as_str().to_string();
                let literal = self.read_number();
                Token {
                    r#type: token_type,
                    literal,
                }
            }
            Some(ch) if is_letter(ch) => {
                let literal = self.read_identifier();
                let token_type = look_up_ident(&literal);
                Token {
                    r#type: token_type,
                    literal,
                }
            }
            Some(_) => {
                let tok = match self.ch {
                    Some('=') => {
                        if let Some('=') = self.peek_char() {
                            let current_ch = self.ch.unwrap();
                            self.read_char();
                            Token {
                                r#type: Tokens::EQ.as_str().to_string(),
                                literal: format!("{}{}", current_ch, self.ch.unwrap()),
                            }
                        } else {
                            new_token(Tokens::ASSIGN.as_str().to_string(), self.ch.unwrap())
                        }
                    }
                    Some(';') => {
                        new_token(Tokens::SEMICOLON.as_str().to_string(), self.ch.unwrap())
                    }
                    Some('(') => new_token(Tokens::LPAREN.as_str().to_string(), self.ch.unwrap()),
                    Some(')') => new_token(Tokens::RPAREN.as_str().to_string(), self.ch.unwrap()),
                    Some(',') => new_token(Tokens::COMMA.as_str().to_string(), self.ch.unwrap()),
                    Some('+') => new_token(Tokens::PLUS.as_str().to_string(), self.ch.unwrap()),
                    Some('-') => new_token(Tokens::MINUS.as_str().to_string(), self.ch.unwrap()),
                    Some('!') => {
                        if let Some('=') = self.peek_char() {
                            let current_ch = self.ch.unwrap();
                            self.read_char();
                            Token {
                                r#type: Tokens::NotEq.as_str().to_string(),
                                literal: format!("{}{}", current_ch, self.ch.unwrap()),
                            }
                        } else {
                            new_token(Tokens::BANG.as_str().to_string(), self.ch.unwrap())
                        }
                    }
                    Some('*') => new_token(Tokens::ASTERISK.as_str().to_string(), self.ch.unwrap()),
                    Some('/') => new_token(Tokens::SLASH.as_str().to_string(), self.ch.unwrap()),
                    Some('<') => new_token(Tokens::LT.as_str().to_string(), self.ch.unwrap()),
                    Some('>') => new_token(Tokens::GT.as_str().to_string(), self.ch.unwrap()),
                    Some('{') => new_token(Tokens::LBRACE.as_str().to_string(), self.ch.unwrap()),
                    Some('}') => new_token(Tokens::RBRACE.as_str().to_string(), self.ch.unwrap()),
                    None => Token {
                        r#type: Tokens::EOF.as_str().to_string(),
                        literal: "".to_string(),
                    },
                    _ => Token {
                        r#type: Tokens::ILLEGAL.as_str().to_string(),
                        literal: self.ch.map(|ch| ch.to_string()).unwrap_or_default(),
                    },
                };
                // Read the next character for single-character tokens
                self.read_char();
                tok
            }
            None => Token {
                r#type: Tokens::EOF.as_str().to_string(),
                literal: "".to_string(),
            },
        };
        tok
    }
}

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
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
