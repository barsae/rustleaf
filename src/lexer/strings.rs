use crate::lexer::error::LexError;
use crate::lexer::token::{LiteralValue, Token, TokenType};

pub struct StringScanner<'a> {
    input: &'a [char],
    position: &'a mut usize,
    line: &'a mut usize,
    column: &'a mut usize,
    byte_offset: &'a mut usize,
    errors: &'a mut Vec<LexError>,
}

impl<'a> StringScanner<'a> {
    pub fn new(
        input: &'a [char],
        position: &'a mut usize,
        line: &'a mut usize,
        column: &'a mut usize,
        byte_offset: &'a mut usize,
        errors: &'a mut Vec<LexError>,
    ) -> Self {
        StringScanner {
            input,
            position,
            line,
            column,
            byte_offset,
            errors,
        }
    }

    pub fn scan_string(
        &mut self,
        start_line: usize,
        start_column: usize,
        start_offset: usize,
    ) -> Option<Token> {
        let start_pos = *self.position - 1; // Include the opening quote

        // Check for triple-quoted string
        if self.peek() == '"' && self.peek_next() == '"' {
            self.advance(); // second quote
            self.advance(); // third quote
            return self.scan_triple_quoted_string(
                start_line,
                start_column,
                start_offset,
                start_pos,
            );
        }

        let mut value = String::new();
        let mut escaped = false;
        let mut found_closing_quote = false;

        while !self.is_at_end() {
            let c = self.peek();

            if c == '"' && !escaped {
                self.advance(); // consume closing quote
                found_closing_quote = true;
                break;
            }

            if c == '\n' || c == '\r' {
                self.error(
                    "Unterminated string literal".to_string(),
                    start_line,
                    start_column,
                    start_offset,
                );
                return None;
            }

            let c = self.advance();

            if escaped {
                match c {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    '$' => value.push('$'),
                    '{' => value.push('{'),
                    '}' => value.push('}'),
                    'u' if self.peek() == '{' => {
                        self.advance(); // consume '{'
                        if let Some(unicode_char) = self.scan_unicode_escape() {
                            value.push(unicode_char);
                        } else {
                            self.error(
                                "Invalid Unicode escape sequence".to_string(),
                                *self.line,
                                *self.column,
                                *self.byte_offset,
                            );
                            return None;
                        }
                    }
                    _ => {
                        self.error(
                            format!("Invalid escape sequence '\\{}'", c),
                            *self.line,
                            *self.column,
                            *self.byte_offset,
                        );
                        return None;
                    }
                }
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else {
                value.push(c);
            }
        }

        if !found_closing_quote {
            self.error(
                "Unterminated string literal".to_string(),
                start_line,
                start_column,
                start_offset,
            );
            return None;
        }

        let lexeme: String = self.input[start_pos..*self.position].iter().collect();

        Some(self.make_literal_token(
            TokenType::StringLiteral,
            &lexeme,
            LiteralValue::String(value),
            start_line,
            start_column,
            start_offset,
        ))
    }

    pub fn scan_triple_quoted_string(
        &mut self,
        start_line: usize,
        start_column: usize,
        start_offset: usize,
        start_pos: usize,
    ) -> Option<Token> {
        let mut value = String::new();

        while !self.is_at_end() {
            // Check if we have three consecutive quotes at current position
            if *self.position + 2 < self.input.len()
                && self.input[*self.position] == '"'
                && self.input[*self.position + 1] == '"'
                && self.input[*self.position + 2] == '"'
            {
                // Consume the closing triple quotes
                self.advance(); // first quote
                self.advance(); // second quote
                self.advance(); // third quote

                // Successfully parsed triple-quoted string
                let lexeme: String = self.input[start_pos..*self.position].iter().collect();
                return Some(self.make_literal_token(
                    TokenType::StringLiteral,
                    &lexeme,
                    LiteralValue::String(value),
                    start_line,
                    start_column,
                    start_offset,
                ));
            }

            // Continue parsing content
            let c = self.advance();
            if c == '\n' {
                *self.line += 1;
                *self.column = 1;
            } else if c == '\r' {
                if self.peek() == '\n' {
                    value.push(c);
                    value.push(self.advance());
                } else {
                    value.push(c);
                }
                *self.line += 1;
                *self.column = 1;
                continue;
            }
            value.push(c);
        }

        // If we reach here, we hit end of input without finding closing quotes
        self.error(
            "Unterminated triple-quoted string".to_string(),
            start_line,
            start_column,
            start_offset,
        );
        None
    }

    pub fn scan_raw_string(
        &mut self,
        start_line: usize,
        start_column: usize,
        start_offset: usize,
    ) -> Option<Token> {
        let start_pos = *self.position - 2; // Include 'r"'
        let mut value = String::new();
        let mut found_closing_quote = false;

        while !self.is_at_end() {
            let c = self.peek();

            if c == '"' {
                self.advance(); // consume closing quote
                found_closing_quote = true;
                break;
            }

            if c == '\n' || c == '\r' {
                self.error(
                    "Unterminated raw string literal".to_string(),
                    start_line,
                    start_column,
                    start_offset,
                );
                return None;
            }

            value.push(self.advance());
        }

        if !found_closing_quote {
            self.error(
                "Unterminated raw string literal".to_string(),
                start_line,
                start_column,
                start_offset,
            );
            return None;
        }

        let lexeme: String = self.input[start_pos..*self.position].iter().collect();

        Some(self.make_literal_token(
            TokenType::RawStringLiteral,
            &lexeme,
            LiteralValue::String(value),
            start_line,
            start_column,
            start_offset,
        ))
    }

    fn scan_unicode_escape(&mut self) -> Option<char> {
        let mut hex_digits = String::new();

        while !self.is_at_end() && hex_digits.len() < 6 {
            let c = self.peek();
            if c == '}' {
                self.advance(); // consume '}'
                break;
            }
            if c.is_ascii_hexdigit() {
                hex_digits.push(self.advance());
            } else {
                return None; // Invalid hex digit
            }
        }

        if hex_digits.is_empty() || hex_digits.len() > 6 {
            return None;
        }

        if let Ok(code_point) = u32::from_str_radix(&hex_digits, 16) {
            char::from_u32(code_point)
        } else {
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

    fn make_literal_token(
        &self,
        token_type: TokenType,
        lexeme: &str,
        value: LiteralValue,
        line: usize,
        column: usize,
        offset: usize,
    ) -> Token {
        Token::new(
            token_type,
            lexeme.to_string(),
            line,
            column,
            offset,
            Some(value),
        )
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
