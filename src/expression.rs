use crate::token::{LiteralValue, Token};

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

pub trait Expression {
    fn accept(&self) -> String;
}

pub struct Binary {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl Expression for Binary {
    fn accept(&self) -> String {
        parenthesize(&self.operator.lexeme, vec![&self.left, &self.right])
    }
}

impl Binary {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

pub struct Grouping {
    expression: Box<dyn Expression>,
}

impl Expression for Grouping {
    fn accept(&self) -> String {
        parenthesize("group", vec![&self.expression])
    }
}

impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

pub struct Literal {
    value: Option<Box<dyn LiteralValue>>,
}

impl Expression for Literal {
    fn accept(&self) -> String {
        return if let Some(v) = &self.value {
            v.print_value()
        } else {
            String::from("nil")
        };
    }
}

impl Literal {
    pub fn new(value: Option<Box<dyn LiteralValue>>) -> Self {
        Self { value }
    }
}

pub struct Unary {
    operator: Token,
    right: Box<dyn Expression>,
}

impl Expression for Unary {
    fn accept(&self) -> String {
        parenthesize(&self.operator.lexeme, vec![&self.right])
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Self { operator, right }
    }
}
