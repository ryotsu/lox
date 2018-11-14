#![allow(non_camel_case_types)]

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    HYPHEN,
    PLUS,
    SEMICOLON,
    SLASH,
    ASTERICS,

    // One or two character tokens
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals
    IDENTIFIER(String),
    STRING(String),
    NUMBER(f64),

    //Keywords
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub offset: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, offset: usize) -> Self {
        Token {
            token_type: token_type,
            line: line,
            offset: offset,
        }
    }
}
