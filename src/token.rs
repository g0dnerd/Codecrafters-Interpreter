use std::fmt;
use strum_macros::Display;
use crate::TokenType;

pub trait LiteralValue {
    fn print_value(&self) -> String;
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Box<dyn LiteralValue>>,
    line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let literal_out: String = if let Some(l) = &self.literal {
            l.print_value()
        } else { String::from("null") };
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal_out)
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Box<dyn LiteralValue>>, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

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
}

pub struct StringLiteral {
    pub value: String,
}

impl LiteralValue for StringLiteral {
    fn print_value(&self) -> String {
        self.value.clone()
    }
}
