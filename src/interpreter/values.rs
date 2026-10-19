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
