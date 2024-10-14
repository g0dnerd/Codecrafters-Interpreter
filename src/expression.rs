use crate::interpret::{is_equal, is_truthy, parenthesize};
use crate::{
    environment::Environment,
    token::{BooleanLiteral, LiteralType, LiteralValue, NumberLiteral, StringLiteral, Token},
    TokenType,
};
use std::fmt;

type Result<T> = std::result::Result<T, RuntimeError>;

pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.token.line)
    }
}

pub trait Expression {
    fn accept(&self) -> String;
    fn evaluate(&self, environment: &mut Environment) -> Result<Option<Box<dyn LiteralValue>>>;
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

    fn evaluate(&self, environment: &mut Environment) -> Result<Option<Box<dyn LiteralValue>>> {
        if let (Some(left), Some(right)) = (
            self.left.evaluate(environment)?,
            self.right.evaluate(environment)?,
        ) {
            let left_type = left.get_type();
            let right_type = right.get_type();

            let left_val = left.print_value();
            let right_val = right.print_value();

            if self.operator.token_type == TokenType::BangEqual {
                let eq = !is_equal(left, right);
                return Ok(Some(Box::new(BooleanLiteral { value: eq })));
            } else if self.operator.token_type == TokenType::EqualEqual {
                let eq = is_equal(left, right);
                return Ok(Some(Box::new(BooleanLiteral { value: eq })));
            }

            if left_type == LiteralType::NumberLiteral && right_type == LiteralType::NumberLiteral {
                let left_num = left_val
                    .parse::<f32>()
                    .expect("to be able to parse left NumberLiteral in binary expression to f32");
                let right_num = right_val
                    .parse::<f32>()
                    .expect("to be able to parse right NumberLiteral in binary expression to f32");

                match self.operator.token_type {
                    TokenType::Minus => {
                        return Ok(Some(Box::new(NumberLiteral {
                            value: left_num - right_num,
                        })));
                    }
                    TokenType::Slash => {
                        return Ok(Some(Box::new(NumberLiteral {
                            value: left_num / right_num,
                        })));
                    }
                    TokenType::Star => {
                        return Ok(Some(Box::new(NumberLiteral {
                            value: left_num * right_num,
                        })));
                    }
                    TokenType::Plus => {
                        return Ok(Some(Box::new(NumberLiteral {
                            value: left_num + right_num,
                        })));
                    }
                    TokenType::Greater => {
                        return Ok(Some(Box::new(BooleanLiteral {
                            value: left_num > right_num,
                        })));
                    }
                    TokenType::GreaterEqual => {
                        return Ok(Some(Box::new(BooleanLiteral {
                            value: left_num >= right_num,
                        })));
                    }
                    TokenType::Less => {
                        return Ok(Some(Box::new(BooleanLiteral {
                            value: left_num < right_num,
                        })));
                    }
                    TokenType::LessEqual => {
                        return Ok(Some(Box::new(BooleanLiteral {
                            value: left_num <= right_num,
                        })));
                    }
                    _ => (),
                }
            } else if left_type == LiteralType::StringLiteral
                && right_type == LiteralType::StringLiteral
            {
                if self.operator.token_type == TokenType::Plus {
                    let mut left_string = left_val.to_owned();
                    left_string.push_str(&right_val.to_owned());
                    return Ok(Some(Box::new(StringLiteral { value: left_string })));
                }
                return Err(RuntimeError {
                    token: self.operator.clone(),
                    message: String::from("Operands must be numbers."),
                });
            }
            Err(RuntimeError {
                token: self.operator.clone(),
                message: String::from("Operands must be numbers or strings."),
            })
        } else {
            return Err(RuntimeError {
                token: self.operator.clone(),
                message: String::from("expected value in expression"),
            });
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

    fn evaluate(&self, environment: &mut Environment) -> Result<Option<Box<dyn LiteralValue>>> {
        self.expression.evaluate(environment)
    }
}

impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

pub struct Literal {
    value: Box<dyn LiteralValue>,
}

impl Expression for Literal {
    fn accept(&self) -> String {
        self.value.print_value()
    }

    fn evaluate(&self, _environment: &mut Environment) -> Result<Option<Box<dyn LiteralValue>>> {
        Ok(Some(self.value.clone()))
    }
}

impl Literal {
    pub fn new(value: Box<dyn LiteralValue>) -> Self {
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

    fn evaluate(&self, environment: &mut Environment) -> Result<Option<Box<dyn LiteralValue>>> {
        if let Some(right) = self.right.evaluate(environment)? {
            match self.operator.token_type {
                TokenType::Minus => {
                    if !(right.get_type() == LiteralType::NumberLiteral) {
                        return Err(RuntimeError {
                            token: self.operator.clone(),
                            message: String::from("Operand must be a number."),
                        });
                    }
                    let num_value: f32 = right
                        .print_value()
                        .parse()
                        .expect("to be able to parse Number Literal to f32");
                    return Ok(Some(Box::new(NumberLiteral { value: -num_value })));
                }
                TokenType::Bang => {
                    return Ok(Some(Box::new(BooleanLiteral {
                        value: !is_truthy(right),
                    })));
                }
                _ => {
                    return Err(RuntimeError {
                        token: self.operator.clone(),
                        message: String::from("Operand must be a number."),
                    })
                }
            }
        }
        Err(RuntimeError {
            token: self.operator.clone(),
            message: String::from("Expected value in unary expression"),
        })
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expression>) -> Self {
        Self { operator, right }
    }
}

pub struct Variable {
    name: Token,
}
impl Expression for Variable {
    fn accept(&self) -> String {
        self.name.to_string()
    }

    fn evaluate(&self, environment: &mut Environment) -> Result<Option<Box<dyn LiteralValue>>> {
        environment.get(self.name.clone())
    }
}
impl Variable {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}
