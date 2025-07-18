use crate::lexer::{LiteralValue, SourceLocation, Token, TokenType};
use crate::parser::ast::*;

#[derive(Clone)]
struct ParserState {
    current: usize,
}

pub struct Parser {
    tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> AstNode {
        let start_location = self.current_location();

        // Skip leading newlines
        self.skip_newlines();

        // Parse all module items
        let all_items = self.parse_many(|parser| {
            parser.skip_newlines();
            parser.parse_module_item()
        });

        // Filter out Empty nodes for the final program
        let items = all_items
            .into_iter()
            .filter(|item| !matches!(item, AstNode::Empty { .. }))
            .collect();

        // Ensure we consumed all input
        if !self.is_at_end() {
            panic!(
                "Parser did not consume all input. Stopped at position {} with token: {:?}",
                self.current,
                self.peek()
            );
        }

        AstNode::Program {
            items,
            location: start_location,
        }
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
        self.peek().location.clone()
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

    pub fn consume(&mut self, token_type: TokenType, _message: &str) -> Option<()> {
        if self.check(&token_type) {
            self.advance();
            Some(())
        } else {
            None
        }
    }

    pub fn consume_identifier(&mut self, _message: &str) -> Option<String> {
        if self.check(&TokenType::Identifier) {
            Some(self.advance().lexeme)
        } else {
            None
        }
    }

    pub fn skip_newlines(&mut self) {
        // Newlines are now handled as whitespace by the lexer, so this is a no-op
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

    // Backtracking support
    fn save_state(&self) -> ParserState {
        ParserState {
            current: self.current,
        }
    }

    fn restore_state(&mut self, state: ParserState) {
        self.current = state.current;
    }

    pub fn try_parse<T>(&mut self, f: impl FnOnce(&mut Self) -> Option<T>) -> Option<T> {
        let checkpoint = self.save_state();
        match f(self) {
            Some(result) => Some(result),
            None => {
                self.restore_state(checkpoint);
                None
            }
        }
    }

    pub fn parse_alternatives<T>(
        &mut self,
        alternatives: &[fn(&mut Self) -> Option<T>],
    ) -> Option<T> {
        for alternative in alternatives {
            if let Some(result) = self.try_parse(alternative) {
                return Some(result);
            }
        }
        None
    }

    // Helper for parsing optional visibility modifier
    pub fn try_parse_visibility(&mut self) -> Option<Visibility> {
        if self.accept_type(TokenType::Pub).is_some() {
            Some(Visibility::Public)
        } else {
            None
        }
    }

    // Helper to accept a specific token type and return the token if it matches
    pub fn accept_type(&mut self, token_type: TokenType) -> Option<Token> {
        if self.check(&token_type) {
            Some(self.advance())
        } else {
            None
        }
    }

    // Helper to accept any of multiple token types and return the token that matched
    pub fn accept_any_type(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types {
            if self.check(token_type) {
                return Some(self.advance());
            }
        }
        None
    }

    // Parser for comments - returns Empty node
    pub fn parse_comment(&mut self) -> Option<AstNode> {
        self.accept_type(TokenType::Comment)
            .map(|_token| AstNode::Empty {
                location: self.current_location(),
            })
    }

    // Parser for empty statements (lone semicolons) - returns Empty node
    pub fn parse_empty_statement(&mut self) -> Option<AstNode> {
        self.accept_type(TokenType::Semicolon)
            .map(|_token| AstNode::Empty {
                location: self.current_location(),
            })
    }

    /// Parse as many items as possible using the given parser function
    pub fn parse_many<T, F>(&mut self, parser_fn: F) -> Vec<T>
    where
        F: Fn(&mut Self) -> Option<T>,
    {
        let mut results = Vec::new();

        while !self.is_at_end() {
            let position_before = self.current;

            match parser_fn(self) {
                Some(item) => {
                    results.push(item);
                    // Continue parsing more items
                }
                None => {
                    // If no progress was made, we're done
                    if self.current == position_before {
                        break;
                    }
                    // If progress was made but parse failed, continue trying
                }
            }
        }

        results
    }
}
