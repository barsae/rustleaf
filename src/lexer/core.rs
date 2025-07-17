use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::{LexError, LexErrors, LexWarning};
use crate::lexer::keywords::create_keywords_map;
use crate::lexer::scanner::TokenScanner;

pub struct Lexer;

impl Lexer {
    /// Main entry point for lexical analysis.
    /// Prints warnings to stderr and returns Result<Vec<Token>, LexErrors>.
    pub fn new(input: &str) -> Result<Vec<Token>, LexErrors> {
        let (tokens, warnings, errors) = Self::tokenize_internal(input)?;
        
        // Print warnings to stderr
        for warning in warnings {
            eprintln!("Warning: {}", warning);
        }
        
        if errors.is_empty() {
            Ok(tokens)
        } else {
            Err(LexErrors::new(errors))
        }
    }
    
    /// Entry point for unit tests that need access to warnings.
    /// Does not print warnings to stderr.
    pub fn new_warnings(input: &str) -> Result<(Vec<Token>, Vec<LexWarning>), LexErrors> {
        let (tokens, warnings, errors) = Self::tokenize_internal(input)?;
        
        if errors.is_empty() {
            Ok((tokens, warnings))
        } else {
            Err(LexErrors::new(errors))
        }
    }
    
    fn tokenize_internal(input: &str) -> Result<(Vec<Token>, Vec<LexWarning>, Vec<LexError>), LexErrors> {
        let keywords = create_keywords_map();
        
        // Check file size and issue warnings/errors according to spec
        let mut warnings = Vec::new();
        let size_bytes = input.len();
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
        
        if size_mb > 100.0 {
            let error = LexError {
                message: format!("Source file is {:.1} MB, which exceeds maximum supported size of 100 MB", size_mb),
                line: 1,
                column: 1,
                byte_offset: 0,
            };
            return Err(LexErrors::new(vec![error]));
        }
        
        if size_mb > 10.0 {
            let warning_msg = format!("Source file is {:.1} MB, which exceeds recommended 10 MB", size_mb);
            warnings.push(LexWarning {
                message: warning_msg,
            });
        }
        
        let input_chars: Vec<char> = input.chars().collect();
        let mut position = 0;
        let mut line = 1;
        let mut column = 1;
        let mut byte_offset = 0;
        let mut errors = Vec::new();
        let mut tokens = Vec::new();
        
        // Skip BOM if present
        if !input_chars.is_empty() && input_chars[0] == '\u{FEFF}' {
            position += 1;
            byte_offset += 3; // BOM is 3 bytes in UTF-8
            // Don't increment column for BOM
        }
        
        while position < input_chars.len() {
            let mut scanner = TokenScanner::new(
                &input_chars,
                &mut position,
                &mut line,
                &mut column,
                &mut byte_offset,
                &keywords,
                &mut errors,
            );
            
            if let Some(token) = scanner.scan_token() {
                tokens.push(token);
            }
        }
        
        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            line,
            column,
            byte_offset,
            None,
        ));
        
        Ok((tokens, warnings, errors))
    }
}