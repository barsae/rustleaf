use crate::lexer::{Token, TokenType};
use anyhow::Result;
use crate::{trace_enter, trace_exit, trace_fail, trace_backtrack, trace_token};

pub struct TokenStream {
    tokens: Vec<Token>,
    current: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { tokens, current: 0 }
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    pub fn peek_type(&self) -> TokenType {
        self.peek().token_type
    }

    pub fn try_parse<T, F>(&mut self, f: F) -> Result<Option<T>>
    where
        F: FnOnce(&mut TokenStream) -> Result<T>,
    {
        trace_enter!("try_parse", "checkpoint: {}", self.current);
        let checkpoint = self.current;
        
        match f(self) {
            Ok(result) => {
                trace_exit!("try_parse"); // Don't log full result to avoid spam
                Ok(Some(result))
            }
            Err(_) => {
                trace_backtrack!("try_parse");
                self.current = checkpoint;
                trace_exit!("try_parse");
                Ok(None)
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
        trace_enter!("accept_type", "looking for: {:?}", token_type);
        let current = self.peek();
        if current.token_type == token_type {
            trace_token!("accept", current);
            let token = self.advance().clone();
            trace_exit!("accept_type");
            Ok(Some(token))
        } else {
            trace_token!("reject", current);
            trace_exit!("accept_type");
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
        trace_enter!("expect_type", "expecting: {:?}", token_type);
        match self.accept_type(token_type)? {
            Some(token) => {
                trace_exit!("expect_type");
                Ok(token)
            }
            None => {
                let err = anyhow::anyhow!(
                    "Expected {:?}, found {:?}",
                    token_type,
                    self.peek().token_type
                );
                trace_fail!("expect_type", &err);
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