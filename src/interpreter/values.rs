use crate::ast::LoxLiteral;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
}

impl From<LoxLiteral> for Value {
    fn from(input: LoxLiteral) -> Self {
        match input {
            LoxLiteral::True => Self::Bool(true),
            LoxLiteral::False => Self::Bool(false),
            LoxLiteral::Nil => Self::Nil,
            LoxLiteral::Float(f) => Value::Number(f),
            LoxLiteral::String(s) => Value::String(s),
        }
    }
}

impl Value {
    pub fn expect_bool(&self, msg: &str) -> bool {
        let Self::Bool(b) = self else { panic!("{msg}") };
        *b
    }

    pub fn expect_number(&self, msg: &str) -> f64 {
        let Self::Number(n) = self else {
            panic!("{msg}")
        };
        *n
    }

    pub fn expect_string(&self, msg: &str) -> &str {
        let Self::String(s) = self else {
            panic!("{msg}")
        };
        s
    }
}
