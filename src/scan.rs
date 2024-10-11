use unicode_segmentation::UnicodeSegmentation;
use regex::Regex;
use crate::token::{LiteralValue, NumberLiteral, StringLiteral, Token};
use crate::{report, TokenType, KEYWORDS};
use std::fmt;

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
            }
            UnexpectedCharacterError::UnterminatedStringLiteral => {
                write!(f, "Unterminated string.")
            }
        }
    }
}

pub struct Scanner {
    graphemes: Vec<String>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    pub has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let graphemes = source
            .graphemes(true)
            .map(|g| g.to_string())
            .collect::<Vec<String>>();
        Self {
            graphemes,
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
                    report(self.line, "", &e.to_string());
                }
            }
        }

        let eof_token = Token::new(TokenType::Eof, String::new(), None, self.line);
        self.tokens.push(eof_token);
    }

    /// Returns true if the current character is the last one in self.source
    fn is_at_end(&self) -> bool {
        // let graphemes = self.source.graphemes(true).collect::<Vec<&str>>();
        self.current >= self.graphemes.len()
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance().expect("Expected character but found none");
        match c {
            // Single-character tokens
            "(" => Ok(self.add_token(TokenType::LeftParen)),
            ")" => Ok(self.add_token(TokenType::RightParen)),
            "{" => Ok(self.add_token(TokenType::LeftBrace)),
            "}" => Ok(self.add_token(TokenType::RightBrace)),
            "," => Ok(self.add_token(TokenType::Comma)),
            "." => Ok(self.add_token(TokenType::Dot)),
            "-" => Ok(self.add_token(TokenType::Minus)),
            "+" => Ok(self.add_token(TokenType::Plus)),
            ";" => Ok(self.add_token(TokenType::Semicolon)),
            "*" => Ok(self.add_token(TokenType::Star)),

            // Operators can potentially have multiple characters
            "!" => {
                let t = if self.match_next("=") {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                return Ok(self.add_token(t));
            }
            "=" => {
                let t = if self.match_next("=") {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                return Ok(self.add_token(t));
            }
            "<" => {
                let t = if self.match_next("=") {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                return Ok(self.add_token(t));
            }
            ">" => {
                let t = if self.match_next("=") {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                return Ok(self.add_token(t));
            }
            "/" => {
                return if self.match_next("/") {
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenType::Slash))
                };
            }

            // '"' begins a string literal
            "\"" => match self.string() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },

            // any digit begins a number literal
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => match self.number() {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            },

            // Newlines
            "\n" => Ok(self.line += 1),

            // Ignore whitespace
            " " | "\r" | "\t" => Ok(()),

            _ => {
                // We assume that every alphabetic character starts an identifier
                if is_alphabetic(c) || c == "_" {
                    return match self.identifier() {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e),
                    };
                }
                // Everything else is an unkown character, raise an error
                Err(UnexpectedCharacterError::UnknownCharacter(c.to_owned()))
            }
        }
    }

    /// Advances the pointer one position, then
    /// returns the new current character, if there is one
    fn advance(&mut self) -> Option<&str> {
        self.current += 1;
        if self.current > self.graphemes.len() {
            return None;
        }
        Some(&self.graphemes[self.current - 1])
    }

    /// Returns true if the next character is equal to `expected`
    fn match_next(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        let next_char = self.peek();
        if next_char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    /// Returns the character at the upcoming position, if there is one
    fn peek(&self) -> &str {
        if self.is_at_end() {
            return "\0";
        }
        &self.graphemes[self.current]
    }

    /// Returns the character two positions ahead, if there is one
    fn peek_next(&self) -> &str {
        if self.is_at_end() {
            return "\0";
        }
        if self.current < self.graphemes.len() {
            return &self.graphemes[self.current + 1];
        }
        return "\0";
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, None);
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Box<dyn LiteralValue>>) {
        // Parse lexeme from source
        let text = self.graphemes[self.start..self.current].concat();
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn string(&mut self) -> Result<()> {
        let mut lines: usize = 0;

        // While we haven't reached the closing " or the end of the line, advance
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                lines += 1;
                self.line += lines;
            }
            self.advance();
        }

        // If we reach the end of the file before finding the closing ",
        // the literal is unterminated.
        if self.is_at_end() {
            self.line -= lines;
            return Err(UnexpectedCharacterError::UnterminatedStringLiteral);
        }

        // Advance to the closing "
        self.advance();

        // Parse the string literals value from source
        let literal = StringLiteral {
            value: self.graphemes[self.start + 1..self.current - 1].concat()
        };

        self.add_literal_token(TokenType::String, Some(Box::new(literal)));
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        // Keep parsing while the next character is numeric
        while is_digit(self.peek()) {
            self.advance();
        }

        // If the next character is a decimal point AND the character after that is numeric,
        // keep parsing
        if self.peek() == "." && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let value_str = self.graphemes[self.start..self.current].concat();
        let literal = NumberLiteral {
            value: value_str
                .parse()
                .expect("to be able to parse number literal value to number"),
        };

        self.add_literal_token(TokenType::Number, Some(Box::new(literal)));
        Ok(())
    }

    fn identifier(&mut self) -> Result<()> {
        // Keep parsing while the next character is alphanumeric or an underscore _
        while is_alphabetic(self.peek()) || is_digit(self.peek()) || self.peek() == "_" {
            self.advance();
        }
        let value_str = self.graphemes[self.start..self.current].concat();
        if let Some(identifier_type) = KEYWORDS.lock().unwrap().get(value_str.as_str()) {
            self.add_token(identifier_type.clone());
            return Ok(());
        } else {
            self.add_token(TokenType::Identifier);
            return Ok(());
        }
    }
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for t in &self.tokens {
            write!(f, "{}\n", t)?;
        }
        Ok(())
    }
}

fn is_digit(grapheme: &str) -> bool {
    match grapheme {
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => return true,
        _ => return false
    }
}

fn is_alphabetic(grapheme: &str) -> bool {
    let re = Regex::new(r"[a-zA-Z]").unwrap();
    re.is_match(grapheme)
}
