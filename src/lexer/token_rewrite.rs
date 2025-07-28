use crate::lexer::{Lexer, Token, TokenType};
use anyhow::Result;

impl Lexer {
    /// Phase 3: Rewrite token pairs like "not in" -> NotIn and "is not" -> IsNot
    pub fn rewrite_token_pairs(&self, tokens: Vec<Token>) -> Result<Vec<Token>> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            // Check for "not in" pattern
            if i + 1 < tokens.len()
                && tokens[i].token_type == TokenType::Not
                && tokens[i + 1].token_type == TokenType::In
            {
                // Replace "not in" with "NotIn"
                result.push(Token::simple(TokenType::NotIn));
                i += 2; // Skip both tokens
            }
            // Check for "is not" pattern
            else if i + 1 < tokens.len()
                && tokens[i].token_type == TokenType::Is
                && tokens[i + 1].token_type == TokenType::Not
            {
                // Replace "is not" with "IsNot"
                result.push(Token::simple(TokenType::IsNot));
                i += 2; // Skip both tokens
            } else {
                // Copy token as-is
                result.push(tokens[i].clone());
                i += 1;
            }
        }

        Ok(result)
    }
}
