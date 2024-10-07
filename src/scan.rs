use std::{
    fmt,
    str::FromStr
};

use crate::token::{ Token, TokenType };

type Result<T> = std::result::Result<T, UnexpectedCharacterError>;

#[derive(Debug, Clone)]
struct UnexpectedCharacterError {
    character: &char,
}

impl fmt::Display for UnexpectedCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unexpected character: {}", self.character)
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from_str(source).expect("to be able to parse input to String"),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            has_error: false,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => (),
                Err(e) => eprintln!("[line {}] Error: {e:?}: ", e)
            }
        }

        let initial_token = Token::new(
            TokenType::Eof,
            String::new(),
            None,
            self.line
        );
        self.tokens.push(initial_token);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance().expect("Expected character but found none");
        eprintln!("Parsing character {}", c);
        match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            ';' => Ok(self.add_token(TokenType::Semicolon)),
            '/' => Ok(self.add_token(TokenType::Slash)),
            '*' => Ok(self.add_token(TokenType::Star)),
            ' ' | '\r' | '\n' | '\t' => Ok(()),
            _ => {
                self.has_error = true;
                UnexpectedCharacterError { character: &c }
            },
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        let token = Token::new(token_type, String::from_str(text).expect("to be able to parse input to String"), None, self.line);
        eprintln!("{:?}", token);
        self.tokens.push(token);
    }

    pub fn print(&self) {
        for t in &self.tokens {
            println!("{}", t);
        }
    }
}
