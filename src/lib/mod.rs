extern crate itertools;
extern crate wasm_bindgen;

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

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "../www/index.js")]
extern "C" {
    pub fn print_js(s: &str);
}

#[wasm_bindgen]
pub fn run(code: &str) {
    let mut scanner = Scanner::new(code);
    let (tokens, errors) = scanner.tokenize();

    let mut env = Environment::new();
    match Program::parse(&mut tokens.into_iter().peekable()) {
        Err(errs) => {
            for err in errors.iter().chain(errs.iter()) {
                print_js(&format!("Error: {}", err));
            }
        }
        Ok(program) => match program.run(&mut env) {
            Err(err) => print_js(&format!("Error: {}", err)),
            _ => (),
        },
    }
}
