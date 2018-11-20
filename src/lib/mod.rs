extern crate itertools;

#[macro_use]
mod utils;
mod ast;
mod parser;
mod runner;
mod scanner;
mod tokens;

use self::ast::Program;
use self::runner::environment::Environment;
use self::scanner::Scanner;

pub fn run(code: &str) {
    let mut scanner = Scanner::new(code);
    let (tokens, errors) = scanner.tokenize();

    let mut env = Environment::new();
    match Program::parse(&mut tokens.into_iter().peekable()) {
        Err(errs) => {
            for err in errors.iter().chain(errs.iter()) {
                eprintln!("{}", err);
            }
        }
        Ok(program) => match program.run(&mut env) {
            Err(err) => eprintln!("{}", err),
            _ => (),
        },
    }
}
