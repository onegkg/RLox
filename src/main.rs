use std::{
    fs::read_to_string,
    io::{self, Write},
    process::exit,
};

mod ast;
mod interpreter;
mod parser;
mod scanner;

use scanner::Scanner;

use crate::{
    parser::{ParseError, Parser},
    scanner::LexError,
};

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
    if line == 0 {
        eprintln!("Error{loc}: {message}");
    }
    eprintln!("[line {line}] Error{loc}: {message}");
}

fn run_file(path: &str) {
    run(&read_to_string(path).unwrap());
}

fn report_parse_error(e: ParseError) {
    let Some(t) = e.token else {
        report(0, " at end of file", &e.msg);
        return;
    };
    report(t.line, &format!(" at '{}'", t.lexeme), &e.msg)
}

fn run(source: &str) {
    let mut scanner = Scanner::new(String::from(source));
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(expr) => println!("{expr}"),
                Err(es) => {
                    for err in es.0 {
                        report_parse_error(err)
                    }
                }
            }
        }
        Err(es) => {
            for LexError { line, loc, message } in es.0 {
                report(line, &loc, &message);
            }
        }
    }
}

fn run_prompt() {
    let mut buf = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
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
