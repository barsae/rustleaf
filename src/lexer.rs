use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords
    And, Break, Case, Catch, Class, Continue, Else, False, Finally, Fn, For,
    From, If, In, Is, Match, Not, Null, Of, Or, Pub, Raise, Require, Return,
    Self_, Static, Super, True, Try, Use, Var, While, With,
    
    // Identifiers and literals
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    RawStringLiteral,
    BooleanLiteral,
    NullLiteral,
    
    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    StarStar,       // **
    Equal,          // =
    PlusEqual,      // +=
    MinusEqual,     // -=
    StarEqual,      // *=
    SlashEqual,     // /=
    PercentEqual,   // %=
    EqualEqual,     // ==
    BangEqual,      // !=
    Less,           // <
    Greater,        // >
    LessEqual,      // <=
    GreaterEqual,   // >=
    Ampersand,      // &
    Pipe,           // |
    Caret,          // ^
    Tilde,          // ~
    LessLess,       // <<
    GreaterGreater, // >>
    
    // Punctuation
    LeftParen,      // (
    RightParen,     // )
    LeftBrace,      // {
    RightBrace,     // }
    LeftBracket,    // [
    RightBracket,   // ]
    Comma,          // ,
    Dot,            // .
    Colon,          // :
    DoubleColon,    // ::
    Semicolon,      // ;
    Arrow,          // ->
    
    // Special
    Newline,
    Eof,
    
    // Comments (preserved for tooling)
    Comment,
    DocComment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
    pub value: Option<LiteralValue>,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        line: usize,
        column: usize,
        byte_offset: usize,
        value: Option<LiteralValue>,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
            byte_offset,
            value,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({}) at {}:{}", self.token_type, self.lexeme, self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical error at {}:{}: {}", self.line, self.column, self.message)
    }
}

#[derive(Debug, Clone)]
pub struct LexWarning {
    pub message: String,
}

