use crate::core::*;
use crate::lexer::TokenType;
use anyhow::Result;

use super::Parser;

impl Parser {
    /// Parse a block with { statements final_expr? }
    /// Expects the opening { to already be consumed
    pub fn parse_block(&mut self) -> Result<Block> {
        let mut statements = Vec::new();
        let mut final_expr = None;

        // Parse all complete statements
        while let Some(stmt) = self.try_parse_statement()? {
            statements.push(stmt);
        }

        // Try to parse optional final expression (if not at closing brace)
        if !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let expr = self.parse_expression()?;
            final_expr = Some(Box::new(expr));
        }

        // Check if the last statement is an expression that should be the final expression
        // This implements Rust-like "last-expression-is-value" semantics
        if final_expr.is_none() && !statements.is_empty() {
            if let Some(Statement::Expression(Expression::If { .. } | Expression::Match { .. })) =
                statements.last()
            {
                // Remove from statements and make it the final expression
                if let Some(Statement::Expression(expr)) = statements.pop() {
                    final_expr = Some(Box::new(expr));
                }
            }
        }

        self.expect(TokenType::RightBrace, "Expected '}' after block")?;
        Ok(Block {
            statements,
            final_expr,
        })
    }
}
