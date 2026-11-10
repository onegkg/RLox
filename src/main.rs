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
    interpreter::{Interpretable, environment::Environment, values::RuntimeError},
    parser::{ParseError, Parser},
    scanner::LexError,
};

fn main() {
    let mut env = Environment::new();
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 2 {
        println!("Usage: lox [script]");
        exit(64);
    } else if args.len() == 2 {
        exit(run_file(&args[1], &mut env));
    } else {
        run_prompt(&mut env);
    }
}

fn report(line: usize, loc: &str, message: &str) {
    if line == 0 {
        eprintln!("Error{loc}: {message}");
    }
    eprintln!("[line {line}] Error{loc}: {message}");
}

fn run_file(path: &str, env: &mut Environment) -> i32 {
    run(&read_to_string(path).unwrap(), env)
}

fn report_parse_error(e: ParseError) {
    let Some(t) = e.token else {
        report(0, " at end of file", &e.msg);
        return;
    };
    report(t.line, &format!(" at '{}'", t.lexeme), &e.msg)
}

fn run(source: &str, env: &mut Environment) -> i32 {
    let mut scanner = Scanner::new(String::from(source));
    match scanner.scan_tokens() {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(stmts) => match stmts.interpret() {
                    Ok(_) => 0,
                    Err(e) => {
                        report_runtime_error(e);
                        70
                    }
                },
                Err(es) => {
                    for e in es.0 {
                        report_parse_error(e)
                    }
                    65
                }
            }
        }
        Err(es) => {
            for e in es.0 {
                report_lex_error(e);
            }
            65
        }
    }
}

fn report_lex_error(e: LexError) {
    report(e.line, &e.loc, &e.message);
}

fn report_runtime_error(e: RuntimeError) {
    eprintln!("{e}");
}

fn run_prompt(env: &mut Environment) {
    let mut buf = String::new();
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let Ok(_) = std::io::stdin().read_line(&mut buf) else {
            break;
        };
        run(&buf.trim(), env);
        buf.clear();
    }
}
