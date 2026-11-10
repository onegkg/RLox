use std::collections::HashMap;

use crate::{
    interpreter::values::{RuntimeError, Value},
    scanner::token::Token,
};

pub struct Environment(HashMap<String, Value>);

impl Environment {
    pub fn new() -> Self {
        Environment(HashMap::new())
    }

    /// Defines a mapping from `name` to `val` as in `var <name> = <val>;`.
    ///
    /// Shadowing variables is supported.
    pub fn define(&mut self, name: String, val: Value) {
        self.0.insert(name, val);
    }

    /// Evaluates a `Token` to the `Value` it refers to. If the caller attempts to evaluate an uninitialized identifier, a `RuntimeError` is returned.
    pub fn get(&self, name: Token) -> Result<Value, RuntimeError> {
        if let Some(val) = self.0.get(&name.lexeme) {
            Ok(val.clone())
        } else {
            let var_name = name.lexeme.clone();
            Err(RuntimeError::new(
                name,
                format!("Undefined variable '{var_name}'."),
            ))
        }
    }
}
