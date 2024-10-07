use std::{
    fmt,
    str::FromStr,
};
use crate::{TokenType, KEYWORDS};
use crate::token::{
    LiteralValue,
    Token,
    NumberLiteral,
    StringLiteral,
};

type Result<T> = std::result::Result<T, UnexpectedCharacterError>;

#[derive(Debug)]
enum UnexpectedCharacterError {
    UnknownCharacter(char),
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
    pub fn new(source: String) -> Self {
        Self {
            source,
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
                Err(e) => {
                    self.has_error = true;
                    eprintln!("[line {}] Error: {e}", self.line)
                }
            }
        }

        let eof_token = Token::new(
            TokenType::Eof,
            String::new(),
            None,
            self.line
        );
        self.tokens.push(eof_token);
    }

    // Returns true if the current character is the last one in self.source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance().expect("Expected character but found none");
        match c {
            // Single-character tokens
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

            // Operators can potentially have multiple characters
            '!' => {
                let t = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                return Ok(self.add_token(t));
            },
            '=' => {
                let t = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                return Ok(self.add_token(t));
            },
            '<' => {
                let t = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                return Ok(self.add_token(t));
            },
            '>' => {
                let t = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
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

            // '"' begins a string literal
            '"' => {
                match self.string() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            },

            // any digit begins a number literal
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                match self.number() {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e)
                }
            },

            // Newlines
            '\n' => Ok(self.line += 1),

            // Ignore whitespace
            ' ' | '\r' | '\t' => Ok(()),

            _ => {
                // We assume that every alphabetic character starts an identifier
                if c.is_alphabetic() {
                    match self.identifier() {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e)
                    }
                }
                // Everything else is an unkown character, raise an error 
                Err(UnexpectedCharacterError::UnknownCharacter(c))
            }
        }
    }

    // Returns the character at the upcoming position, if there is one
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current - 1)
    }

    /// Returns true if the next character is equal to `expected`
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() { return false; }
        let next_char = self.peek();
        if next_char != expected { return false; }

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

    fn peek_next(&self) -> char {
        if self.is_at_end() { return '\0'; }
        if let Some(c) = self.source.chars().nth(self.current + 1) {
            return c;
        } else {
            return '\0';
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, None);
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Box<dyn LiteralValue>>) {
        // Parse lexeme from source
        let text = String::from_str(&self.source[self.start..self.current])
            .expect("to be able to parse str to String");
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

        // If we reach the end of the file before finding the closing ", the literal is
        // unterminated.
        if self.is_at_end() {
            self.line -= lines;
            return Err(UnexpectedCharacterError::UnterminatedStringLiteral);
        }

        // Advance to the closing "
        self.advance();

        // Parse the string literals value from source
        let literal = StringLiteral {
            value: String::from_str(&self.source[self.start + 1..self.current - 1])
                .expect("to be able to parse str to String")
        };

        self.add_literal_token(
            TokenType::String, Some(Box::new(literal))
        );
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        // Keep parsing while the next character is numeric
        while self.peek().is_digit(10) { self.advance(); }

        // If the next character is a decimal point AND the character after that is numeric,
        // keep parsing
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_numeric() { self.advance(); }
        }

        // TODO: this feels dirty and I don't like it
        let value_str = &self.source[self.start..self.current];
        let literal = NumberLiteral {
            value: value_str.parse().expect("to be able to parse number literal value to number")
        };

        self.add_literal_token(
            TokenType::Number, Some(Box::new(literal))
        );
        Ok(())
    }

    // TODO: does this need a result? When would this error?
    fn identifier(&mut self) -> Result<()> {
        // Keep parsing while the next character is alphanumeric or an underscore _
        while self.peek().is_alphanumeric() || self.peek() == '_' { self.advance(); }
        let value_str = &self.source[self.start..self.current];
        if let Some(identifier_type) = KEYWORDS.lock().unwrap().get(value_str) {
            self.add_token(identifier_type);
            return Ok(());
        } else {
            self.add_token(TokenType::Identifier);
            return Ok(());
        }

    }

    pub fn print(&self) {
        for t in &self.tokens {
            println!("{}", t);
        }
    }
}
