/// Parser implementation for RustLeaf
use crate::core::*;
use crate::lexer::{Token, TokenType};
use anyhow::{anyhow, Result};

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    pub(crate) current: usize,
}

impl Parser {
    /// Parse source code string directly into an AST
    pub fn parse_str(source: &str) -> Result<Program> {
        let tokens = crate::lexer::Lexer::tokenize(source)?;
        Self::parse(tokens)
    }

    /// Parse tokens into an AST
    pub fn parse(tokens: Vec<Token>) -> Result<Program> {
        let mut parser = Self::new(tokens);
        parser.parse_program()
    }

    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program(statements))
    }

    // ===== Helper Functions =====

    pub fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    pub fn accept_token(&mut self, token_type: TokenType) -> Option<Token> {
        if self.check(token_type) {
            Some(self.advance().clone())
        } else {
            None
        }
    }

    pub fn accept(&mut self, token_type: TokenType) -> bool {
        self.accept_token(token_type).is_some()
    }

    pub fn expect(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if let Some(token) = self.accept_token(token_type) {
            Ok(token)
        } else {
            Err(anyhow!("{}: expected {:?}, found {:?}", message, token_type, self.peek().token_type))
        }
    }
}