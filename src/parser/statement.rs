use crate::core::*;
use crate::lexer::TokenType;
use anyhow::{anyhow, Result};

use super::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement> {
        self.try_parse_statement()?.ok_or_else(|| anyhow!("Expected statement"))
    }

    pub fn try_parse_statement(&mut self) -> Result<Option<Statement>> {
        if let Some(stmt) = self.try_parse_macro()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_var_declaration()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_function_declaration()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_pub_declaration()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_class_declaration()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_import_statement()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_return_statement()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_break_statement()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_continue_statement()? {
            return Ok(Some(stmt));
        }
        if let Some(stmt) = self.try_parse_assignment()? {
            return Ok(Some(stmt));
        }

        // Fall back to expression statement
        self.try_parse_expression_statement()
    }

    fn try_parse_expression_statement(&mut self) -> Result<Option<Statement>> {
        let saved_pos = self.current;
        if let Ok(expr) = self.parse_expression() {
            if self.accept(TokenType::Semicolon) {
                Ok(Some(Statement::Expression(expr)))
            } else {
                // No semicolon - backtrack and return None
                self.current = saved_pos;
                Ok(None)
            }
        } else {
            // Couldn't parse expression either
            self.current = saved_pos;
            Ok(None)
        }
    }

    pub fn try_parse_macro(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Hash) {
            return Ok(None);
        }

        self.expect(TokenType::LeftBracket, "Expected '[' after '#'")?;
        let name_token = self.expect(TokenType::Ident, "Expected macro name")?;
        let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;
        let mut args = Vec::new();

        if self.accept(TokenType::LeftParen) {
            loop {
                if self.is_at_end() {
                    return Err(anyhow!("Unexpected EOF in macro arguments"));
                }

                if self.accept(TokenType::RightParen) {
                    break;
                }

                if let Some(arg) = self.parse_macro_arg()? {
                    args.push(arg);
                }

                if !self.accept(TokenType::Comma) {
                    self.expect(TokenType::RightParen, "Expected ')' after macro arguments")?;
                    break;
                }
            }
        }

        self.expect(TokenType::RightBracket, "Expected ']' after macro")?;
        // Recurse
        let statement = Box::new(self.parse_statement()?);
        Ok(Some(Statement::Macro { name, args, statement }))
    }

    fn parse_macro_arg(&mut self) -> Result<Option<MacroArg>> {
        match self.peek().token_type {
            TokenType::Ident => {
                let name_token = self.expect(TokenType::Ident, "Expected identifier")?;
                let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;

                if self.accept(TokenType::Colon) {
                    let value = self.parse_literal_value()?;
                    Ok(Some(MacroArg::Named(name, value)))
                } else {
                    // This was actually a positional literal, backtrack
                    self.current -= 1;
                    let value = self.parse_literal_value()?;
                    Ok(Some(MacroArg::Positional(value)))
                }
            }
            _ => {
                let value = self.parse_literal_value()?;
                Ok(Some(MacroArg::Positional(value)))
            }
        }
    }

    pub fn try_parse_var_declaration(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Var) {
            return Ok(None);
        }

        let pattern = self.parse_pattern()?;
        let value = if self.accept(TokenType::Equal) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(TokenType::Semicolon, "Expected ';'")?;

        Ok(Some(Statement::VarDecl { pattern, value }))
    }

    // ===== Stub Functions =====
    // These will be implemented next

    pub fn parse_pattern(&mut self) -> Result<Pattern> {
        match self.peek().token_type {
            TokenType::Ident => {
                let name_token = self.expect(TokenType::Ident, "Expected identifier")?;
                let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;
                Ok(Pattern::Variable(name))
            }
            _ => Err(anyhow!("Pattern parsing not fully implemented yet")),
        }
    }

    pub fn try_parse_assignment(&mut self) -> Result<Option<Statement>> {
        // Try to parse an lvalue
        let saved_pos = self.current;
        if let Ok(target) = self.try_parse_lvalue() {
            // Check if we have an assignment operator
            if let Some(op) = self.try_parse_assign_op() {
                let value = self.parse_expression()?;
                self.expect(TokenType::Semicolon, "Expected ';' after assignment")?;
                return Ok(Some(Statement::Assignment { target, op, value }));
            }
        }

        // Reset position if not an assignment
        self.current = saved_pos;
        Ok(None)
    }

    fn try_parse_lvalue(&mut self) -> Result<LValue> {
        let mut expr = if let Some(name_token) = self.accept_token(TokenType::Ident) {
            let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;
            LValue::Identifier(name)
        } else {
            return Err(anyhow!("Expected lvalue"));
        };

        // Handle chained property/index access
        loop {
            if self.accept(TokenType::Dot) {
                let property_token = self.expect(TokenType::Ident, "Expected property name after '.'")?;
                let property = property_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;

                // Convert LValue to Expression for GetAttr
                let base_expr = match expr {
                    LValue::Identifier(name) => Expression::Identifier(name),
                    LValue::GetAttr(obj, field) => Expression::GetAttr(obj, field),
                    LValue::GetItem(obj, key) => Expression::GetItem(obj, key),
                };
                expr = LValue::GetAttr(Box::new(base_expr), property);
            } else if self.accept(TokenType::LeftBracket) {
                let index = self.parse_expression()?;
                self.expect(TokenType::RightBracket, "Expected ']' after array index")?;

                // Convert LValue to Expression for GetItem
                let base_expr = match expr {
                    LValue::Identifier(name) => Expression::Identifier(name),
                    LValue::GetAttr(obj, field) => Expression::GetAttr(obj, field),
                    LValue::GetItem(obj, key) => Expression::GetItem(obj, key),
                };
                expr = LValue::GetItem(Box::new(base_expr), Box::new(index));
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn try_parse_assign_op(&mut self) -> Option<AssignOp> {
        if self.accept(TokenType::Equal) {
            Some(AssignOp::Assign)
        } else if self.accept(TokenType::PlusEqual) {
            Some(AssignOp::AddAssign)
        } else if self.accept(TokenType::MinusEqual) {
            Some(AssignOp::SubAssign)
        } else if self.accept(TokenType::StarEqual) {
            Some(AssignOp::MulAssign)
        } else if self.accept(TokenType::SlashEqual) {
            Some(AssignOp::DivAssign)
        } else if self.accept(TokenType::PercentEqual) {
            Some(AssignOp::ModAssign)
        } else {
            None
        }
    }

    pub fn try_parse_function_declaration(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Fn) {
            return Ok(None);
        }

        let name_token = self.expect(TokenType::Ident, "Expected function name")?;
        let name = name_token.text.ok_or_else(|| anyhow!("Function name token missing text"))?;

        self.expect(TokenType::LeftParen, "Expected '(' after function name")?;

        let mut params = Vec::new();
        loop {
            if self.is_at_end() {
                return Err(anyhow!("Unexpected EOF in function parameters"));
            }

            if self.accept(TokenType::RightParen) {
                break;
            }

            let param = self.parse_parameter()?;
            params.push(param);

            if !self.accept(TokenType::Comma) {
                self.expect(TokenType::RightParen, "Expected ')' after parameters")?;
                break;
            }
        }

        // Function body can be either a block or a simple expression
        let body = if self.accept(TokenType::LeftBrace) {
            // Block body: fn name() { statements }
            self.parse_block()?
        } else {
            // Simple expression body: fn name() expr
            let expr = self.parse_expression()?;
            Block {
                statements: vec![],
                final_expr: Some(Box::new(expr)),
            }
        };

        Ok(Some(Statement::FnDecl {
            name,
            params,
            body,
            is_pub: false, // Will be handled by try_parse_pub_declaration later
        }))
    }

    fn parse_parameter(&mut self) -> Result<Parameter> {
        // Check for parameter kind prefixes
        let kind = if self.accept(TokenType::Star) {
            if self.accept(TokenType::Star) {
                ParameterKind::Keyword // **name
            } else {
                ParameterKind::Rest // *name
            }
        } else {
            ParameterKind::Regular // name
        };

        let name_token = self.expect(TokenType::Ident, "Expected parameter name")?;
        let name = name_token.text.ok_or_else(|| anyhow!("Parameter name token missing text"))?;

        // Check for default value
        let default = if self.accept(TokenType::Equal) {
            Some(self.parse_literal_value()?)
        } else {
            None
        };

        Ok(Parameter { name, default, kind })
    }

    pub fn try_parse_pub_declaration(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Pub) {
            return Ok(None);
        }

        // TODO: Implement pub declaration parsing
        Err(anyhow!("Public declaration parsing not implemented yet"))
    }

    pub fn try_parse_class_declaration(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Class) {
            return Ok(None);
        }

        // TODO: Implement class declaration parsing
        Err(anyhow!("Class declaration parsing not implemented yet"))
    }

    pub fn try_parse_import_statement(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Use) {
            return Ok(None);
        }

        // TODO: Implement import statement parsing
        Err(anyhow!("Import statement parsing not implemented yet"))
    }

    pub fn try_parse_return_statement(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Return) {
            return Ok(None);
        }

        let expr = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect(TokenType::Semicolon, "Expected ';' after return statement")?;
        Ok(Some(Statement::Return(expr)))
    }

    pub fn try_parse_break_statement(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Break) {
            return Ok(None);
        }

        let expr = if self.check(TokenType::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect(TokenType::Semicolon, "Expected ';' after break statement")?;
        Ok(Some(Statement::Break(expr)))
    }

    pub fn try_parse_continue_statement(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Continue) {
            return Ok(None);
        }

        self.expect(TokenType::Semicolon, "Expected ';' after continue statement")?;
        Ok(Some(Statement::Continue))
    }
}