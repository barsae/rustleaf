use crate::lexer::{LiteralValue, Token, TokenType};
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pub current: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            errors: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Result<AstNode, Vec<ParseError>> {
        let start_location = self.current_location();
        let mut items = Vec::new();

        // Skip leading newlines
        self.skip_newlines();

        while !self.is_at_end() {
            let position_before = self.current;

            if let Some(item) = self.parse_module_item() {
                items.push(item);
            }

            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!(
                    "Parser stuck: no progress made at position {}",
                    self.current
                );
            }

            self.skip_newlines();
        }

        if !self.errors.is_empty() {
            return Err(self.errors.clone());
        }

        Ok(AstNode::Program {
            items,
            location: start_location,
        })
    }

    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }

    // Core utility methods
    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn current_location(&self) -> SourceLocation {
        SourceLocation::from_token(self.peek())
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    pub fn check_ahead(&self, offset: usize, token_type: &TokenType) -> bool {
        if self.current + offset >= self.tokens.len() {
            false
        } else {
            &self.tokens[self.current + offset].token_type == token_type
        }
    }

    pub fn match_token(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> Option<()> {
        if self.check(&token_type) {
            self.advance();
            Some(())
        } else {
            self.error(message);
            None
        }
    }

    pub fn consume_identifier(&mut self, message: &str) -> Option<String> {
        if self.check(&TokenType::Identifier) {
            Some(self.advance().lexeme)
        } else {
            self.error(message);
            None
        }
    }

    pub fn skip_newlines(&mut self) {
        while self.match_token(&TokenType::Newline) {
            // Skip newlines
        }
    }

    pub fn match_string_literal(&mut self) -> Option<String> {
        if self.check(&TokenType::StringLiteral) {
            let token = self.advance();
            if let Some(LiteralValue::String(s)) = token.value {
                Some(s)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn error(&mut self, message: &str) {
        let location = self.current_location();
        self.errors.push(ParseError {
            message: message.to_string(),
            location,
        });
    }
}
