/// Parser implementation for RustLeaf
use crate::core::*;
use crate::lexer::Token;
use anyhow::Result;
use super::stream::TokenStream;

pub struct Parser;

impl Parser {
    /// Parse source code string directly into an AST
    pub fn parse_str(source: &str) -> Result<Program> {
        let tokens = crate::lexer::Lexer::tokenize(source)?;
        Self::parse(tokens)
    }

    /// Parse tokens into an AST
    pub fn parse(tokens: Vec<Token>) -> Result<Program> {
        let mut stream = TokenStream::new(tokens);
        stream.parse(parse_program)
    }
}

// ===== New API functions =====

/// Parse a complete program
fn parse_program(s: &mut TokenStream) -> Result<Program> {
    let mut statements = Vec::new();

    while !s.is_at_end() {
        statements.push(parse_statement(s)?);
    }

    Ok(Program(statements))
}

/// Parse a single statement
fn parse_statement(s: &mut TokenStream) -> Result<Statement> {
    super::statement::parse_statement(s)
}