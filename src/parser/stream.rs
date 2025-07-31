use crate::lexer::{Token, TokenType};
use anyhow::Result;

pub struct TokenStream {
    tokens: Vec<Token>,
    current: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, current: 0 }
    }

    pub fn position(&self) -> usize {
        self.current
    }

    pub fn current_token_info(&self) -> String {
        let token = self.peek();
        match &token.text {
            Some(text) => format!("{:?}({})", token.token_type, text),
            None => format!("{:?}", token.token_type),
        }
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    pub fn peek_type(&self) -> TokenType {
        self.peek().token_type
    }

    pub fn try_parse<T, F>(&mut self, f: F) -> Result<Option<T>>
    where
        F: FnOnce(&mut TokenStream) -> Result<Option<T>>,
    {
        let checkpoint = self.current;
        
        match f(self) {
            Ok(Some(result)) => {
                // Success - just return
                Ok(Some(result))
            }
            Ok(None) => {
                // This alternative didn't match - restore checkpoint, return Ok(None)
                self.current = checkpoint;
                Ok(None)
            }
            Err(err) => {
                // Hard error - restore checkpoint, return Err
                self.current = checkpoint;
                Err(err)
            }
        }
    }
    
    pub fn parse<T, F>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut TokenStream) -> Result<T>,
    {
        f(self)
    }

    #[allow(dead_code)]
    pub fn accept_lexeme(&mut self, lexeme: &str) -> Result<Option<Token>> {
        let token = self.peek();
        if token.text.as_deref() == Some(lexeme) {
            Ok(Some(self.advance().clone()))
        } else {
            Ok(None)
        }
    }

    pub fn accept_type(&mut self, token_type: TokenType) -> Result<Option<Token>> {
        let current = self.peek();
        if current.token_type == token_type {
            let token = self.advance().clone();
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
    
    #[allow(dead_code)]
    pub fn expect_lexeme(&mut self, lexeme: &str) -> Result<Token> {
        self.accept_lexeme(lexeme)?
            .ok_or_else(|| anyhow::anyhow!(
                "Expected '{}', found {:?}",
                lexeme,
                self.peek().token_type
            ))
    }
    
    pub fn expect_type(&mut self, token_type: TokenType) -> Result<Token> {
        match self.accept_type(token_type)? {
            Some(token) => {
                Ok(token)
            }
            None => {
                let err = anyhow::anyhow!(
                    "Expected {:?}, found {:?}",
                    token_type,
                    self.peek().token_type
                );
                Err(err)
            }
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }
}