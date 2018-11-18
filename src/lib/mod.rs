extern crate itertools;

#[macro_use]
mod utils;
mod ast;
mod parser;
mod scanner;
mod tokens;

use self::scanner::Scanner;
use std::fs::File;
use std::io::prelude::*;
use std::io::{stdin, Error};

pub fn run_file(path: &str) -> Result<(), Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for token in Scanner::new(&contents) {
        println!("{:?}", token);
    }

    Ok(())
}

pub fn run_repl() {
    loop {
        let mut source = String::new();
        stdin().read_line(&mut source).unwrap();
        let (tokens, _errors) = Scanner::new(&source).tokenize();
        parser::parse_it(tokens.into_iter().peekable());
    }
}
