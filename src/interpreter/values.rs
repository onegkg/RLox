use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

use crate::{ast::LoxLiteral, scanner::token::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::Number(n) => match n.fract() {
                0.0 => write!(f, "{n:.0}"),
                _ => write!(f, "{n}"),
            },
            Value::String(s) => write!(f, "{s}"),
        }
    }
}

pub struct ValueWithTok {
    pub value: Value,
    pub token: Token,
}

impl ValueWithTok {
    fn new(token: Token, value: Value) -> Self {
        Self { token, value }
    }
}

impl From<(LoxLiteral, Token)> for ValueWithTok {
    fn from(input: (LoxLiteral, Token)) -> Self {
        match input.0 {
            LoxLiteral::True => Self::new(input.1, Value::Bool(true)),
            LoxLiteral::False => Self::new(input.1, Value::Bool(false)),
            LoxLiteral::Nil => Self::new(input.1, Value::Nil),
            LoxLiteral::Float(f) => Self::new(input.1, Value::Number(f)),
            LoxLiteral::String(s) => Self::new(input.1, Value::String(s)),
        }
    }
}

impl From<(Value, Token)> for ValueWithTok {
    fn from(input: (Value, Token)) -> Self {
        Self {
            value: input.0,
            token: input.1,
        }
    }
}

impl ValueWithTok {
    pub fn expect_number(&self, msg: &str) -> Result<f64, RuntimeError> {
        match self.value {
            Value::Number(n) => Ok(n),
            _ => Err(RuntimeError::new(self.token.clone(), msg.into())),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RuntimeError {
    token: Token,
    msg: String,
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n[line {}]", self.msg, self.token.line)
    }
}

impl Error for RuntimeError {}

impl RuntimeError {
    pub fn new(token: Token, msg: String) -> Self {
        RuntimeError { token, msg }
    }
}
