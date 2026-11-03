use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{ast::LoxLiteral, scanner::token::Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

pub struct ValueWithTok {
    pub value: Value,
    pub token: Token,
}

impl ValueWithTok {
    fn new(token: Token, value: Value) -> Self {
        Self { token, value }
    }

    fn stringify(&self) -> String {
        todo!()
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
        write!(f, "{}: {}", self.token.line, self.msg)
    }
}

impl Error for RuntimeError {}

impl RuntimeError {
    pub fn new(token: Token, msg: String) -> Self {
        RuntimeError { token, msg }
    }
}
