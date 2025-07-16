use crate::lexer::token::{Token, TokenType, LiteralValue};
use crate::lexer::error::LexError;

pub struct NumberScanner<'a> {
    input: &'a [char],
    position: &'a mut usize,
    _line: &'a mut usize,
    column: &'a mut usize,
    byte_offset: &'a mut usize,
    errors: &'a mut Vec<LexError>,
}

impl<'a> NumberScanner<'a> {
    pub fn new(
        input: &'a [char],
        position: &'a mut usize,
        line: &'a mut usize,
        column: &'a mut usize,
        byte_offset: &'a mut usize,
        errors: &'a mut Vec<LexError>,
    ) -> Self {
        NumberScanner {
            input,
            position,
            _line: line,
            column,
            byte_offset,
            errors,
        }
    }

    pub fn scan_number(&mut self, first_digit: char, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = *self.position - 1;
        
        // Handle different number bases
        if first_digit == '0' && !self.is_at_end() {
            match self.peek() {
                'x' | 'X' => {
                    self.advance(); // consume 'x'
                    return self.scan_hex_number(start_line, start_column, start_offset, start_pos);
                }
                'o' | 'O' => {
                    self.advance(); // consume 'o'
                    return self.scan_octal_number(start_line, start_column, start_offset, start_pos);
                }
                'b' | 'B' => {
                    self.advance(); // consume 'b'
                    return self.scan_binary_number(start_line, start_column, start_offset, start_pos);
                }
                '0'..='9' => {
                    self.error("Leading zeros not allowed in decimal literals".to_string(), start_line, start_column, start_offset);
                    return None;
                }
                _ => {} // Fall through to decimal parsing
            }
        }
        
        // Decimal number (integer or float)
        self.scan_decimal_digits();
        
        // Check for float
        if self.peek() == '.' {
            // Look ahead to determine if this should be a float or separate dot token
            // It's a float if:
            // 1. There are digits after the dot (e.g., 42.5)
            // 2. There's nothing after the dot or it's followed by whitespace/operators (e.g., 42.)
            // 3. It's followed by scientific notation (e.g., 42.e10)
            let next_char = self.peek_next();
            if next_char.is_ascii_digit() || 
               next_char == 'e' || next_char == 'E' ||
               next_char.is_whitespace() || 
               matches!(next_char, '\0' | ')' | ']' | '}' | ',' | ';' | '+' | '-' | '*' | '/' | '%' | '=' | '<' | '>' | '&' | '|' | '^' | '~') {
                self.advance(); // consume '.'
                if self.peek().is_ascii_digit() {
                    self.scan_decimal_digits();
                }
                
                // Check for scientific notation after decimal part
                if self.peek() == 'e' || self.peek() == 'E' {
                    return self.scan_scientific_notation(start_line, start_column, start_offset, start_pos);
                }
                
                return self.finalize_float_token(start_line, start_column, start_offset, start_pos);
            }
        }
        
        // Check for scientific notation
        if self.peek() == 'e' || self.peek() == 'E' {
            return self.scan_scientific_notation(start_line, start_column, start_offset, start_pos);
        }
        
        // Integer literal
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        
        if let Ok(value) = lexeme.replace('_', "").parse::<i64>() {
            Some(self.make_literal_token(
                TokenType::IntegerLiteral,
                &lexeme,
                LiteralValue::Integer(value),
                start_line,
                start_column,
                start_offset,
            ))
        } else {
            self.error("Integer literal out of range".to_string(), start_line, start_column, start_offset);
            None
        }
    }
    
    pub fn scan_float_starting_with_dot(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = *self.position - 1; // Include the '.'
        self.scan_decimal_digits();
        
        // Check for scientific notation
        if self.peek() == 'e' || self.peek() == 'E' {
            return self.scan_scientific_notation(start_line, start_column, start_offset, start_pos);
        }
        
        self.finalize_float_token(start_line, start_column, start_offset, start_pos)
    }
    
    fn scan_scientific_notation(&mut self, start_line: usize, start_column: usize, start_offset: usize, start_pos: usize) -> Option<Token> {
        self.advance(); // consume 'e' or 'E'
        
        // Optional sign
        if self.peek() == '+' || self.peek() == '-' {
            self.advance();
        }
        
        if !self.peek().is_ascii_digit() {
            self.error("Invalid scientific notation".to_string(), start_line, start_column, start_offset);
            return None;
        }
        
        self.scan_decimal_digits();
        self.finalize_float_token(start_line, start_column, start_offset, start_pos)
    }
    
