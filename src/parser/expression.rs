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
            if self.accept(TokenType::Dot) {
                let property_token =
                    self.expect(TokenType::Ident, "Expected property name after '.'")?;
                let property = property_token
                    .text
                    .ok_or_else(|| anyhow!("Identifier token missing text"))?;
                left = Expression::GetAttr(Box::new(left), property);
            } else if self.accept(TokenType::LeftBracket) {
                // TODO: this is a clear indicator our Block-Expression grammar is wrong
                // Check if the left expression is a control flow expression that shouldn't be indexed
                let should_allow_indexing = !matches!(
                    &left,
                    Expression::For { .. }
                        | Expression::While { .. }
                        | Expression::Loop { .. }
                        | Expression::If { .. }
                        | Expression::Match { .. }
                        | Expression::Try { .. }
                        | Expression::With { .. }
                );

                if should_allow_indexing {
                    let index = self.parse_expression()?;
                    self.expect(TokenType::RightBracket, "Expected ']' after array index")?;
                    left = Expression::GetItem(Box::new(left), Box::new(index));
                } else {
                    // Don't consume the LeftBracket, let it be parsed as a separate expression
                    self.current -= 1; // Put back the LeftBracket token
                    break;
                }
            } else if self.accept(TokenType::LeftParen) {
                let mut args = Vec::new();

                loop {
                    if self.is_at_end() {
                        return Err(anyhow!("Unexpected EOF in function call arguments"));
                    }

                    if self.accept(TokenType::RightParen) {
                        break;
                    }

                    args.push(self.parse_expression()?);
                    if !self.accept(TokenType::Comma) {
                        self.expect(TokenType::RightParen, "Expected ')' after arguments")?;
                        break;
                    }
                }

                // Check if this is a method call (function call on a property access)
                left = match left {
                    Expression::GetAttr(obj, method_name) => {
                        // Convert obj.method(args) to MethodCall
                        Expression::MethodCall(obj, method_name, args)
                    }
                    _ => {
                        // Regular function call
                        Expression::FunctionCall(Box::new(left), args)
                    }
                };
            } else {
                // Binary operators
                if let Some(expr_constructor) =
                    self.get_binary_expression_constructor(self.peek().token_type)
                {
                    self.advance();
                    let right_precedence =
                        if self.is_right_associative_token(self.previous().token_type) {
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

        Ok(left)
    }

    pub fn parse_unary(&mut self) -> Result<Expression> {
        if self.accept(TokenType::Minus) {
            // Unary operators have precedence 14 (higher than binary ops, lower than postfix)
            let expr = self.parse_precedence(14)?;
            Ok(Expression::Neg(Box::new(expr)))
        } else if self.accept(TokenType::Not) {
            // Unary operators have precedence 14 (higher than binary ops, lower than postfix)
            let expr = self.parse_precedence(14)?;
            Ok(Expression::Not(Box::new(expr)))
        } else {
            self.parse_primary()
        }
    }

    pub fn parse_primary(&mut self) -> Result<Expression> {
        if self.accept(TokenType::True) {
            Ok(Expression::Literal(LiteralValue::Bool(true)))
        } else if self.accept(TokenType::False) {
            Ok(Expression::Literal(LiteralValue::Bool(false)))
        } else if self.accept(TokenType::Null) {
            Ok(Expression::Literal(LiteralValue::Null))
        } else if let Some(token) = self.accept_token(TokenType::Int) {
            let text = token
                .text
                .as_ref()
                .ok_or_else(|| anyhow!("Int token missing text"))?;
            let n = text
                .parse::<i64>()
                .map_err(|e| anyhow!("Failed to parse integer '{}': {}", text, e))?;
            Ok(Expression::Literal(LiteralValue::Int(n)))
        } else if let Some(token) = self.accept_token(TokenType::Float) {
            let text = token
                .text
                .as_ref()
                .ok_or_else(|| anyhow!("Float token missing text"))?;
            let f = text
                .parse::<f64>()
                .map_err(|e| anyhow!("Failed to parse float '{}': {}", text, e))?;
            Ok(Expression::Literal(LiteralValue::Float(f)))
        } else if let Some(token) = self.accept_token(TokenType::String) {
            let text = token
                .text
                .as_ref()
                .ok_or_else(|| anyhow!("String token missing text"))?
                .clone();
            Ok(Expression::Literal(LiteralValue::String(text)))
        } else if let Some(token) = self.accept_token(TokenType::Ident) {
            let text = token
                .text
                .as_ref()
                .ok_or_else(|| anyhow!("Identifier token missing text"))?
                .clone();
            Ok(Expression::Identifier(text))
        } else if self.check(TokenType::StringPart) || self.check(TokenType::InterpolationStart) {
            self.parse_interpolated_string()
        } else if self.check(TokenType::LeftBrace) {
            self.parse_brace_expression()
        } else if self.check(TokenType::If) {
            self.try_parse_if_expression()
        } else if self.check(TokenType::Loop) {
            self.try_parse_loop_expression()
        } else if self.check(TokenType::While) {
            self.try_parse_while_expression()
        } else if self.check(TokenType::For) {
            self.try_parse_for_expression()
        } else if self.check(TokenType::Match) {
            self.try_parse_match_expression()
        } else if self.check(TokenType::Try) {
            self.try_parse_try_expression()
        } else if self.check(TokenType::With) {
            self.try_parse_with_expression()
        } else if self.check(TokenType::Pipe) {
            self.try_parse_lambda_expression()
        } else if self.accept(TokenType::LeftBracket) {
            self.parse_list_literal()
        } else if self.accept(TokenType::LeftParen) {
            // Grouped expression: (expr)
            let expr = self.parse_expression()?;
            self.expect(
                TokenType::RightParen,
                "Expected ')' after grouped expression",
            )?;
            Ok(expr)
        } else {
            Err(anyhow!("Unexpected token: {:?}", self.peek().token_type))
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
                let text = token
                    .text
                    .as_ref()
                    .ok_or_else(|| anyhow!("Int token missing text"))?;
                let n = text
                    .parse::<i64>()
                    .map_err(|e| anyhow!("Failed to parse integer '{}': {}", text, e))?;
                self.advance();
                Ok(LiteralValue::Int(n))
            }
            TokenType::Float => {
                let text = token
                    .text
                    .as_ref()
                    .ok_or_else(|| anyhow!("Float token missing text"))?;
                let f = text
                    .parse::<f64>()
                    .map_err(|e| anyhow!("Failed to parse float '{}': {}", text, e))?;
                self.advance();
                Ok(LiteralValue::Float(f))
            }
            TokenType::String => {
                let text = token
                    .text
                    .as_ref()
                    .ok_or_else(|| anyhow!("String token missing text"))?
                    .clone();
                self.advance();
                Ok(LiteralValue::String(text))
            }
            TokenType::RawString => {
                let text = token
                    .text
                    .as_ref()
                    .ok_or_else(|| anyhow!("RawString token missing text"))?
                    .clone();
                self.advance();
                Ok(LiteralValue::RawString(text))
            }
            _ => Err(anyhow!(
                "Expected literal value, found {:?}",
                token.token_type
            )),
        }
    }

    pub fn parse_block_expression(&mut self) -> Result<Expression> {
        // Opening { already consumed by parse_primary
        let block = self.parse_block()?;
        Ok(Expression::Block(block))
    }

    pub fn try_parse_if_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::If, "Expected 'if'")?;

        let condition = Box::new(self.parse_expression()?);

        self.expect(TokenType::LeftBrace, "Expected '{' after if condition")?;
        let then_expr = self.parse_block()?;

        let mut else_expr = None;

        // Handle else clause
        if self.accept(TokenType::Else) {
            if self.accept(TokenType::If) {
                // else if - parse as nested if expression
                let nested_if = self.try_parse_if_expression()?;
                else_expr = Some(Block {
                    statements: vec![],
                    final_expr: Some(Box::new(nested_if)),
                });
            } else {
                // final else clause
                self.expect(TokenType::LeftBrace, "Expected '{' after else")?;
                else_expr = Some(self.parse_block()?);
            }
        }

        Ok(Expression::If {
            condition,
            then_expr,
            else_expr,
        })
    }

    pub fn parse_list_literal(&mut self) -> Result<Expression> {
        // Opening [ already consumed by parse_primary
        let mut elements = Vec::new();

        // Handle empty list
        if self.accept(TokenType::RightBracket) {
            return Ok(Expression::List(elements));
        }

        // Parse comma-separated expressions
        loop {
            elements.push(self.parse_expression()?);

            if self.accept(TokenType::RightBracket) {
                break;
            } else if self.accept(TokenType::Comma) {
                // Check for trailing comma before ]
                if self.check(TokenType::RightBracket) {
                    self.advance(); // consume the ]
                    break;
                }
                continue;
            } else {
                return Err(anyhow!("Expected ',' or ']' in list literal"));
            }
        }

        Ok(Expression::List(elements))
    }

    pub fn try_parse_loop_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::Loop, "Expected 'loop'")?;
        self.expect(TokenType::LeftBrace, "Expected '{' after loop")?;
        let body = self.parse_block()?;
        Ok(Expression::Loop { body })
    }

    pub fn try_parse_while_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::While, "Expected 'while'")?;
        let condition = Box::new(self.parse_expression()?);
        self.expect(TokenType::LeftBrace, "Expected '{' after while condition")?;
        let body = self.parse_block()?;
        Ok(Expression::While { condition, body })
    }

    pub fn try_parse_for_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::For, "Expected 'for'")?;
        let pattern = self.parse_pattern()?;
        self.expect(TokenType::In, "Expected 'in' after for pattern")?;
        let iter = Box::new(self.parse_expression()?);
        self.expect(TokenType::LeftBrace, "Expected '{' after for iterator")?;
        let body = self.parse_block()?;
        Ok(Expression::For {
            pattern,
            iter,
            body,
        })
    }

    pub fn try_parse_match_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::Match, "Expected 'match'")?;
        let expr = Box::new(self.parse_expression()?);
        self.expect(TokenType::LeftBrace, "Expected '{' after match expression")?;

        let mut cases = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            self.expect(TokenType::Case, "Expected 'case' in match arm")?;
            let pattern = self.parse_pattern()?;

            // Optional guard
            let guard = if self.accept(TokenType::If) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            self.expect(TokenType::LeftBrace, "Expected '{' after match pattern")?;
            let body = self.parse_block()?;

            cases.push(MatchCase {
                pattern,
                guard,
                body,
            });
        }

        self.expect(TokenType::RightBrace, "Expected '}' after match cases")?;
        Ok(Expression::Match { expr, cases })
    }

    pub fn try_parse_try_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::Try, "Expected 'try'")?;
        self.expect(TokenType::LeftBrace, "Expected '{' after 'try'")?;
        let body = self.parse_block()?;

        self.expect(TokenType::Catch, "Expected 'catch' after try block")?;
        let pattern = self.parse_pattern()?;
        self.expect(TokenType::LeftBrace, "Expected '{' after catch pattern")?;
        let catch_body = self.parse_block()?;

        let catch = CatchClause {
            pattern,
            body: catch_body,
        };

        Ok(Expression::Try { body, catch })
    }

    pub fn try_parse_with_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::With, "Expected 'with'")?;

        let mut resources = Vec::new();
        loop {
            let name_token = self.expect(TokenType::Ident, "Expected resource identifier")?;
            let name = name_token
                .text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;

            // Check for two syntax forms:
            // 1. with name = expr { ... } (original spec syntax)
            // 2. with variable { ... } (simplified syntax for existing variables)
            if self.accept(TokenType::Equal) {
                // Original syntax: with name = expr
                let value = self.parse_expression()?;
                resources.push(WithResource { name, value });
            } else {
                // Simplified syntax: with variable
                // Treat as: with name = name (use existing variable as resource)
                let value = Expression::Identifier(name.clone());
                resources.push(WithResource { name, value });
            }

            if !self.accept(TokenType::Comma) {
                break;
            }
        }

        self.expect(TokenType::LeftBrace, "Expected '{' after with resources")?;
        let body = self.parse_block()?;

        Ok(Expression::With { resources, body })
    }

    pub fn try_parse_lambda_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::Pipe, "Expected '|' to start lambda")?;

        let mut params = Vec::new();

        // Parse parameters until closing |
        while !self.check(TokenType::Pipe) && !self.is_at_end() {
            let param_token = self.expect(TokenType::Ident, "Expected parameter name")?;
            let param_name = param_token
                .text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            params.push(param_name);

            if !self.accept(TokenType::Comma) {
                break;
            }
        }

        self.expect(TokenType::Pipe, "Expected '|' to end lambda parameters")?;

        // Parse body - either expression or block
        let body = if self.check(TokenType::LeftBrace) {
            // Block body: |x| { ... }
            self.advance(); // consume {
            let block = self.parse_block()?;
            LambdaBody::Block(block)
        } else {
            // Expression body: |x| x + 1
            let expr = self.parse_expression()?;
            LambdaBody::Expression(Box::new(expr))
        };

        Ok(Expression::Lambda { params, body })
    }

    pub fn parse_brace_expression(&mut self) -> Result<Expression> {
        self.expect(TokenType::LeftBrace, "Expected '{'")?;

        // Handle empty dict: {}
        if self.accept(TokenType::RightBrace) {
            return Ok(Expression::Dict(Vec::new()));
        }

        // Try to parse as dictionary first, fall back to block
        if let Some(dict_expr) = self.with_checkpoint(|parser| parser.try_parse_dict_content())? {
            return Ok(dict_expr);
        }

        // Parse as block expression (opening { already consumed)
        let block = self.parse_block()?;
        Ok(Expression::Block(block))
    }

    pub fn try_parse_dict_content(&mut self) -> Result<Option<Expression>> {
        let mut pairs = Vec::new();

        loop {
            // Parse key expression - use precedence lower than pipe to avoid consuming colons
            let key = match self.parse_precedence(14) {
                // Higher than pipe precedence (13)
                Ok(expr) => expr,
                Err(_) => return Ok(None), // Not a valid expression, not a dictionary
            };

            // Must be followed by ':'
            self.expect(TokenType::Colon, "Expected ':' after dictionary key")?;

            // Parse value expression
            let value = self.parse_expression()?;

            pairs.push((key, value));

            // Check for continuation
            if self.accept(TokenType::Comma) {
                // Allow trailing comma before }
                if self.check(TokenType::RightBrace) {
                    break;
                }
                continue;
            } else {
                break;
            }
        }

        self.expect(TokenType::RightBrace, "Expected '}' to close dictionary")?;
        Ok(Some(Expression::Dict(pairs)))
    }

    fn parse_interpolated_string(&mut self) -> Result<Expression> {
        use crate::core::InterpolationPart;
        let mut parts = Vec::new();

        loop {
            if let Some(token) = self.accept_token(TokenType::StringPart) {
                // Text part
                let text = token
                    .text
                    .ok_or_else(|| anyhow!("StringPart token missing text"))?;
                parts.push(InterpolationPart::Text(text));
            } else if self.accept(TokenType::InterpolationStart) {
                // Expression part
                let expr = self.parse_expression()?;
                parts.push(InterpolationPart::Expression(expr));
                self.expect(
                    TokenType::InterpolationEnd,
                    "Expected closing '}' in string interpolation",
                )?;
            } else {
                break;
            }
        }

        if parts.is_empty() {
            return Err(anyhow!("Empty interpolated string"));
        }

        Ok(Expression::InterpolatedString(parts))
    }
}
