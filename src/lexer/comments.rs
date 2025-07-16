use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::LexError;

pub struct CommentScanner<'a> {
    input: &'a [char],
    position: &'a mut usize,
    line: &'a mut usize,
    column: &'a mut usize,
    byte_offset: &'a mut usize,
    errors: &'a mut Vec<LexError>,
}

impl<'a> CommentScanner<'a> {
    pub fn new(
        input: &'a [char],
        position: &'a mut usize,
        line: &'a mut usize,
        column: &'a mut usize,
        byte_offset: &'a mut usize,
        errors: &'a mut Vec<LexError>,
    ) -> Self {
        CommentScanner {
            input,
            position,
            line,
            column,
            byte_offset,
            errors,
        }
    }

    pub fn scan_single_line_comment(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = *self.position - 2; // Account for the "//" we already consumed
        
        // Check for documentation comment
        let is_doc_comment = self.peek() == '/';
        if is_doc_comment {
            self.advance(); // consume the third '/'
        }
        
        // Consume until end of line
        while self.peek() != '\n' && self.peek() != '\r' && !self.is_at_end() {
            self.advance();
        }
        
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        
        Some(Token::new(
            if is_doc_comment { TokenType::DocComment } else { TokenType::Comment },
            lexeme,
            start_line,
            start_column,
            start_offset,
            None,
        ))
    }
    
    pub fn scan_multi_line_comment(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = *self.position - 2; // Account for the "/*" we already consumed
        let mut nesting_level = 1;
        
        // Check for documentation comment
        let is_doc_comment = self.peek() == '*' && self.peek_next() != '/';
        if is_doc_comment {
            self.advance(); // consume the '*'
        }
        
        while nesting_level > 0 && !self.is_at_end() {
            let c = self.advance();
            match c {
                '/' if self.peek() == '*' => {
                    self.advance(); // consume '*'
                    nesting_level += 1;
                }
                '*' if self.peek() == '/' => {
                    self.advance(); // consume '/'
                    nesting_level -= 1;
                }
                '\n' => {
                    *self.line += 1;
                    *self.column = 1;
                }
                '\r' => {
                    if self.peek() == '\n' {
                        self.advance();
                    }
                    *self.line += 1;
                    *self.column = 1;
                }
                _ => {}
            }
        }
        
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        
        Some(Token::new(
            if is_doc_comment { TokenType::DocComment } else { TokenType::Comment },
            lexeme,
            start_line,
            start_column,
            start_offset,
            None,
        ))
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
    
    fn peek_next(&self) -> char {
        if *self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[*self.position + 1]
        }
    }
    
    fn is_at_end(&self) -> bool {
        *self.position >= self.input.len()
    }
    
    fn error(&mut self, message: String, line: usize, column: usize, offset: usize) {
        self.errors.push(LexError {
            message,
            line,
            column,
            byte_offset: offset,
        });
    }
}