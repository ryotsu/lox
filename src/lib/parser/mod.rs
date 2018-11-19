mod expression;
mod statement;
mod utils;

use super::tokens::{Token, TokenType};
use crate::ast::{Program, Statement};
use std::iter::Peekable;

impl Program {
    pub fn parse<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) -> Result<Self, Vec<String>> {
        let mut statements = vec![];
        let mut errors = vec![];

        while tokens.peek() != None {
            match Statement::parse(tokens) {
                Ok(statement) => statements.push(statement),
                Err(error) => {
                    errors.push(error);
                    synchronize(tokens);
                }
            }
        }

        if errors.len() != 0 {
            Err(errors)
        } else {
            Ok(Program {
                statements: statements,
            })
        }
    }
}

fn synchronize<T: Iterator<Item = Token>>(tokens: &mut Peekable<T>) {
    use self::TokenType::*;

    while Some(false) == check_next_token!(tokens, CLASS, FUN, VAR, FOR, IF, WHILE, PRINT, RETURN) {
        if match_next_token!(tokens, SEMICOLON) {
            return;
        }

        tokens.next();
    }
}
