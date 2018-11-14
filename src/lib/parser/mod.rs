mod expression;
mod statement;
mod utils;

use super::scanner::Scanner;
use crate::ast::Statement;
use std::iter::Peekable;

pub fn parse_it(mut tokens: Peekable<Scanner>) {
    let mut statements = vec![];

    while tokens.peek() != None {
        let statement = Statement::parse(&mut tokens);
        statements.push(statement);
    }

    for s in statements {
        println!("{:?}", s);
    }
}
