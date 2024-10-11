use crate::expression::{Binary, Expression, Grouping, Literal, Unary, Variable};
use crate::token::{BooleanLiteral, NilLiteral, Token};
use crate::TokenType;
use crate::statement::{ExpressionStatement, PrintStatement, Statement, VarStatement};
use std::fmt;

type Result<T> = std::result::Result<T, ParserError>;

pub enum ParserError {
    UndisclosedDelimiter(Token),
    ExpectExpression(Token),
    UnexpectedToken(Token),
    NoSemicolon(Token),
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UndisclosedDelimiter(t) => match t.token_type {
                TokenType::Eof => write!(f, "at end: Undisclosed delimiter"),
                _ => write!(f, "at {}: Undisclosed delimiter", t.to_string()),
            },
            Self::ExpectExpression(t) => match t.token_type {
                TokenType::Eof => write!(f, "at end: Expected expression"),
                _ => write!(f, "at {}: Expected expression", t.to_string()),
            },
            Self::UnexpectedToken(t) => match t.token_type {
                TokenType::Eof => write!(f, "at end: Unexpected token"),
                _ => write!(f, "at {}: Unexpected token", t.to_string()),
            },
            Self::NoSemicolon(t) => match t.token_type {
                TokenType::Eof => write!(f, "at end: Missing semicolon"),
                _ => write!(f, "Missing semicolon after {}", t.to_string()),
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Left in for legacy tests
    pub fn parse_(&mut self) -> Result<Box<dyn Expression>> {
        match self.expression() {
            Ok(expr) => return Ok(expr),
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Statement>>> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => {
                    statements.push(stmt);
                },
                Err(e) => {
                    eprintln!("Error: {e}");
                    return Err(e);
                }
            }
        }
        Ok(statements)
    }

    fn statement(&mut self) -> Result<Box<dyn Statement>> {
        if self.match_tokens(vec![TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Box<dyn Statement>> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Box::new(PrintStatement::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Box<dyn Statement>> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon)?;
        Ok(Box::new(ExpressionStatement::new(expr)))
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

        while self.match_tokens(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
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
            return Ok(Box::new(Literal::new(Box::new(BooleanLiteral {
                value: false,
            }))));
        }
        if self.match_tokens(vec![TokenType::True]) {
            return Ok(Box::new(Literal::new(Box::new(BooleanLiteral {
                value: true,
            }))));
        }
        if self.match_tokens(vec![TokenType::Nil]) {
            return Ok(Box::new(Literal::new(Box::new(NilLiteral))));
        }
        if self.match_tokens(vec![TokenType::Number, TokenType::String]) {
            if let Some(l) = self.previous().literal {
                return Ok(Box::new(Literal::new(l)));
            }
            // return Err(ParserError::UnexpectedToken(self.peek()));
        }
        if self.match_tokens(vec![TokenType::Identifier]) {
            return Ok(Box::new(Variable::new(self.previous())));
        }
        if self.match_tokens(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            return match self.consume(TokenType::RightParen) {
                Ok(_) => Ok(Box::new(Grouping::new(expr))),
                Err(e) => Err(e),
            };
        }
        Err(ParserError::UnexpectedToken(self.peek()))
    }

    /// Looks for a closing delimiter and returns an Err if it doesn't find it
    fn consume(&mut self, token_type: TokenType) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        if token_type == TokenType::Semicolon {
            return Err(ParserError::NoSemicolon(self.peek()));
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
        if self.is_at_end() {
            return false;
        }
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
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => (),
            }

            self.advance();
        }
    }

    fn declaration(&mut self) -> Result<Box<dyn Statement>> {
        if self.match_tokens(vec![TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => return Ok(stmt),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        match self.statement() {
            Ok(stmt) => return Ok(stmt),
            Err(e) => {
                self.synchronize();
                return Err(e);
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Box<dyn Statement>> {
        match self.consume(TokenType::Identifier) {
            Ok(t) => {
                if self.match_tokens(vec![TokenType::Equal]) {
                    let initializer = self.expression()?;
                    match self.consume(TokenType::Semicolon) {
                        Ok(_) => (),
                        Err(e) => return Err(e)
                    }
                    return Ok(Box::new(VarStatement::new(t, Some(initializer))));
                }
                return Ok(Box::new(VarStatement::new(t, None)));
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
}
