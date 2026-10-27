use crate::{ast::Expr, interpreter::values::Value, scanner::token::TokenType};
pub mod values;

pub trait Interpret {
    fn interpret(&self) -> Option<Value>;
}

impl Interpret for Expr {
    fn interpret(&self) -> Option<Value> {
        match self.clone() {
            Self::Literal(lit) => Some(Value::from(lit)),
            Self::Grouping { expression } => expression.interpret(),
            Self::Unary { operator, right } => {
                let right = right.interpret().expect("nothing returns None");
                match operator.kind {
                    TokenType::Minus => {
                        let n =
                            right.expect_number("Runtime Error: Can only negate a number with `-`");
                        Some(Value::Number(-n))
                    }
                    TokenType::Bang => Some(Value::Bool(!is_truthy(&right))),
                    _ => unreachable!("no other unary operations"),
                }
            }
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.interpret().expect("nothing returns None");
                let right = right.interpret().expect("nothing returns None");
                match operator.kind {
                    TokenType::Minus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Some(Value::Number(left - right))
                        }
                        _ => panic!("Runtime Error: Can only subtract two numbers"),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Some(Value::Number(left / right))
                        }
                        _ => panic!("Runtime Error: Can only divide two numbers"),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Some(Value::Number(left * right))
                        }
                        _ => panic!("Runtime Error: Can only multiply two numbers"),
                    },
                    TokenType::Greater => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Some(Value::Bool(left > right))
                        }
                        _ => panic!("Runtime Error: Can only compare two numbers"),
                    },
                    TokenType::Less => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Some(Value::Bool(left < right))
                        }
                        _ => panic!("Runtime Error: Can only compare two numbers"),
                    },
                    TokenType::GreaterEqual => todo!(),
                    TokenType::LessEqual => todo!(),
                    TokenType::Plus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => {
                            Some(Value::Number(left + right))
                        }
                        (Value::String(left), Value::String(right)) => {
                            Some(Value::String(left + &right))
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn is_truthy(v: &Value) -> bool {
    match *v {
        Value::Nil => false,
        Value::Bool(b) => b,
        _ => true,
    }
}
