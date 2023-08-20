use std::collections::HashMap;

use lazy_static::lazy_static;

pub type TokenType = String;
#[derive(Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

pub enum Tokens {
    ILLEGAL,
    EOF,
    // identifiers and literals
    IDENT,
    INT,
    // operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    // delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl Tokens {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tokens::ILLEGAL => "ILLEGAL",
            Tokens::EOF => "EOF",
            Tokens::IDENT => "IDENT",
            Tokens::INT => "INT",
            Tokens::ASSIGN => "ASSIGN",
            Tokens::PLUS => "+",
            Tokens::MINUS => "-",
            Tokens::BANG => "!",
            Tokens::ASTERISK => "*",
            Tokens::SLASH => "/",
            Tokens::GT => ">",
            Tokens::LT => "<",
            Tokens::COMMA => ",",
            Tokens::SEMICOLON => ";",
            Tokens::LPAREN => "(",
            Tokens::RPAREN => ")",
            Tokens::LBRACE => "{",
            Tokens::RBRACE => "}",
            Tokens::FUNCTION => "FUNCTION",
            Tokens::LET => "LET",
            Tokens::TRUE => "TRUE",
            Tokens::FALSE => "FALSE",
            Tokens::IF => "IF",
            Tokens::ELSE => "ELSE",
            Tokens::RETURN => "RETURN",
        }
    }
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("fn", Tokens::FUNCTION.as_str()),
            ("let", Tokens::LET.as_str()),
            ("true", Tokens::TRUE.as_str()),
            ("false", Tokens::FALSE.as_str()),
            ("if", Tokens::IF.as_str()),
            ("else", Tokens::ELSE.as_str()),
            ("return", Tokens::RETURN.as_str()),
        ])
    };
}
pub fn look_up_ident(ident: &str) -> TokenType {
    if let Some(i) = KEYWORDS.get(ident) {
        return i.to_string();
    }
    Tokens::IDENT.as_str().to_string()
}
