mod token_type;
mod token;
mod scanner;
mod lox;
use std::process::exit;
use std::{env};
use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut lox = Lox::new();

    if args.len() > 1 {
        println!("Usage: lox [script]");
        exit(64);
    } else if args.len() == 1 {
        unsafe { lox.run_file(&args[0]); }
    } else {
        unsafe { lox.run_prompt(); }
    }
}


