use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;
use strum_macros::Display;

pub mod scan;
pub mod token;
pub mod parse;
pub mod expression;
pub mod ast;

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}

#[derive(Debug, Display, Copy, Clone, Eq, PartialEq)]
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

pub static KEYWORDS: Lazy<Mutex<HashMap<String, TokenType>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(String::from("and"), TokenType::And);
    m.insert(String::from("class"), TokenType::Class);
    m.insert(String::from("else"), TokenType::Else);
    m.insert(String::from("false"), TokenType::False);
    m.insert(String::from("fun"), TokenType::Fun);
    m.insert(String::from("for"), TokenType::For);
    m.insert(String::from("if"), TokenType::If);
    m.insert(String::from("nil"), TokenType::Nil);
    m.insert(String::from("or"), TokenType::Or);
    m.insert(String::from("print"), TokenType::Print);
    m.insert(String::from("return"), TokenType::Return);
    m.insert(String::from("super"), TokenType::Super);
    m.insert(String::from("this"), TokenType::This);
    m.insert(String::from("true"), TokenType::True);
    m.insert(String::from("var"), TokenType::Var);
    m.insert(String::from("while"), TokenType::While);
    Mutex::new(m)
});
