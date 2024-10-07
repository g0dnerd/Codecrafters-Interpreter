use std::str::FromStr;

use crate::token::{ Token, TokenType };

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: String::from_str(source).expect("to be able to parse input to String"),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
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

    fn scan_token(&mut self) {
        let c = self.advance().expect("Expected character but found none");
        eprintln!("Parsing character {}", c);
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '/' => self.add_token(TokenType::Slash),
            '*' => self.add_token(TokenType::Star),
            ' ' | '\r' | '\n' | '\t' => (),
            _ => eprintln!("Unexpected character {}", c)
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
