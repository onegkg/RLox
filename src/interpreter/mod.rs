use crate::{
    ast::{Expr, Program, Stmt},
    interpreter::{
        environment::Environment,
        values::{RuntimeError, Value, ValueWithTok},
    },
    scanner::token::TokenType,
};
pub mod environment;
pub mod values;

pub trait Interpretable {
    type Return;
    fn interpret(&self, env: &mut Environment) -> Result<Self::Return, RuntimeError>;
}

trait Evaluatable {
    type EvaluatesTo;
    fn evaluate(&self, env: &Environment) -> Result<Self::EvaluatesTo, RuntimeError>;
}

impl Interpretable for Expr {
    type Return = Value;
    fn interpret(&self, env: &mut Environment) -> Result<Self::Return, RuntimeError> {
        match self.evaluate(env) {
            Ok(ValueWithTok { value, .. }) => Ok(value),
            Err(e) => Err(e),
        }
    }
}

impl Evaluatable for Expr {
    type EvaluatesTo = ValueWithTok;
    fn evaluate(&self, env: &Environment) -> Result<Self::EvaluatesTo, RuntimeError> {
        match self.clone() {
            Self::Literal { literal, token } => Ok((literal, token).into()),
            Self::Grouping { expression, .. } => expression.evaluate(env),
            Self::Variable { name } => Ok((env.get(name.clone())?, name).into()),
            Self::Unary {
                operator,
                right,
                token,
            } => {
                let right = right.evaluate(env)?;
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
                let left = left.evaluate(env)?;
                let right = right.evaluate(env)?;
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
                        (Value::Number(_), Value::Number(0.)) => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Attempted division by zero".into(),
                        )),
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
                        // I think implicit conversions are annoying and would prefer not to support them
                        // (Value::String(left), v) => {
                        //     Ok((Value::String(left + &format!("{v}")), token).into())
                        // }
                        _ => Err(RuntimeError::new(
                            token,
                            "Runtime Error: Can only add two strings or two numbers".into(),
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

impl Interpretable for Stmt {
    type Return = ();
    fn interpret(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        match self {
            // evaluates the expression for side effects then discards result
            Stmt::Expression(e) => {
                e.evaluate(env)?;
                Ok(())
            }
            // evaluates the expression then prints the result
            Stmt::Print(e) => {
                let ValueWithTok { value, .. } = e.evaluate(env)?;
                println!("{value}");
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let mut val = Value::Nil;
                if let Some(i) = initializer {
                    val = i.interpret(env)?
                }
                env.define(name.lexeme.clone(), val);
                Ok(())
            }
        }
    }
}

impl Interpretable for Program {
    type Return = ();
    fn interpret(&self, env: &mut Environment) -> Result<(), RuntimeError> {
        for stmt in &self.0 {
            stmt.interpret(env)?;
        }
        Ok(())
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
