use std::collections::HashMap;
use crate::lexer::token::{Token, TokenType, LiteralValue};

pub struct IdentifierScanner<'a> {
    input: &'a [char],
    position: &'a mut usize,
    _line: &'a mut usize,
    column: &'a mut usize,
    byte_offset: &'a mut usize,
    keywords: &'a HashMap<String, TokenType>,
}

impl<'a> IdentifierScanner<'a> {
    pub fn new(
        input: &'a [char],
        position: &'a mut usize,
        line: &'a mut usize,
        column: &'a mut usize,
        byte_offset: &'a mut usize,
        keywords: &'a HashMap<String, TokenType>,
    ) -> Self {
        IdentifierScanner {
            input,
            position,
            _line: line,
            column,
            byte_offset,
            keywords,
        }
    }

    pub fn scan_identifier(&mut self, _first_char: char, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = *self.position - 1;
        
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        
        // Check if it's a keyword
        if let Some(token_type) = self.keywords.get(&lexeme) {
            let value = match token_type {
                TokenType::True => Some(LiteralValue::Boolean(true)),
                TokenType::False => Some(LiteralValue::Boolean(false)),
                TokenType::Null => Some(LiteralValue::Null),
                _ => None,
            };
            
            Some(Token::new(*token_type, lexeme, start_line, start_column, start_offset, value))
        } else {
            Some(self.make_token(TokenType::Identifier, &lexeme, start_line, start_column, start_offset))
        }
    }

    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        
        let c = self.input[*self.position];
        *self.position += 1;
        *self.column += 1;
        *self.byte_offset += c.len_utf8();
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[*self.position]
        }
    }
    
    fn is_at_end(&self) -> bool {
        *self.position >= self.input.len()
    }
    
    fn make_token(&self, token_type: TokenType, lexeme: &str, line: usize, column: usize, offset: usize) -> Token {
        Token::new(token_type, lexeme.to_string(), line, column, offset, None)
    }
}