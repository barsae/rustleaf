/// Lexer implementation for RustLeaf
use anyhow::Result;
use crate::lexer::Token;

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Tokenize source code and produce tokens
    pub fn tokenize(source: &str) -> Result<Vec<Token>> {
        let mut lexer = Self::new(source);
        lexer.tokenize_internal()
    }

    fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    fn tokenize_internal(&mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.next_token()?;
        }

        self.tokens.push(Token::Eof);
        Ok(self.tokens.clone())
    }

    fn next_token(&mut self) -> Result<()> {
        // TODO: Implement tokenizing
        // This will handle:
        // - Whitespace and comments
        // - Keywords and identifiers
        // - Numbers (int and float)
        // - Strings (including interpolation)
        // - Operators and delimiters

        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' => {
                // Skip whitespace
            }
            '\n' => {
                self.line += 1;
                self.column = 1;
            }
            _ => {
                return Err(anyhow::anyhow!("Unexpected character: '{}'", c));
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

}