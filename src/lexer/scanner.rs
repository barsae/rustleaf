use crate::lexer::comments::CommentScanner;
use crate::lexer::error::LexError;
use crate::lexer::identifiers::IdentifierScanner;
use crate::lexer::numbers::NumberScanner;
use crate::lexer::strings::StringScanner;
use crate::lexer::token::{Token, TokenType};
use std::collections::HashMap;

pub struct TokenScanner<'a> {
    input: &'a [char],
    position: &'a mut usize,
    line: &'a mut usize,
    column: &'a mut usize,
    byte_offset: &'a mut usize,
    keywords: &'a HashMap<String, TokenType>,
    errors: &'a mut Vec<LexError>,
}

impl<'a> TokenScanner<'a> {
    pub fn new(
        input: &'a [char],
        position: &'a mut usize,
        line: &'a mut usize,
        column: &'a mut usize,
        byte_offset: &'a mut usize,
        keywords: &'a HashMap<String, TokenType>,
        errors: &'a mut Vec<LexError>,
    ) -> Self {
        TokenScanner {
            input,
            position,
            line,
            column,
            byte_offset,
            keywords,
            errors,
        }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        let start_line = *self.line;
        let start_column = *self.column;
        let start_offset = *self.byte_offset;

        let c = self.advance();

        match c {
            // Whitespace (skip but track newlines)
            ' ' | '\t' => {
                self.skip_whitespace();
                None
            }
            '\n' => {
                let token = Token::new(
                    TokenType::Newline,
                    "\n".to_string(),
                    start_line,
                    start_column,
                    start_offset,
                    None,
                );
                *self.line += 1;
                *self.column = 1;
                Some(token)
            }
            '\r' => {
                // Handle CRLF
                if self.peek() == '\n' {
                    self.advance();
                }
                let token = Token::new(
                    TokenType::Newline,
                    if self.peek() == '\n' { "\r\n" } else { "\r" }.to_string(),
                    start_line,
                    start_column,
                    start_offset,
                    None,
                );
                *self.line += 1;
                *self.column = 1;
                Some(token)
            }

            // Single-character tokens
            '(' => Some(self.make_token(
                TokenType::LeftParen,
                "(",
                start_line,
                start_column,
                start_offset,
            )),
            ')' => Some(self.make_token(
                TokenType::RightParen,
                ")",
                start_line,
                start_column,
                start_offset,
            )),
            '{' => Some(self.make_token(
                TokenType::LeftBrace,
                "{",
                start_line,
                start_column,
                start_offset,
            )),
            '}' => Some(self.make_token(
                TokenType::RightBrace,
                "}",
                start_line,
                start_column,
                start_offset,
            )),
            '[' => Some(self.make_token(
                TokenType::LeftBracket,
                "[",
                start_line,
                start_column,
                start_offset,
            )),
            ']' => Some(self.make_token(
                TokenType::RightBracket,
                "]",
                start_line,
                start_column,
                start_offset,
            )),
            ',' => Some(self.make_token(
                TokenType::Comma,
                ",",
                start_line,
                start_column,
                start_offset,
            )),
            '.' if !self.peek().is_ascii_digit() => {
                Some(self.make_token(TokenType::Dot, ".", start_line, start_column, start_offset))
            }
            ';' => Some(self.make_token(
                TokenType::Semicolon,
                ";",
                start_line,
                start_column,
                start_offset,
            )),
            '~' => Some(self.make_token(
                TokenType::Tilde,
                "~",
                start_line,
                start_column,
                start_offset,
            )),
            '^' => Some(self.make_token(
                TokenType::Caret,
                "^",
                start_line,
                start_column,
                start_offset,
            )),

            // Multi-character tokens
            '+' => {
                if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::PlusEqual,
                        "+=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Plus,
                        "+",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '-' => {
                if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::MinusEqual,
                        "-=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else if self.match_char('>') {
                    Some(self.make_token(
                        TokenType::Arrow,
                        "->",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Minus,
                        "-",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '*' => {
                if self.match_char('*') {
                    Some(self.make_token(
                        TokenType::StarStar,
                        "**",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::StarEqual,
                        "*=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Star,
                        "*",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '/' => {
                if self.match_char('/') {
                    let mut comment_scanner = CommentScanner::new(
                        self.input,
                        self.position,
                        self.line,
                        self.column,
                        self.byte_offset,
                        self.errors,
                    );
                    comment_scanner.scan_single_line_comment(start_line, start_column, start_offset)
                } else if self.match_char('*') {
                    let mut comment_scanner = CommentScanner::new(
                        self.input,
                        self.position,
                        self.line,
                        self.column,
                        self.byte_offset,
                        self.errors,
                    );
                    comment_scanner.scan_multi_line_comment(start_line, start_column, start_offset)
                } else if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::SlashEqual,
                        "/=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Slash,
                        "/",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '%' => {
                if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::PercentEqual,
                        "%=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Percent,
                        "%",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::EqualEqual,
                        "==",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Equal,
                        "=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::BangEqual,
                        "!=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    self.error(
                        "Unexpected character '!' - use 'not' for logical negation".to_string(),
                        start_line,
                        start_column,
                        start_offset,
                    );
                    None
                }
            }
            '<' => {
                if self.match_char('<') {
                    Some(self.make_token(
                        TokenType::LessLess,
                        "<<",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::LessEqual,
                        "<=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Less,
                        "<",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '>' => {
                if self.match_char('>') {
                    Some(self.make_token(
                        TokenType::GreaterGreater,
                        ">>",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else if self.match_char('=') {
                    Some(self.make_token(
                        TokenType::GreaterEqual,
                        ">=",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Greater,
                        ">",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.error(
                        "Unexpected '&&' - use 'and' for logical AND".to_string(),
                        start_line,
                        start_column,
                        start_offset,
                    );
                    None
                } else {
                    Some(self.make_token(
                        TokenType::Ampersand,
                        "&",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.error(
                        "Unexpected '||' - use 'or' for logical OR".to_string(),
                        start_line,
                        start_column,
                        start_offset,
                    );
                    None
                } else {
                    Some(self.make_token(
                        TokenType::Pipe,
                        "|",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }
            ':' => {
                if self.match_char(':') {
                    Some(self.make_token(
                        TokenType::DoubleColon,
                        "::",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                } else {
                    Some(self.make_token(
                        TokenType::Colon,
                        ":",
                        start_line,
                        start_column,
                        start_offset,
                    ))
                }
            }

            // String literals
            '"' => {
                let mut string_scanner = StringScanner::new(
                    self.input,
                    self.position,
                    self.line,
                    self.column,
                    self.byte_offset,
                    self.errors,
                );
                string_scanner.scan_string(start_line, start_column, start_offset)
            }

            // Raw string literals
            'r' if self.peek() == '"' => {
                self.advance(); // consume the '"'
                let mut string_scanner = StringScanner::new(
                    self.input,
                    self.position,
                    self.line,
                    self.column,
                    self.byte_offset,
                    self.errors,
                );
                string_scanner.scan_raw_string(start_line, start_column, start_offset)
            }

            // Numbers
            c if c.is_ascii_digit() => {
                let mut number_scanner = NumberScanner::new(
                    self.input,
                    self.position,
                    self.line,
                    self.column,
                    self.byte_offset,
                    self.errors,
                );
                number_scanner.scan_number(c, start_line, start_column, start_offset)
            }

            // Handle leading dot for float literals
            '.' if self.peek().is_ascii_digit() => {
                let mut number_scanner = NumberScanner::new(
                    self.input,
                    self.position,
                    self.line,
                    self.column,
                    self.byte_offset,
                    self.errors,
                );
                number_scanner.scan_float_starting_with_dot(start_line, start_column, start_offset)
            }

            // Identifiers and keywords
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut identifier_scanner = IdentifierScanner::new(
                    self.input,
                    self.position,
                    self.line,
                    self.column,
                    self.byte_offset,
                    self.keywords,
                );
                identifier_scanner.scan_identifier(c, start_line, start_column, start_offset)
            }

            _ => {
                self.error(
                    format!("Unexpected character '{}'", c),
                    start_line,
                    start_column,
                    start_offset,
                );
                None
            }
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

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[*self.position] != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn is_at_end(&self) -> bool {
        *self.position >= self.input.len()
    }

    fn make_token(
        &self,
        token_type: TokenType,
        lexeme: &str,
        line: usize,
        column: usize,
        offset: usize,
    ) -> Token {
        Token::new(token_type, lexeme.to_string(), line, column, offset, None)
    }

    fn error(&mut self, message: String, line: usize, column: usize, offset: usize) {
        self.errors.push(LexError {
            message,
            line,
            column,
            byte_offset: offset,
        });
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
            self.advance();
        }
    }
}
