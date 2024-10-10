use crate::TokenType;
use std::fmt;

pub trait LiteralValue: LiteralValueClone {
    fn print_value(&self) -> String;
    fn get_type(&self) -> LiteralType;
}

pub trait LiteralValueClone {
    fn clone_box(&self) -> Box<dyn LiteralValue>;
}

impl<T> LiteralValueClone for T
where
    T: 'static + LiteralValue + Clone,
{
    fn clone_box(&self) -> Box<dyn LiteralValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn LiteralValue> {
    fn clone(&self) -> Box<dyn LiteralValue> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn LiteralValue>>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal_out: String = if let Some(l) = &self.literal {
            l.print_value()
        } else {
            String::from("null")
        };
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_out)
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn LiteralValue>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum LiteralType {
    NumberLiteral,
    StringLiteral,
    BooleanLiteral,
    NilLiteral,
}

#[derive(Clone)]
pub struct NumberLiteral {
    pub value: f32,
}

impl LiteralValue for NumberLiteral {
    fn print_value(&self) -> String {
        // In Rust, `42.0f32.to_string()` yields `42` and not `42.0`,
        // so we have to handle that case manually
        if self.value.fract() == 0.0 {
            return format!("{:.1}", self.value);
        } else {
            self.value.to_string()
        }
    }

    fn get_type(&self) -> LiteralType {
        LiteralType::NumberLiteral
    }
}

#[derive(Clone)]
pub struct StringLiteral {
    pub value: String,
}

impl LiteralValue for StringLiteral {
    fn print_value(&self) -> String {
        self.value.clone()
    }

    fn get_type(&self) -> LiteralType {
        LiteralType::StringLiteral
    }
}

#[derive(Clone)]
pub struct BooleanLiteral {
    pub value: bool,
}

impl LiteralValue for BooleanLiteral {
    fn print_value(&self) -> String {
        if !self.value {
            return String::from("false");
        }
        String::from("true")
    }

    fn get_type(&self) -> LiteralType {
        LiteralType::BooleanLiteral
    }
}

#[derive(Clone)]
pub struct NilLiteral;

impl LiteralValue for NilLiteral {
    fn print_value(&self) -> String {
        String::from("nil")
    }

    fn get_type(&self) -> LiteralType {
        LiteralType::NilLiteral
    }
}
