mod expression;
mod statement;
mod utils;

use super::tokens::Token;
use crate::ast::Statement;
use std::iter::Peekable;

pub fn parse_it<T: Iterator<Item = Token>>(mut tokens: Peekable<T>) {
    let mut statements = vec![];

    while tokens.peek() != None {
        let statement = Statement::parse(&mut tokens);
        statements.push(statement);
    }

    for s in statements {
        println!("{:?}", s);
    }
}
