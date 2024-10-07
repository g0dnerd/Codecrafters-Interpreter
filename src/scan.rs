use std::{
    fmt,
    str::FromStr,
};

use crate::token::{ Token, TokenType };

type Result<T> = std::result::Result<T, UnexpectedCharacterError>;

#[derive(Debug)]
enum UnexpectedCharacterError {
    UnknownCharacter(String),
    UnterminatedStringLiteral,
}

impl fmt::Display for UnexpectedCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnexpectedCharacterError::UnknownCharacter(c) => {
                write!(f, "Unexpected character: {}", &c)
            },
            UnexpectedCharacterError::UnterminatedStringLiteral => {
                write!(f, "Unterminated string.")
            }
        }
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub has_error: bool,
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
                Err(e) => eprintln!("[line {}] Error: {e}", self.line)
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
            '*' => Ok(self.add_token(TokenType::Star)),
            '!' => {
                let t = if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang };
                return Ok(self.add_token(t));
            },
            '=' => {
                let t = if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal };
                return Ok(self.add_token(t));
            },
            '<' => {
                let t = if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less };
                return Ok(self.add_token(t));
            },
            '>' => {
                let t = if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                return Ok(self.add_token(t));
            },
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                    return Ok(())
                } else {
                    return Ok(self.add_token(TokenType::Slash));
                }
            },
            '"' => {
                match self.string() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            },
            '\n' => Ok(self.line += 1),
            ' ' | '\r' | '\t' => Ok(()),
            _ => {
                self.has_error = true;
                Err(UnexpectedCharacterError::UnknownCharacter(String::from(c)))
            },
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        if self.source.chars().nth(self.current).expect("no character while matching operator") != expected { return false; }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        if let Some(c) = self.source.chars().nth(self.current) {
            return c;
        } else {
            return '\0';
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, None);
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = String::from_str(&self.source[self.start..self.current]).expect("to be able to parse str to String");
        self.tokens.push(
            Token::new(token_type, text, literal, self.line)
        );
    }

    fn string(&mut self) -> Result<()> {
        let mut lines: usize = 0;

        // While we haven't reached the closing " or the end of the line, advance
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                lines += 1;
                self.line += lines; }
            self.advance();
        }

        if self.is_at_end() {
            self.line -= lines;
            return Err(UnexpectedCharacterError::UnterminatedStringLiteral);
        }

        // Advance to the closing "
        self.advance();

        let value = String::from_str(&self.source[self.start + 1..self.current - 1]).expect("to be able to parse str to String");
        self.add_literal_token(TokenType::String, Some(value));
        Ok(())
    }

    pub fn print(&self) {
        for t in &self.tokens {
            println!("{}", t);
        }
    }
}
