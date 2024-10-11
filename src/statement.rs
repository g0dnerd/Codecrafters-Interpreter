use crate::{environment::Environment, expression::{Expression, RuntimeError}, token::{LiteralType, Token}};

type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Statement {
    fn evaluate(&self, environment: &mut Environment) -> Result<()>;
}

pub struct ExpressionStatement {
    value: Box<dyn Expression>,
}
impl Statement for ExpressionStatement {
    fn evaluate(&self, environment: &mut Environment) -> Result<()> {
        match self.value.evaluate(environment) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e)
        }
    }
}
impl ExpressionStatement {
    pub fn new(value: Box<dyn Expression>) -> Self {
        Self { value }
    }
}

pub struct PrintStatement {
    value: Box<dyn Expression>,
}
impl Statement for PrintStatement {
    fn evaluate(&self, environment: &mut Environment) -> Result<()> {
        match self.value.evaluate(environment) {
            Ok(v) => {
                let value = v.unwrap();
                let out = value.print_value();
                if value.get_type() == LiteralType::NumberLiteral {
                    let n = out.parse::<f32>()
                        .expect("to be able to parse number literal to f32");
                    println!("{n}");
                } else {
                    println!("{out}");
                }
            },
            Err(e) => {
                eprintln!("{e}");
                return Err(e);
            }
        }
        Ok(())
    }
}
impl PrintStatement {
    pub fn new(value: Box<dyn Expression>) -> Self {
        Self { value }
    }
}

pub struct VarStatement {
    name: Token,
    initializer: Option<Box<dyn Expression>>,
}
impl Statement for VarStatement {
    fn evaluate(&self, environment: &mut Environment) -> Result<()> {
        if let Some(initializer) = &self.initializer {
            match initializer.evaluate(environment) {
                Ok(value) => {
                    environment.define(self.name.lexeme.clone(), value);
                    return Ok(());
                },
                Err(e) => return Err(e)
            }
        }
        environment.define(self.name.lexeme.clone(), None);
        Ok(())
    }
}
impl VarStatement {
    pub fn new(name: Token, initializer: Option<Box<dyn Expression>>) -> Self {
        Self { name, initializer }
    }
}
