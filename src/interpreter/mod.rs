use crate::{
    ast::Expr,
    interpreter::values::{RuntimeError, Value, ValueWithTok},
    scanner::token::TokenType,
};
pub mod values;

/// Requirement for internal interpretation of the ast
trait InternallyInterpretable {
    fn interpret_internal(&self) -> Result<ValueWithTok, RuntimeError>;
}

/// Wraps the `interpret_internal` function with a user facing api
pub trait Interpretable: InternallyInterpretable {
    fn interpret(&self);
}

impl Interpretable for Expr {
    fn interpret(&self) {
        match self.interpret_internal() {
            Ok(ValueWithTok { value, .. }) => println!("{}", value.stringify()),
            // TODO: Properly Implement Display for RuntimeError
            Err(e) => println!("{e}"),
        }
    }
}

impl InternallyInterpretable for Expr {
    fn interpret_internal(&self) -> Result<ValueWithTok, RuntimeError> {
        match self.clone() {
            Self::Literal { literal, token } => Ok((literal, token).into()),
            Self::Grouping { expression, .. } => expression.interpret_internal(),
            Self::Unary {
                operator,
                right,
                token,
            } => {
                let right = right.interpret_internal()?;
                match operator.kind {
                    TokenType::Minus => {
                        let n = right
                            .expect_number("Runtime Error: Can only negate a number with `-`")?;
                        Ok((Value::Number(-n), token).into())
                    }
                    TokenType::Bang => Ok((Value::Bool(!is_truthy(&right)), token).into()),
                    _ => Err(RuntimeError::new(token, "Invalid unary operator".into())),
                }
            }
            Self::Binary {
                left,
                operator,
                right,
                token,
            } => {
                let left = left.interpret_internal()?;
                let right = right.interpret_internal()?;
                match operator.kind {
                    TokenType::Minus => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left - right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only subtract two numbers".into(),
                        )),
                    },
                    TokenType::Slash => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left / right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only divide two numbers".into(),
                        )),
                    },
                    TokenType::Star => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Number(left * right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only multiply two numbers".into(),
                        )),
                    },
                    TokenType::Greater => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left > right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only compare two numbers".into(),
                        )),
                    },
                    TokenType::Less => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left < right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only compare two numbers".into(),
                        )),
                    },
                    TokenType::GreaterEqual => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left >= right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only compare two numbers".into(),
                        )),
                    },
                    TokenType::LessEqual => match (left.value, right.value) {
                        (Value::Number(left), Value::Number(right)) => {
                            Ok((Value::Bool(left <= right), token).into())
                        }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only compare two numbers".into(),
                        )),
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
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only add strings or numbers".into(),
                        )),
                    },
                    _ => Err(RuntimeError::new(
                        token,
                        "Runtime Error: Unexpected token".into(),
                    )),
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
