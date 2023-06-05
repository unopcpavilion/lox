use crate::scanner::Scanner;
use std::io::{BufRead, Read};
use std::process::exit;
use std::{fs, io};

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Lox {
        Lox { had_error: false }
    }

    pub unsafe fn run_file(&self, path: &String) {
        let mut file = fs::File::open(path).expect("Error opening file");
        let mut source: String = String::new();

        file.read_to_string(&mut source)
            .expect("Error reading file");

        self.run((&source).parse().unwrap());
        if self.had_error {
            exit(65);
        }
    }

    pub unsafe fn run_prompt(&mut self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            self.run((&line.unwrap()).parse().unwrap());
            self.had_error = false;
        }
    }

    fn run(&self, source: String) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token.to_string());
        }
    }

    pub fn error(line: usize, message: &str) {
        Lox::report(line, "", message);
    }

    fn report( line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
    }
}
