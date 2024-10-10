use crate::expression::{Expression, RuntimeError};
use crate::statement::Statement;
use crate::token::{LiteralType, LiteralValue};

type Result<T> = std::result::Result<T, RuntimeError>;

pub fn interpret_single_expr(expr: Box<dyn Expression>) -> Result<()> {
    let value = expr.evaluate();
    match value {
        Ok(v) => {
            let expr_type = v.get_type();
            let expr_value = v.print_value();
            if expr_type == LiteralType::NumberLiteral {
                let out_num = expr_value.parse::<f32>()
                    .expect("to be able to parse number expression to f32");
                println!("{}", out_num);
                return Ok(());
            } else {
                println!("{}", expr_value);
                return Ok(());
            }
        },
        Err(e) => {
            eprintln!("{e}");
            return Err(e);
        },
    }
}

pub fn interpret(statements: Vec<Box<dyn Statement>>) -> Result<()> {
    for s in statements {
        match execute(s) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
    }
    Ok(())
}

fn execute(statement: Box<dyn Statement>) -> Result<()> {
    statement.evaluate()
}

pub fn is_truthy(expr: Box<dyn LiteralValue>) -> bool {
    match expr.get_type() {
        LiteralType::NilLiteral => return false,
        LiteralType::BooleanLiteral => {
            let expr_val = expr.print_value();
            match expr_val.as_ref() {
                "false" => return false,
                _ => return true
            }
        },
        _ => return true
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
