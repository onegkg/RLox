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

use crate::scanner::{self, token::Token};

#[derive(Debug, Clone)]
pub enum LoxLiteral {
    True,
    False,
    Nil,
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
            Self::True => format!("true"),
            Self::False => format!("false"),
            Self::Nil => format!("nil"),
        };
        f.write_str(&val)
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: scanner::token::Token,
        right: Box<Expr>,
        token: Token,
    },
    Grouping {
        expression: Box<Expr>,
        token: Token,
    },
    Literal {
        literal: LoxLiteral,
        token: Token,
    },
    Unary {
        operator: scanner::token::Token,
        right: Box<Expr>,
        token: Token,
    },
    Variable {
        name: Token,
    },
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let rep = match self.clone() {
            Self::Binary {
                left,
                operator,
                right,
                ..
            } => Self::parenthesize(&operator.lexeme, &[*left, *right]),
            Self::Grouping { expression, .. } => Self::parenthesize("group", &[*expression]),
            Self::Literal { literal, .. } => {
                format!("{}", literal)
            }
            Self::Variable { name } => format!("{:?}", name),
            Self::Unary {
                operator, right, ..
            } => Self::parenthesize(&operator.lexeme, &[*right]),
        };
        f.write_str(&rep)
    }
}

impl Expr {
    pub fn print(&self) -> String {
        format!("{self}")
    }

    fn parenthesize(name: &str, exprs: &[Expr]) -> String {
        let mut sb = String::new();
        sb.push('(');
        sb.push_str(name);
        for e in exprs {
            sb.push(' ');
            sb.push_str(&e.print());
        }
        sb.push(')');
        sb
    }
}

#[derive(Debug, Clone)]
pub struct Program(pub Vec<Stmt>);

impl From<Vec<Stmt>> for Program {
    fn from(input: Vec<Stmt>) -> Self {
        Self(input)
    }
}
