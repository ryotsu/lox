use crate::ast::BinaryOp;
use crate::tokens::{Token, TokenType};
use std::fmt::Display;
use std::iter::Peekable;

pub fn get_identifier<T>(tokens: &mut Peekable<T>) -> Result<String, String>
where
    T: Iterator<Item = Token>,
{
    match tokens.next() {
        Some(token) => match token.token_type {
            TokenType::IDENTIFIER(identifier) => Ok(identifier),
            _ => Err(format!(
                "L{}:{} Expected IDENTIFIER, found {:?}",
                token.line, token.offset, token.token_type
            )),
        },
        None => Err(format!("Expected IDENTIFIER, found EOF")),
    }
}

pub fn consume<T, U>(
    tokens: &mut Peekable<T>,
    token: TokenType,
    message: U,
) -> Result<TokenType, String>
where
    T: Iterator<Item = Token>,
    U: Display,
{
    match tokens.peek() {
        Some(t) if t.token_type == token => {
            tokens.next();
            Ok(token)
        }
        Some(t) => match t.token_type {
            TokenType::IDENTIFIER(_) => {
                if let TokenType::IDENTIFIER(_) = token {
                    Ok(tokens.next().unwrap().token_type)
                } else {
                    Err(format!("L{}:{} {}", t.line, t.offset, message))
                }
            }
            _ => Err(format!("L{}:{} {}", t.line, t.offset, message)),
        },
        None => Err(format!("{} found EOF", message)),
    }
}

pub fn map_binary_op(token: TokenType) -> BinaryOp {
    use self::BinaryOp::*;
    use self::TokenType::*;

    match token {
        EQUAL_EQUAL => EqualEqual,
        BANG_EQUAL => NotEqual,
        LESS => Less,
        LESS_EQUAL => LessEqual,
        GREATER => Greater,
        GREATER_EQUAL => GreaterEqual,
        PLUS => Plus,
        HYPHEN => Minus,
        ASTERICS => Multiply,
        _ => Divide,
    }
}
