use crate::{
    expression::RuntimeError,
    token::{LiteralValue, Token},
};
use std::collections::HashMap;

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Box<dyn LiteralValue>>>,
    enclosing: Option<Box<Environment>>,
}
impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        let values: HashMap<String, Option<Box<dyn LiteralValue>>> = HashMap::new();
        Self { values, enclosing }
    }

    pub fn define(&mut self, name: String, value: Option<Box<dyn LiteralValue>>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Option<Box<dyn LiteralValue>>> {
        if let Some(item) = self.values.get(&name.lexeme) {
            return Ok(item.clone());
        } else {
            if let Some(e) = &self.enclosing {
                return e.get(name);
            }
            let message = format!("Undefined variable '{}'.", name.lexeme);
            return Err(RuntimeError {
                token: name,
                message,
            });
        }
    }

    pub fn assign(&mut self, name: Token, value: Box<dyn LiteralValue>) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), Some(value));
            return Ok(());
        }
        if let Some(e) = self.enclosing.as_mut() {
            return e.assign(name, value);
        }

        let message = format!("Undefined variable '{}'.", name.lexeme);
        return Err(RuntimeError {
            token: name,
            message,
        });
    }

    pub fn revert_to(&mut self, target: &Environment) {
        self.values = target.values.clone();
    }

    pub fn enclosing(&self) -> Option<&Box<Environment>> {
        self.enclosing.as_ref()
    }
}
