use std::fmt;
use crate::TokenType;
use crate::token::{
    BooleanLiteral,
    NilLiteral,
    Token
};
use crate::expression::{
    Binary,
    Expression,
    Grouping,
    Literal,
    Unary
};

type Result<T> = std::result::Result<T, ParserError>;

pub enum ParserError {
    UndisclosedDelimiter(Token),
    ExpectExpression(Token),
    UnexpectedToken(),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UndisclosedDelimiter(t) | Self::ExpectExpression(t) => {
                match t.token_type {
                    TokenType::Eof => write!(f, "at end"),
                    _ => write!(f, "at ' {}", t.lexeme)
                }
            },
            _ => write!(f, "PRIMARY CRASH")
        }
    }
}

// TODO: do I want to integrate this with scan::Scanner?
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expression>> {
        // TODO: aaaaah
        Ok(self.expression()?)
    }

    fn expression(&mut self) -> Result<Box<dyn Expression>> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<dyn Expression>> {
        let mut expr = self.comparison()?;

        while self.match_tokens(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expression>> {
        let mut expr = self.term()?;

        while self.match_tokens(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<dyn Expression>> {
        let mut expr = self.factor()?;

        while self.match_tokens(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<dyn Expression>> {
        let mut expr = self.unary()?;

        while self.match_tokens(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<dyn Expression>> {
        if self.match_tokens(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Box::new(Unary::new(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<dyn Expression>> {
        if self.match_tokens(vec![TokenType::False]) {
            return Ok(Box::new(Literal::new(Some(Box::new(BooleanLiteral{ value: false })))));
        }
        if self.match_tokens(vec![TokenType::True]) {
            return Ok(Box::new(Literal::new(Some(Box::new(BooleanLiteral{ value: true })))));
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Ok(Box::new(Literal::new(Some(Box::new(NilLiteral)))));
        }
        if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            return Ok(Box::new(Literal::new(self.previous().literal)));
        }
        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            return match self.consume(TokenType::RightParen) {
                Ok(_) => Ok(Box::new(Grouping::new(expr))),
                Err(e) => Err(e)
            }
        }
        Err(ParserError::UnexpectedToken())
    }

    /// Looks for a closing delimiter and returns an Err if it doesn't find it
    fn consume(&mut self, token_type: TokenType) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParserError::UndisclosedDelimiter(self.peek()))
    }

    fn match_tokens(&mut self, types: Vec<TokenType>) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() { return false; }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class |
                TokenType::Fun |
                TokenType::Var |
                TokenType::For |
                TokenType::If |
                TokenType::While |
                TokenType::Print |
                TokenType::Return => return,
                _ => ()
            }

            self.advance();
        }
    }
}
