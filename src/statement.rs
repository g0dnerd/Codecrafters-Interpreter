use crate::{expression::{Expression, RuntimeError}, token::LiteralType};

type Result<T> = std::result::Result<T, RuntimeError>;

pub trait Statement {
    fn evaluate(&self) -> Result<()>;
}

pub struct ExpressionStatement {
    value: Box<dyn Expression>,
}
impl Statement for ExpressionStatement {
    fn evaluate(&self) -> Result<()> {
        match self.value.evaluate() {
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
    fn evaluate(&self) -> Result<()> {
        match self.value.evaluate() {
            Ok(v) => {
                let out = v.print_value();
                if v.get_type() == LiteralType::NumberLiteral {
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
