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
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, Error};

pub fn run_file(path: &str) -> Result<(), Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut scanner = Scanner::new(&contents);
    let (tokens, errors) = scanner.tokenize();

    let mut env = Environment::new();
    match Program::parse(&mut tokens.into_iter().peekable()) {
        Err(errs) => {
            for err in errors.iter().chain(errs.iter()) {
                eprintln!("{}", err);
            }
            Ok(())
        }
        Ok(program) => match program.run(&mut env) {
            Err(err) => Ok(eprintln!("{}", err)),
            _ => Ok(()),
        },
    }
}

pub fn run_repl() {
    let mut env = Environment::new();
    loop {
        let mut source = String::new();
        stdin().read_line(&mut source).unwrap();
        let mut scanner = Scanner::new(&source);
        let (tokens, errors) = scanner.tokenize();
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
}
