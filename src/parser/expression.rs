use crate::core::*;
use crate::lexer::TokenType;
use anyhow::{anyhow, Result};

use super::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_precedence(0)
    }

    /// Parse expressions with precedence climbing
    pub fn parse_precedence(&mut self, min_precedence: u8) -> Result<Expression> {
        let mut left = self.parse_unary()?;

        while !self.is_at_end() {
            let op_precedence = self.get_binary_precedence(self.peek().token_type);
            if op_precedence < min_precedence {
                break;
            }

            // Handle postfix operators (method calls, array access, property access)
            match self.peek().token_type {
                TokenType::Dot => {
                    self.advance();
                    let property_token = self.expect(TokenType::Ident, "Expected property name after '.'")?;
                    let property = property_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;
                    left = Expression::GetAttr(Box::new(left), property);
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenType::RightBracket, "Expected ']' after array index")?;
                    left = Expression::GetItem(Box::new(left), Box::new(index));
                }
                TokenType::LeftParen => {
                    self.advance();
                    let mut args = Vec::new();

                    while !self.check(TokenType::RightParen) && !self.is_at_end() {
                        args.push(self.parse_expression()?);
                        if !self.check(TokenType::RightParen) {
                            self.expect(TokenType::Comma, "Expected ',' between arguments")?;
                        }
                    }

                    self.expect(TokenType::RightParen, "Expected ')' after arguments")?;
                    left = Expression::FunctionCall(Box::new(left), args);
                }
                _ => {
                    // Binary operators
                    if let Some(expr_constructor) = self.get_binary_expression_constructor(self.peek().token_type) {
                        self.advance();
                        let right_precedence = if self.is_right_associative_token(self.previous().token_type) {
                            op_precedence
                        } else {
                            op_precedence + 1
                        };
                        let right = self.parse_precedence(right_precedence)?;
                        left = expr_constructor(Box::new(left), Box::new(right));
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(left)
    }

    pub fn parse_unary(&mut self) -> Result<Expression> {
        if self.accept(TokenType::Minus).is_some() {
            let expr = self.parse_unary()?;
            Ok(Expression::Neg(Box::new(expr)))
        } else if self.accept(TokenType::Not).is_some() {
            let expr = self.parse_unary()?;
            Ok(Expression::Not(Box::new(expr)))
        } else if self.accept(TokenType::Tilde).is_some() {
            let expr = self.parse_unary()?;
            Ok(Expression::BitNot(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    pub fn parse_primary(&mut self) -> Result<Expression> {
        if self.accept(TokenType::True).is_some() {
            Ok(Expression::Literal(LiteralValue::Bool(true)))
        } else if self.accept(TokenType::False).is_some() {
            Ok(Expression::Literal(LiteralValue::Bool(false)))
        } else if self.accept(TokenType::Null).is_some() {
            Ok(Expression::Literal(LiteralValue::Null))
        } else if let Some(token) = self.accept(TokenType::Int) {
            let text = token.text.as_ref()
                .ok_or_else(|| anyhow!("Int token missing text"))?;
            let n = text.parse::<i64>()
                .map_err(|e| anyhow!("Failed to parse integer '{}': {}", text, e))?;
            Ok(Expression::Literal(LiteralValue::Int(n)))
        } else if let Some(token) = self.accept(TokenType::Float) {
            let text = token.text.as_ref()
                .ok_or_else(|| anyhow!("Float token missing text"))?;
            let f = text.parse::<f64>()
                .map_err(|e| anyhow!("Failed to parse float '{}': {}", text, e))?;
            Ok(Expression::Literal(LiteralValue::Float(f)))
        } else if let Some(token) = self.accept(TokenType::String) {
            let text = token.text.as_ref()
                .ok_or_else(|| anyhow!("String token missing text"))?
                .clone();
            Ok(Expression::Literal(LiteralValue::String(text)))
        } else if let Some(token) = self.accept(TokenType::Ident) {
            let text = token.text.as_ref()
                .ok_or_else(|| anyhow!("Identifier token missing text"))?
                .clone();
            Ok(Expression::Identifier(text))
        } else {
            Err(anyhow!(
                "Unexpected token: {:?}",
                self.peek().token_type
            ))
        }
    }

    pub fn parse_literal_value(&mut self) -> Result<LiteralValue> {
        let token = self.peek();
        match &token.token_type {
            TokenType::True => {
                self.advance();
                Ok(LiteralValue::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Ok(LiteralValue::Bool(false))
            }
            TokenType::Null => {
                self.advance();
                Ok(LiteralValue::Null)
            }
            TokenType::Int => {
                let text = token.text.as_ref()
                    .ok_or_else(|| anyhow!("Int token missing text"))?;
                let n = text.parse::<i64>()
                    .map_err(|e| anyhow!("Failed to parse integer '{}': {}", text, e))?;
                self.advance();
                Ok(LiteralValue::Int(n))
            }
            TokenType::Float => {
                let text = token.text.as_ref()
                    .ok_or_else(|| anyhow!("Float token missing text"))?;
                let f = text.parse::<f64>()
                    .map_err(|e| anyhow!("Failed to parse float '{}': {}", text, e))?;
                self.advance();
                Ok(LiteralValue::Float(f))
            }
            TokenType::String => {
                let text = token.text.as_ref()
                    .ok_or_else(|| anyhow!("String token missing text"))?
                    .clone();
                self.advance();
                Ok(LiteralValue::String(text))
            }
            TokenType::RawString => {
                let text = token.text.as_ref()
                    .ok_or_else(|| anyhow!("RawString token missing text"))?
                    .clone();
                self.advance();
                Ok(LiteralValue::RawString(text))
            }
            _ => Err(anyhow!("Expected literal value, found {:?}", token.token_type)),
        }
    }
}