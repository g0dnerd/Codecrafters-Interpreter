use crate::{token::{BooleanLiteral, LiteralValue, NumberLiteral, StringLiteral, Token}, TokenType};

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
    fn evaluate(&self) -> Box<dyn LiteralValue>;
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

    fn evaluate(&self) -> Box<dyn LiteralValue> {
        let left = self.left.evaluate();
        let right = self.right.evaluate();

        let is_left_str = left.get_name().contains("StringLiteral");
        let is_right_str = right.get_name().contains("StringLiteral");

        let left_val = left.print_value();
        let right_val = right.print_value();

        let left_num = match left_val.parse::<f32>() {
            Ok(e) => Some(e),
            Err(_) => None
        };
        let right_num = match right_val.parse::<f32>() {
            Ok(e) => Some(e),
            Err(_) => None
        };

        match self.operator.token_type {
            TokenType::Minus => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(NumberLiteral{ value: l - r });
                    }
                }
                panic!("Trying to negate non-number literal");
            },
            TokenType::Slash => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(NumberLiteral{ value: l / r });
                    }
                }
                panic!("Trying to negate non-number literal");
            },
            TokenType::Star => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(NumberLiteral{ value: l * r });
                    }
                }
                panic!("Trying to negate non-number literal");
            },
            TokenType::Plus => {
                if is_left_str && is_right_str {
                    let mut left_string = left_val.to_owned();
                    left_string.push_str(&right_val.to_owned());
                    return Box::new(StringLiteral{ value: left_string });
                } else if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(NumberLiteral{ value: l + r });
                    }
                }
                panic!("Trying to add invalid types");
            },
            TokenType::Greater => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(BooleanLiteral{ value: l > r });
                    }
                }
                panic!("Trying to compare to non-numeric values");
            },
            TokenType::GreaterEqual => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(BooleanLiteral{ value: l >= r });
                    }
                }
                panic!("Trying to compare to non-numeric values");
            },
            TokenType::Less => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(BooleanLiteral{ value: l > r });
                    }
                }
                panic!("Trying to compare to non-numeric values");
            },
            TokenType::LessEqual => {
                if !is_left_str && !is_right_str {
                    if let (Some(l), Some(r)) = (left_num, right_num) {
                        return Box::new(BooleanLiteral{ value: l <= r });
                    }
                }
                panic!("Trying to compare to non-numeric values");
            },
            TokenType::BangEqual => return Box::new(BooleanLiteral{ value: !is_equal(left, right)}),
            TokenType::EqualEqual => return Box::new(BooleanLiteral{ value: is_equal(left, right)}),
            _ => panic!("Invalid operation in binary expression")
        }
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

    fn evaluate(&self) -> Box<dyn LiteralValue> {
        self.expression.evaluate()
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

    fn evaluate(&self) -> Box<dyn LiteralValue> {
        if let Some(v) = &self.value {
            return v.clone();
        } else {
            panic!("Evaluating empty literal expression");
        }
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

    fn evaluate(&self) -> Box<dyn LiteralValue> {
        let right = self.right.evaluate();

        match self.operator.token_type {
            TokenType::Minus => {
                let num_value: f32 = right.print_value().parse()
                    .expect("Unable to parse negated expression to f32");
                return Box::new(NumberLiteral{ value: -num_value });
            },
            TokenType::Bang => {
                return Box::new(BooleanLiteral{ value: !is_truthy(right)});
            },
            _ => panic!("Invalid negated expression")
        }
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Self { operator, right }
    }
}

fn is_truthy(expr: Box<dyn LiteralValue>) -> bool {
    let expr_val = expr.print_value();
    match expr_val.as_ref() {
        "nil" | "false" => return false,
        _ => return true
    }
}

fn is_equal(left: Box<dyn LiteralValue>, right: Box<dyn LiteralValue>) -> bool {
    let left_val = left.print_value();
    let right_val = right.print_value();
    &left_val == &right_val
}
