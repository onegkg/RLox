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
                let right = right.interpret().expect("nothing return None");
                match operator.kind {
                    TokenType::Minus => {
                        let Value::Number(n) = right else {
                            panic!("Runtime Error: Can only negate a number with `-`")
                        };
                        Some(Value::Number(-n))
                    }
                    TokenType::Bang => Some(Value::Bool(!is_truthy(&right))),
                    _ => unreachable!("idk ci says it is so I believe them ig?"),
                }
            }
            Self::Binary { .. } => todo!(),
        }
    }
}

fn is_truthy(v: &Value) -> bool {
    todo!()
}
