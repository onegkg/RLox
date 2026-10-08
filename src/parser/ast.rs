//! Lox Grammer:
//! expression -> literal
//!               | unary
//!               | binary
//!               | grouping ;
//! literal    -> NUMBER | STRING | "true" | "false" | "nil" ;
//! grouping   -> "(" expression ")" ;
//! unary      -> ( "-" | "!" ) expression ;
//! binary     -> expression operator expression
//!operator    -> "==" | "!=" | "<" | "<=" | ">" | ">="
//!               | "+" | "-" | "*" | "/" ;

use std::fmt::{Display, Formatter};

use crate::scanner;

#[derive(Debug, Clone)]
pub enum LoxLiteral {
    Float(f64),
    String(String),
}

impl Display for LoxLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Float(f) => {
                format!("{f}")
            }
            Self::String(s) => s.clone(),
        };
        f.write_str(&val)
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: scanner::token::Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal(LoxLiteral),
    Unary {
        operator: scanner::token::Token,
        right: Box<Expr>,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rep = match self.clone() {
            Self::Binary {
                left,
                operator,
                right,
            } => Self::parenthesize(&operator.lexeme, &[*left, *right]),
            Self::Grouping { expression } => Self::parenthesize("group", &[*expression]),
            Self::Literal(lit) => {
                format!("{}", lit)
            }
            Self::Unary { operator, right } => Self::parenthesize(&operator.lexeme, &[*right]),
        };
        f.write_str(&rep)
    }
}

impl Expr {
    pub fn print(&self) -> String {
        format!("{self}")
    }

    fn parenthesize(_name: &str, _exprs: &[Expr]) -> String {
        let sb = String::new();

        todo!()
    }
}