    fn finalize_float_token(&self, start_line: usize, start_column: usize, start_offset: usize, start_pos: usize) -> Option<Token> {
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        
        if let Ok(value) = lexeme.replace('_', "").parse::<f64>() {
            Some(self.make_literal_token(
                TokenType::FloatLiteral,
                &lexeme,
                LiteralValue::Float(value),
                start_line,
                start_column,
                start_offset,
            ))
        } else {
            None
        }
    }
    
    fn scan_decimal_digits(&mut self) {
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '_' && self.peek_next().is_ascii_digit() {
                self.advance(); // consume underscore
            } else {
                break;
            }
        }
    }
    
    fn scan_hex_number(&mut self, start_line: usize, start_column: usize, start_offset: usize, start_pos: usize) -> Option<Token> {
        if !self.peek().is_ascii_hexdigit() {
            self.error("Invalid hexadecimal literal".to_string(), start_line, start_column, start_offset);
            return None;
        }
        
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_ascii_hexdigit() {
                self.advance();
            } else if c == '_' && self.peek_next().is_ascii_hexdigit() {
                self.advance();
            } else {
                break;
            }
        }
        
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        let hex_part = &lexeme[2..].replace('_', ""); // Remove "0x" prefix and underscores
        
        if let Ok(value) = i64::from_str_radix(hex_part, 16) {
            Some(self.make_literal_token(
                TokenType::IntegerLiteral,
                &lexeme,
                LiteralValue::Integer(value),
                start_line,
                start_column,
                start_offset,
            ))
        } else {
            self.error("Hexadecimal literal out of range".to_string(), start_line, start_column, start_offset);
            None
        }
    }
    
    fn scan_octal_number(&mut self, start_line: usize, start_column: usize, start_offset: usize, start_pos: usize) -> Option<Token> {
        if !matches!(self.peek(), '0'..='7') {
            self.error("Invalid octal literal".to_string(), start_line, start_column, start_offset);
            return None;
        }
        
        while !self.is_at_end() {
            let c = self.peek();
            if matches!(c, '0'..='7') {
                self.advance();
            } else if c == '_' && matches!(self.peek_next(), '0'..='7') {
                self.advance();
            } else {
                break;
            }
        }
        
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        let octal_part = &lexeme[2..].replace('_', ""); // Remove "0o" prefix and underscores
        
        if let Ok(value) = i64::from_str_radix(octal_part, 8) {
            Some(self.make_literal_token(
                TokenType::IntegerLiteral,
                &lexeme,
                LiteralValue::Integer(value),
                start_line,
                start_column,
                start_offset,
            ))
        } else {
            self.error("Octal literal out of range".to_string(), start_line, start_column, start_offset);
            None
        }
    }
    
    fn scan_binary_number(&mut self, start_line: usize, start_column: usize, start_offset: usize, start_pos: usize) -> Option<Token> {
        if !matches!(self.peek(), '0' | '1') {
            self.error("Invalid binary literal".to_string(), start_line, start_column, start_offset);
            return None;
        }
        
        while !self.is_at_end() {
            let c = self.peek();
            if matches!(c, '0' | '1') {
                self.advance();
            } else if c == '_' && matches!(self.peek_next(), '0' | '1') {
                self.advance();
            } else {
                break;
            }
        }
        
        let lexeme: String = self.input[start_pos..*self.position].iter().collect();
        let binary_part = &lexeme[2..].replace('_', ""); // Remove "0b" prefix and underscores
        
        if let Ok(value) = i64::from_str_radix(binary_part, 2) {
            Some(self.make_literal_token(
                TokenType::IntegerLiteral,
                &lexeme,
                LiteralValue::Integer(value),
                start_line,
                start_column,
                start_offset,
            ))
        } else {
            self.error("Binary literal out of range".to_string(), start_line, start_column, start_offset);
            None
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
    
    fn make_literal_token(&self, token_type: TokenType, lexeme: &str, value: LiteralValue, line: usize, column: usize, offset: usize) -> Token {
        Token::new(token_type, lexeme.to_string(), line, column, offset, Some(value))
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