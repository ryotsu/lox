extern crate lox;

use lox::run;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
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
            println!("Usage: {} [script]", exec);
            process::exit(64);
        }
    }
}

fn run_file(path: &str) -> Result<(), Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    run(&contents);
    Ok(())
}
