use crate::lexer::{Lexer, Token, TokenType};
use anyhow::Result;

impl Lexer {
    pub fn expand_string_interpolation(&self, tokens: Vec<Token>) -> Result<Vec<Token>> {
        let mut result = Vec::new();

        for token in tokens {
            if token.token_type == TokenType::String && self.contains_interpolation(&token) {
                result.extend(self.expand_interpolated_string(token)?);
            } else {
                result.push(token);
            }
        }

        Ok(result)
    }

    fn contains_interpolation(&self, token: &Token) -> bool {
        if let Some(text) = &token.text {
            text.contains("${")
        } else {
            false
        }
    }

    fn expand_interpolated_string(&self, token: Token) -> Result<Vec<Token>> {
        let text = token.text.as_ref().unwrap();
        let mut result = Vec::new();
        let mut current_pos = 0;

        while let Some(start_pos) = text[current_pos..].find("${") {
            let actual_start = current_pos + start_pos;

            // Add text part before interpolation (if any)
            if actual_start > current_pos {
                let text_part = &text[current_pos..actual_start];
                result.push(Token::with_text(TokenType::StringPart, text_part));
            }

            // Find the matching closing brace
            let expr_start = actual_start + 2; // Skip ${
            if let Some(end_pos) = self.find_matching_brace(text, expr_start) {
                // Add interpolation start marker
                result.push(Token::simple(TokenType::InterpolationStart));

                // Extract and tokenize the expression
                let expr_text = &text[expr_start..end_pos];
                let expr_tokens = Self::tokenize(expr_text)?;

                // Add expression tokens (excluding EOF)
                for expr_token in expr_tokens {
                    if expr_token.token_type != TokenType::Eof {
                        result.push(expr_token);
                    }
                }

                // Add interpolation end marker
                result.push(Token::simple(TokenType::InterpolationEnd));

                current_pos = end_pos + 1; // Skip closing brace
            } else {
                return Err(anyhow::anyhow!(
                    "Unclosed interpolation in string: missing '}}' after '${{' at position {}",
                    actual_start
                ));
            }
        }

        // Add remaining text (if any)
        if current_pos < text.len() {
            let remaining = &text[current_pos..];
            result.push(Token::with_text(TokenType::StringPart, remaining));
        }

        Ok(result)
    }

    fn find_matching_brace(&self, text: &str, start: usize) -> Option<usize> {
        let chars: Vec<char> = text.chars().collect();
        let mut brace_count = 1;
        let mut in_string = false;
        let mut escape_next = false;

        for (i, &ch) in chars.iter().enumerate().skip(start) {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' if in_string => escape_next = true,
                '"' => in_string = !in_string,
                '{' if !in_string => brace_count += 1,
                '}' if !in_string => {
                    brace_count -= 1;
                    if brace_count == 0 {
                        return Some(i);
                    }
                }
                _ => {}
            }
        }

        None // No matching brace found
    }
}
