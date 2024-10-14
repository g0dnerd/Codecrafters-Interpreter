use crate::{
    expression::RuntimeError,
    token::{LiteralValue, Token},
};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Box<dyn LiteralValue>>>,
}
impl Environment {
    pub fn new() -> Self {
        let values: HashMap<String, Option<Box<dyn LiteralValue>>> = HashMap::new();
        Self { values }
    }

    pub fn define(&mut self, name: String, value: Option<Box<dyn LiteralValue>>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Option<Box<dyn LiteralValue>>> {
        if let Some(item) = self.values.get(&name.lexeme) {
            return Ok(item.clone());
        } else {
            let message = format!("Undefined variable '{}'.", name.lexeme);
            return Err(RuntimeError {
                token: name,
                message,
            });
        }
    }

    pub fn assign(&mut self, name: Token, value: Box<dyn LiteralValue>) -> Result<()> {
        match self.values.insert(name.lexeme.clone(), Some(value)) {
            Some(_) => Ok(()),
            _ => {
                let message = format!("Undefined variable '{}'.", name.lexeme);
                Err(RuntimeError {
                    token: name,
                    message,
                })
            }
        }
    }

    pub fn revert_to(&mut self, target: &Environment) {
        self.values = target.values.clone();
    }
}
