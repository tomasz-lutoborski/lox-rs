pub mod token;
pub mod scanner;

use std::fs;
use std::io::{self, BufRead, Write};
use scanner::Scanner;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut lox = Lox::new();

    match args.len() {
        1 => lox.run_prompt(),
        2 => lox.run_file(&args[1]),
        _ => {
            println!("Usage: lox [script]");
            std::process::exit(64);
        }
    }
}

struct Lox {
    had_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox { had_error: false }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    fn run_file(&mut self, path: &str) -> io::Result<()> {
        let content = fs::read_to_string(path)?;
        self.run(&content);
        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        let reader = stdin.lock();

        for line in reader.lines() {
            print!("> ");
            stdout.flush()?;
            let line = line?;
            if line.is_empty() {
                break;
            }
            self.run(&line);
            self.had_error = false;
        }

	Ok(())
    }
}
