use crate::expression::Expression;

pub fn interpret(expr: Box<dyn Expression>) {
    let value = expr.evaluate();
    let out = value.print_value();
    let out_num = out.parse::<f32>();
    match out_num {
        Ok(f) => {
            println!("{}", f);
        },
        Err(_) => println!("{}", out)
    }
}
