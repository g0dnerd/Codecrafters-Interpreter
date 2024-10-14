use crate::environment::Environment;
use crate::expression::{Expression, RuntimeError};
use crate::statement::Statement;
use crate::token::{LiteralType, LiteralValue};

type Result<T> = std::result::Result<T, RuntimeError>;

pub struct Interpreter {
    statements: Vec<Box<dyn Statement>>,
    environment: Environment,
}
impl Interpreter {
    pub fn new(statements: Vec<Box<dyn Statement>>) -> Self {
        Self {
            statements,
            environment: Environment::new(None),
        }
    }

    pub fn interpret(&mut self) -> Result<()> {
        for s in self.statements.iter_mut() {
            match s.evaluate(&mut self.environment) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

pub fn is_truthy(expr: Box<dyn LiteralValue>) -> bool {
    match expr.get_type() {
        LiteralType::NilLiteral => return false,
        LiteralType::BooleanLiteral => {
            let expr_val = expr.print_value();
            match expr_val.as_ref() {
                "false" => return false,
                _ => return true,
            }
        }
        _ => return true,
    }
}

pub fn is_equal(left: Box<dyn LiteralValue>, right: Box<dyn LiteralValue>) -> bool {
    let left_val = left.print_value();
    let right_val = right.print_value();
    &left_val == &right_val
}

pub fn parenthesize(name: &str, expressions: Vec<&Box<dyn Expression>>) -> String {
    let mut parsed = String::new();
    parsed.push('(');
    parsed.push_str(name);

    for expr in expressions {
        parsed.push(' ');
        parsed.push_str(&expr.accept());
    }

    parsed.push(')');
    parsed
}

pub fn interpret_single_expr(
    expr: Box<dyn Expression>,
    environment: &mut Environment,
) -> Result<()> {
    let value = expr.evaluate(environment);
    match value {
        Ok(v) => {
            if let Some(value) = v {
                let expr_type = value.get_type();
                let expr_value = value.print_value();
                if expr_type == LiteralType::NumberLiteral {
                    let out_num = expr_value
                        .parse::<f32>()
                        .expect("to be able to parse number expression to f32");
                    println!("{}", out_num);
                    return Ok(());
                } else {
                    println!("{}", expr_value);
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("Error: {e}");
            return Err(e);
        }
    }
}
