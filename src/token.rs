use std::fmt;
use strum_macros::Display;

#[derive(Debug, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    // Single-character tokens
    LeftParen, // (
    RightParen, // )
    LeftBrace, // {
    RightBrace, // }
    Comma, // ,
    Dot, // .
    Minus, // -
    Plus, // +
    Semicolon, // ;
    Slash, // /
    Star, // *

    // One or two-character tokens
    Bang, // !
    BangEqual, // !=
    Equal, // =
    EqualEqual, // ==
    Greater, // >
    GreaterEqual, // >=
    Less, // <
    LessEqual, // <=

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

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
        self.value.to_string()
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
