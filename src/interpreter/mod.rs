use crate::{
    ast::Expr,
    interpreter::values::{RuntimeError, Value, ValueWithTok},
    scanner::token::TokenType,
};
pub mod values;

pub trait Interpret {
    fn interpret(&self) -> Result<ValueWithTok, RuntimeError>;
}

impl Interpret for Expr {
    fn interpret(&self) -> Result<ValueWithTok, RuntimeError> {
        match self.clone() {
            Self::Literal { literal, token } => Ok((literal, token).into()),
            Self::Grouping { expression, .. } => expression.interpret(),
            Self::Unary {
                operator,
                right,
                token,
            } => {
                let right = right.interpret().expect("nothing returns None");
                match operator.kind {
                    TokenType::Minus => {
                        let n = right
                            .expect_number("Runtime Error: Can only negate a number with `-`")
                            .expect("TODO: error handling");
                        Ok((Value::Number(-n), token).into())
                    }
                    TokenType::Bang => Ok((Value::Bool(!is_truthy(&right)), token).into()),
                    _ => unreachable!("no other unary operations"),
                }
            }
            Self::Binary {
                left,
                operator,
                right,
                token,
            } => {
                let left = left.interpret().expect("nothing returns None");
                let right = right.interpret().expect("nothing returns None");
                match operator.kind {
                    TokenType::Minus => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left - right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only subtract two numbers"),
                    },
                    TokenType::Slash => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left / right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only divide two numbers"),
                    },
                    TokenType::Star => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left * right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only multiply two numbers"),
                    },
                    TokenType::Greater => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left > right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only compare two numbers"),
                    },
                    TokenType::Less => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left < right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only compare two numbers"),
                    },
                    TokenType::GreaterEqual => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left >= right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only compare two numbers"),
                    },
                    TokenType::LessEqual => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left <= right), token).into())
                        }
                        _ => panic!("Runtime Error: Can only compare two numbers"),
                    },
                    TokenType::BangEqual => {
                        Ok((Value::Bool(!is_equal(&left, &right)), token).into())
                    }
                    TokenType::EqualEqual => {
                        Ok((Value::Bool(is_equal(&left, &right)), token).into())
                    }
                    TokenType::Plus => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left + right), token).into())
                        }
                        (Value::String(left), Value::String(right)) => {
                            Ok((Value::String(left + &right), token).into())
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn is_truthy(v: &ValueWithTok) -> bool {
    match v.value {
        Value::Nil => false,
        Value::Bool(b) => b,
        _ => true,
    }
}

fn is_equal(left: &ValueWithTok, right: &ValueWithTok) -> bool {
    match (&left.value, &right.value) {
        (Value::String(sl), Value::String(sr)) => sl == sr,
        (Value::Bool(bl), Value::Bool(br)) => bl == br,
        (Value::Number(nl), Value::Number(nr)) => nl == nr,
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(_), _) | (Value::String(_), _) | (Value::Number(_), _) | (Value::Nil, _) => {
            false
        }
    }
}
