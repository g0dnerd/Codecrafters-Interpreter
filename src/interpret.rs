use crate::expression::Expression;

pub fn interpret(expr: Box<dyn Expression>) {
    let value = expr.evaluate();
    eprintln!("{}", value.print_value());
}