impl fmt::Display for LexWarning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical warning: {}", self.message)
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
    byte_offset: usize,
    keywords: HashMap<String, TokenType>,
    errors: Vec<LexError>,
    warnings: Vec<LexWarning>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        match Self::try_new(input) {
            Ok(lexer) => lexer,
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }
    }

    pub fn try_new(input: &str) -> Result<Self, String> {
        let mut keywords = HashMap::new();
        
        // Insert all keywords
        keywords.insert("and".to_string(), TokenType::And);
        keywords.insert("break".to_string(), TokenType::Break);
        keywords.insert("case".to_string(), TokenType::Case);
        keywords.insert("catch".to_string(), TokenType::Catch);
        keywords.insert("class".to_string(), TokenType::Class);
        keywords.insert("continue".to_string(), TokenType::Continue);
        keywords.insert("else".to_string(), TokenType::Else);
        keywords.insert("false".to_string(), TokenType::False);
        keywords.insert("finally".to_string(), TokenType::Finally);
        keywords.insert("fn".to_string(), TokenType::Fn);
        keywords.insert("for".to_string(), TokenType::For);
        keywords.insert("from".to_string(), TokenType::From);
        keywords.insert("if".to_string(), TokenType::If);
        keywords.insert("in".to_string(), TokenType::In);
        keywords.insert("is".to_string(), TokenType::Is);
        keywords.insert("match".to_string(), TokenType::Match);
        keywords.insert("not".to_string(), TokenType::Not);
        keywords.insert("null".to_string(), TokenType::Null);
        keywords.insert("of".to_string(), TokenType::Of);
        keywords.insert("or".to_string(), TokenType::Or);
        keywords.insert("pub".to_string(), TokenType::Pub);
        keywords.insert("raise".to_string(), TokenType::Raise);
        keywords.insert("require".to_string(), TokenType::Require);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("self".to_string(), TokenType::Self_);
        keywords.insert("static".to_string(), TokenType::Static);
        keywords.insert("super".to_string(), TokenType::Super);
        keywords.insert("true".to_string(), TokenType::True);
        keywords.insert("try".to_string(), TokenType::Try);
        keywords.insert("use".to_string(), TokenType::Use);
        keywords.insert("var".to_string(), TokenType::Var);
        keywords.insert("while".to_string(), TokenType::While);
        keywords.insert("with".to_string(), TokenType::With);
        
        // Check file size and issue warnings/errors according to spec
        let mut warnings = Vec::new();
        let size_bytes = input.len();
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
        
        if size_mb > 100.0 {
            return Err(format!("Source file is {:.1} MB, which exceeds maximum supported size of 100 MB", size_mb));
        }
        
        if size_mb > 10.0 {
            let warning_msg = format!("Source file is {:.1} MB, which exceeds recommended 10 MB", size_mb);
            eprintln!("Warning: {}", warning_msg);
            warnings.push(LexWarning {
                message: warning_msg,
            });
        }
        
        Ok(Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
            byte_offset: 0,
            keywords,
            errors: Vec::new(),
            warnings,
        })
    }
    
    pub fn tokenize(&mut self) -> (Vec<Token>, Vec<LexError>) {
        let mut tokens = Vec::new();
        
        // Skip BOM if present
        if self.input.len() >= 1 && self.input[0] == '\u{FEFF}' {
            self.position += 1;
            self.byte_offset += 3; // BOM is 3 bytes in UTF-8
            // Don't increment column for BOM
        }
        
        while !self.is_at_end() {
            if let Some(token) = self.scan_token() {
                tokens.push(token);
            }
        }
        
        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
            self.byte_offset,
            None,
        ));
        
        (tokens, self.errors.clone())
    }
    
    fn scan_token(&mut self) -> Option<Token> {
        let start_line = self.line;
        let start_column = self.column;
        let start_offset = self.byte_offset;
        
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
                self.line += 1;
                self.column = 1;
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
                self.line += 1;
                self.column = 1;
                Some(token)
            }
            
            // Single-character tokens
            '(' => Some(self.make_token(TokenType::LeftParen, "(", start_line, start_column, start_offset)),
            ')' => Some(self.make_token(TokenType::RightParen, ")", start_line, start_column, start_offset)),
            '{' => Some(self.make_token(TokenType::LeftBrace, "{", start_line, start_column, start_offset)),
            '}' => Some(self.make_token(TokenType::RightBrace, "}", start_line, start_column, start_offset)),
            '[' => Some(self.make_token(TokenType::LeftBracket, "[", start_line, start_column, start_offset)),
            ']' => Some(self.make_token(TokenType::RightBracket, "]", start_line, start_column, start_offset)),
            ',' => Some(self.make_token(TokenType::Comma, ",", start_line, start_column, start_offset)),
            '.' if !self.peek().is_ascii_digit() => Some(self.make_token(TokenType::Dot, ".", start_line, start_column, start_offset)),
            ';' => Some(self.make_token(TokenType::Semicolon, ";", start_line, start_column, start_offset)),
            '~' => Some(self.make_token(TokenType::Tilde, "~", start_line, start_column, start_offset)),
            '^' => Some(self.make_token(TokenType::Caret, "^", start_line, start_column, start_offset)),
            
            // Multi-character tokens
            '+' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenType::PlusEqual, "+=", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Plus, "+", start_line, start_column, start_offset))
                }
            }
            '-' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenType::MinusEqual, "-=", start_line, start_column, start_offset))
                } else if self.match_char('>') {
                    Some(self.make_token(TokenType::Arrow, "->", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Minus, "-", start_line, start_column, start_offset))
                }
            }
            '*' => {
                if self.match_char('*') {
                    Some(self.make_token(TokenType::StarStar, "**", start_line, start_column, start_offset))
                } else if self.match_char('=') {
                    Some(self.make_token(TokenType::StarEqual, "*=", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Star, "*", start_line, start_column, start_offset))
                }
            }
            '/' => {
                if self.match_char('/') {
                    self.scan_single_line_comment(start_line, start_column, start_offset)
                } else if self.match_char('*') {
                    self.scan_multi_line_comment(start_line, start_column, start_offset)
                } else if self.match_char('=') {
                    Some(self.make_token(TokenType::SlashEqual, "/=", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Slash, "/", start_line, start_column, start_offset))
                }
            }
            '%' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenType::PercentEqual, "%=", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Percent, "%", start_line, start_column, start_offset))
                }
            }
            '=' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenType::EqualEqual, "==", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Equal, "=", start_line, start_column, start_offset))
                }
            }
            '!' => {
                if self.match_char('=') {
                    Some(self.make_token(TokenType::BangEqual, "!=", start_line, start_column, start_offset))
                } else {
                    self.error("Unexpected character '!'".to_string(), start_line, start_column, start_offset);
                    None
                }
            }
            '<' => {
                if self.match_char('<') {
                    Some(self.make_token(TokenType::LessLess, "<<", start_line, start_column, start_offset))
                } else if self.match_char('=') {
                    Some(self.make_token(TokenType::LessEqual, "<=", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Less, "<", start_line, start_column, start_offset))
                }
            }
            '>' => {
                if self.match_char('>') {
                    Some(self.make_token(TokenType::GreaterGreater, ">>", start_line, start_column, start_offset))
                } else if self.match_char('=') {
                    Some(self.make_token(TokenType::GreaterEqual, ">=", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Greater, ">", start_line, start_column, start_offset))
                }
            }
            '&' => Some(self.make_token(TokenType::Ampersand, "&", start_line, start_column, start_offset)),
            '|' => Some(self.make_token(TokenType::Pipe, "|", start_line, start_column, start_offset)),
            ':' => {
                if self.match_char(':') {
                    Some(self.make_token(TokenType::DoubleColon, "::", start_line, start_column, start_offset))
                } else {
                    Some(self.make_token(TokenType::Colon, ":", start_line, start_column, start_offset))
                }
            }
            
            // String literals
            '"' => self.scan_string(start_line, start_column, start_offset),
            
            // Raw string literals
            'r' if self.peek() == '"' => {
                self.advance(); // consume the '"'
                self.scan_raw_string(start_line, start_column, start_offset)
            }
            
            // Numbers
            c if c.is_ascii_digit() => self.scan_number(c, start_line, start_column, start_offset),
            
            // Handle leading dot for float literals
            '.' if self.peek().is_ascii_digit() => self.scan_float_starting_with_dot(start_line, start_column, start_offset),
            
            // Identifiers and keywords
            c if c.is_ascii_alphabetic() || c == '_' => self.scan_identifier(c, start_line, start_column, start_offset),
            
            _ => {
                self.error(format!("Unexpected character '{}'", c), start_line, start_column, start_offset);
                None
            }
        }
    }
    
    fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        
        let c = self.input[self.position];
        self.position += 1;
        self.column += 1;
        self.byte_offset += c.len_utf8();
        c
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }
    
    fn peek_next(&self) -> char {
        if self.position + 1 >= self.input.len() {
            '\0'
        } else {
            self.input[self.position + 1]
        }
    }
    
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.input[self.position] != expected {
            false
        } else {
            self.advance();
            true
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn make_token(&self, token_type: TokenType, lexeme: &str, line: usize, column: usize, offset: usize) -> Token {
        Token::new(token_type, lexeme.to_string(), line, column, offset, None)
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
    
    fn skip_whitespace(&mut self) {
        while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
            self.advance();
        }
    }
    
    fn scan_single_line_comment(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 2; // Account for the "//" we already consumed
        
        // Check for documentation comment
        let is_doc_comment = self.peek() == '/';
        if is_doc_comment {
            self.advance(); // consume the third '/'
        }
        
        // Consume until end of line
        while self.peek() != '\n' && self.peek() != '\r' && !self.is_at_end() {
            self.advance();
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        Some(Token::new(
            if is_doc_comment { TokenType::DocComment } else { TokenType::Comment },
            lexeme,
            start_line,
            start_column,
            start_offset,
            None,
        ))
    }
    
    fn scan_multi_line_comment(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 2; // Account for the "/*" we already consumed
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
                    self.line += 1;
                    self.column = 1;
                }
                '\r' => {
                    if self.peek() == '\n' {
                        self.advance();
                    }
                    self.line += 1;
                    self.column = 1;
                }
                _ => {}
            }
        }
        
        if nesting_level > 0 {
            self.error("Unterminated comment".to_string(), start_line, start_column, start_offset);
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        Some(Token::new(
            if is_doc_comment { TokenType::DocComment } else { TokenType::Comment },
            lexeme,
            start_line,
            start_column,
            start_offset,
            None,
        ))
    }
    
    fn scan_string(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 1; // Include the opening quote
        
        // Check for triple-quoted string
        if self.peek() == '"' && self.peek_next() == '"' {
            self.advance(); // second quote
            self.advance(); // third quote
            return self.scan_triple_quoted_string(start_line, start_column, start_offset, start_pos);
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
                self.error("Unterminated string literal".to_string(), start_line, start_column, start_offset);
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
                            self.error("Invalid Unicode escape sequence".to_string(), self.line, self.column, self.byte_offset);
                            return None;
                        }
                    }
                    _ => {
                        self.error(format!("Invalid escape sequence '\\{}'", c), self.line, self.column, self.byte_offset);
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
            self.error("Unterminated string literal".to_string(), start_line, start_column, start_offset);
            return None;
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
        Some(self.make_literal_token(
            TokenType::StringLiteral,
            &lexeme,
            LiteralValue::String(value),
            start_line,
            start_column,
            start_offset,
        ))
    }
    
    fn scan_triple_quoted_string(&mut self, start_line: usize, start_column: usize, start_offset: usize, start_pos: usize) -> Option<Token> {
        let mut value = String::new();
        
        while !self.is_at_end() {
            // Check if we have three consecutive quotes at current position
            if self.position + 2 < self.input.len() && 
               self.input[self.position] == '"' && 
               self.input[self.position + 1] == '"' && 
               self.input[self.position + 2] == '"' {
                // Consume the closing triple quotes
                self.advance(); // first quote
                self.advance(); // second quote
                self.advance(); // third quote
                
                // Successfully parsed triple-quoted string
                let lexeme: String = self.input[start_pos..self.position].iter().collect();
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
                self.line += 1;
                self.column = 1;
            } else if c == '\r' {
                if self.peek() == '\n' {
                    value.push(c);
                    value.push(self.advance());
                } else {
                    value.push(c);
                }
                self.line += 1;
                self.column = 1;
                continue;
            }
            value.push(c);
        }
        
        // If we reach here, we hit end of input without finding closing quotes
        self.error("Unterminated triple-quoted string".to_string(), start_line, start_column, start_offset);
        None
    }
    
    fn scan_raw_string(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 2; // Include 'r"'
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
                self.error("Unterminated raw string literal".to_string(), start_line, start_column, start_offset);
                return None;
            }
            
            value.push(self.advance());
        }
        
        if !found_closing_quote {
            self.error("Unterminated raw string literal".to_string(), start_line, start_column, start_offset);
            return None;
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
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
            let result = char::from_u32(code_point);
            result
        } else {
            None
        }
    }
    
    fn scan_number(&mut self, first_digit: char, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 1;
        
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
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
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
    
    fn scan_float_starting_with_dot(&mut self, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 1; // Include the '.'
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
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
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
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
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
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
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
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
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
    
    fn scan_identifier(&mut self, _first_char: char, start_line: usize, start_column: usize, start_offset: usize) -> Option<Token> {
        let start_pos = self.position - 1;
        
        while !self.is_at_end() {
            let c = self.peek();
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }
        
        let lexeme: String = self.input[start_pos..self.position].iter().collect();
        
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
    
    pub fn errors(&self) -> &[LexError] {
        &self.errors
    }
    
    pub fn warnings(&self) -> &[LexWarning] {
        &self.warnings
    }
}