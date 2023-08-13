#![allow(dead_code)]

pub type TokenType = String;

pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
// identifiers and literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
// operators
pub const ASSIGN: &str = "ASSIGN";
pub const PLUS: &str = "PLUS";

// delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
// keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
