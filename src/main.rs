use std::{fs::read_to_string, process::exit};

mod scanner;

use scanner::Scanner;

use crate::scanner::LexError;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line}] Error{loc}: {message}");
}

fn run_file(path: &str) {
    run(&read_to_string(path).unwrap());
}

fn run(source: &str) {
    let mut scanner = Scanner::new(String::from(source));
    let (tokens, errors) = scanner.scan_tokens();
    if errors.len() == 0 {
        for token in tokens {
            println!("{token:?}");
        }
    } else {
        for LexError { line, loc, message } in errors {
            report(line, &loc, &message);
        }
    }
}

fn run_prompt() {
    let mut buf = String::new();
    loop {
        print!("> ");
        let Ok(_) = std::io::stdin().read_line(&mut buf) else {
            break;
        };
        run(&buf.trim());
        buf.clear();
    }
}

// fn error(&mut self, line: usize, message: &str) {
//     self.report(line, "", message);
// }
