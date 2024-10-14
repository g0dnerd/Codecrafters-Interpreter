use crate::{
    environment::Environment,
    expression::{Expression, RuntimeError},
    token::{LiteralType, Token},
};

type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug, Eq, PartialEq)]
pub enum StatementType {
    Expression,
    Print,
    Var,
}

pub trait Statement {
    fn evaluate(&self, environment: &mut Environment) -> Result<()>;
    fn get_type(&self) -> StatementType;
    fn dbg(&self) -> String;
}

pub struct ExpressionStatement {
    value: Box<dyn Expression>,
}
impl Statement for ExpressionStatement {
    fn evaluate(&self, environment: &mut Environment) -> Result<()> {
        match self.value.evaluate(environment) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        }
    }

    fn get_type(&self) -> StatementType {
        StatementType::Expression
    }

    fn dbg(&self) -> String {
        format!("Expression statement with value {}", self.value.accept())
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
                if let Some(parsed) = v {
                    let out = parsed.print_value();
                    if parsed.get_type() == LiteralType::NumberLiteral {
                        let n = out
                            .parse::<f32>()
                            .expect("to be able to parse number literal to f32");
                        println!("{n}");
                    } else {
                        println!("{out}");
                    }
                } else {
                    println!("nil");
                    return Ok(());
                }
            }
            Err(e) => {
                eprintln!("{e}");
                return Err(e);
            }
        }
        Ok(())
    }

    fn get_type(&self) -> StatementType {
        StatementType::Print
    }

    fn dbg(&self) -> String {
        format!("Print statement with value {}", self.value.accept())
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
                }
                Err(e) => return Err(e),
            }
        }
        environment.define(self.name.lexeme.clone(), None);
        Ok(())
    }

    fn get_type(&self) -> StatementType {
        StatementType::Var
    }

    fn dbg(&self) -> String {
        let v = if let Some(i) = &self.initializer {
            i.accept()
        } else {
            String::from("null")
        };
        format!("name: {}, initializer: {}", self.name.to_string(), v)
    }
}
impl VarStatement {
    pub fn new(name: Token, initializer: Option<Box<dyn Expression>>) -> Self {
        Self { name, initializer }
    }
}
