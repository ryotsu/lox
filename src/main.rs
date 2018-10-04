extern crate lox;

use lox::*;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    let exec = args.next().unwrap();
    match args.next() {
        Some(path) => match args.next() {
            None => {
                println!("Running file: {}", path);
                run_file(&path).unwrap();
            }
            Some(_) => {
                println!("Usage: {} [script]", exec);
                process::exit(64);
            }
        },
        None => {
            println!("Running prompt");
            run_repl();
        }
    }
}
