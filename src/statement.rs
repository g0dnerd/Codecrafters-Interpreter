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
    Block,
}

pub trait Statement {
    fn evaluate(&self, env: &mut Environment) -> Result<()>;
    fn get_type(&self) -> StatementType;
    fn dbg(&self) -> String;
}

pub struct ExpressionStmt {
    value: Box<dyn Expression>,
}
impl Statement for ExpressionStmt {
    fn evaluate(&self, env: &mut Environment) -> Result<()> {
        match self.value.evaluate(env) {
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
impl ExpressionStmt {
    pub fn new(value: Box<dyn Expression>) -> Self {
        Self { value }
    }
}

pub struct PrintStmt {
    value: Box<dyn Expression>,
}
impl Statement for PrintStmt {
    fn evaluate(&self, env: &mut Environment) -> Result<()> {
        match self.value.evaluate(env) {
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
                eprintln!("Error while evaluating print statement: {e}");
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
impl PrintStmt {
    pub fn new(value: Box<dyn Expression>) -> Self {
        Self { value }
    }
}

pub struct VarStmt {
    name: Token,
    initializer: Option<Box<dyn Expression>>,
}
impl Statement for VarStmt {
    fn evaluate(&self, env: &mut Environment) -> Result<()> {
        if let Some(initializer) = &self.initializer {
            match initializer.evaluate(env) {
                Ok(value) => {
                    env.define(self.name.lexeme.clone(), value);
                    return Ok(());
                }
                Err(e) => return Err(e),
            }
        } else {
            env.define(self.name.lexeme.clone(), None);
            Ok(())
        }
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
impl VarStmt {
    pub fn new(name: Token, initializer: Option<Box<dyn Expression>>) -> Self {
        Self { name, initializer }
    }
}

pub struct BlockStmt {
    stmts: Vec<Box<dyn Statement>>,
}
impl Statement for BlockStmt {
    fn evaluate(&self, env: &mut Environment) -> Result<()> {
        let previous = env.clone();
        let mut enclosing = Environment::new(Some(Box::new(env.clone())));
        for s in &self.stmts {
            match s.evaluate(&mut enclosing) {
                Ok(_) => (),
                Err(e) => {
                    env.revert_to(&previous);
                    return Err(e);
                }
            }
        }
        let outer = enclosing
            .enclosing()
            .expect("expected enclosing environment");
        env.revert_to(outer);
        Ok(())
    }

    fn get_type(&self) -> StatementType {
        StatementType::Block
    }

    fn dbg(&self) -> String {
        let mut o = String::new();
        for s in &self.stmts {
            o.push_str(&s.dbg());
        }
        o
    }
}
impl BlockStmt {
    pub fn new(stmts: Vec<Box<dyn Statement>>) -> Self {
        Self { stmts }
    }
}
