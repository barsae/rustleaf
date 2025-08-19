use super::stream::TokenStream;
/// Parser implementation for RustLeaf
use crate::core::*;
use crate::lexer::Token;
use anyhow::Result;
use tracing::debug;

/// Parser for converting tokens into abstract syntax trees
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
    debug!("parse_program: starting");
    let mut statements = Vec::new();

    while !s.is_at_end() {
        debug!(
            "parse_program: parsing statement at position {} ({})",
            s.position(),
            s.current_token_info()
        );

        // Try to parse a statement normally
        if let Some(stmt) = s.try_parse(try_parse_statement)? {
            statements.push(stmt);
        } else {
            // Statement parsing failed, try to parse as a final expression without semicolon
            if let Some(final_expr) = s.try_parse(parse_final_expression)? {
                statements.push(Statement::Expression(final_expr));
                break;
            } else {
                // Both failed, return an error
                return Err(parse_statement(s).unwrap_err());
            }
        }
    }

    debug!("parse_program: parsed {} statements", statements.len());
    Ok(Program(statements))
}

/// Parse a single statement
fn parse_statement(s: &mut TokenStream) -> Result<Statement> {
    super::statement::parse_statement(s)
}

/// Wrapper for try_parse compatibility
fn try_parse_statement(s: &mut TokenStream) -> Result<Option<Statement>> {
    match parse_statement(s) {
        Ok(stmt) => Ok(Some(stmt)),
        Err(_) => Ok(None),
    }
}

/// Parse a final expression (without requiring a semicolon)
fn parse_final_expression(s: &mut TokenStream) -> Result<Option<Expression>> {
    use crate::lexer::TokenType;

    // Parse the expression
    let expr = match super::expression::parse_expression(s) {
        Ok(expr) => expr,
        Err(_) => return Ok(None),
    };

    // Check if we're at EOF (no semicolon) or if there's a semicolon that we should NOT consume
    if s.is_at_end() {
        // Perfect - we're at the end with no semicolon
        Ok(Some(expr))
    } else if s.peek_type() == TokenType::Semicolon {
        // There's a semicolon, which means this should be parsed as a regular statement
        Ok(None)
    } else {
        // There are more tokens but no semicolon - this is an error
        Ok(None)
    }
}
