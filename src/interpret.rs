use crate::{expression::Expression, token::LiteralType};

pub fn interpret(expr: Box<dyn Expression>) -> Result<(), ()> {
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
            return Err(());
        }
    }
}
