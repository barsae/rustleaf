use crate::core::*;
use crate::lexer::TokenType;
use anyhow::{anyhow, Result};

use super::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement> {
        self.try_parse_statement()?
            .ok_or_else(|| anyhow!("Expected statement"))
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

        // Try block-like expressions that don't require semicolons
        if let Some(stmt) = self.try_parse_block_like_expression_statement()? {
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

        // Accept either identifier or macro keyword
        let name = if self.check(TokenType::Ident) {
            let token = self.advance();
            token
                .text
                .clone()
                .ok_or_else(|| anyhow!("Identifier token missing text"))?
        } else if self.accept(TokenType::Macro) {
            "macro".to_string()
        } else {
            return Err(anyhow!("Expected macro name: identifier or 'macro'"));
        };
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
        Ok(Some(Statement::Macro {
            name,
            args,
            statement,
        }))
    }

    fn parse_macro_arg(&mut self) -> Result<Option<MacroArg>> {
        match self.peek().token_type {
            TokenType::Ident => {
                let name_token = self.expect(TokenType::Ident, "Expected identifier")?;
                let name = name_token
                    .text
                    .ok_or_else(|| anyhow!("Identifier token missing text"))?;

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
                let name = name_token
                    .text
                    .ok_or_else(|| anyhow!("Identifier token missing text"))?;
                if name == "_" {
                    Ok(Pattern::Wildcard)
                } else {
                    Ok(Pattern::Variable(name))
                }
            }
            TokenType::Int
            | TokenType::Float
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Null => {
                // Use with_checkpoint for safe lookahead
                match self.with_checkpoint(|parser| {
                    let first_literal = parser.parse_literal_value()?;

                    // Check for range operators
                    if parser.accept(TokenType::DotDot) || parser.accept(TokenType::DotDotEqual) {
                        // This is a range pattern
                        let second_literal = parser.parse_literal_value()?;
                        Ok(Some(Pattern::Range(
                            Box::new(Pattern::Literal(first_literal)),
                            Box::new(Pattern::Literal(second_literal)),
                        )))
                    } else {
                        // Not a range pattern
                        Ok(None)
                    }
                })? {
                    Some(range_pattern) => Ok(range_pattern),
                    None => {
                        // Parse as simple literal
                        let literal = self.parse_literal_value()?;
                        Ok(Pattern::Literal(literal))
                    }
                }
            }
            TokenType::LeftBracket => self.parse_list_pattern(),
            TokenType::LeftBrace => self.parse_dict_pattern(),
            _ => Err(anyhow!(
                "Unsupported pattern type: {:?}",
                self.peek().token_type
            )),
        }
    }

    fn parse_list_pattern(&mut self) -> Result<Pattern> {
        self.expect(TokenType::LeftBracket, "Expected '['")?;

        let mut patterns = Vec::new();
        let mut rest_var = None;

        // Handle empty list pattern: []
        if self.accept(TokenType::RightBracket) {
            return Ok(Pattern::List(patterns));
        }

        loop {
            // Check for rest pattern: *name
            if self.accept(TokenType::Star) {
                let rest_token = self.expect(TokenType::Ident, "Expected identifier after '*'")?;
                let rest_name = rest_token
                    .text
                    .ok_or_else(|| anyhow!("Identifier token missing text"))?;
                rest_var = Some(rest_name);

                // After rest pattern, we can only have the closing bracket or comma + closing bracket
                if self.accept(TokenType::Comma) {
                    // Allow trailing comma
                    if !self.check(TokenType::RightBracket) {
                        return Err(anyhow!("Rest pattern must be the last element"));
                    }
                }
                break;
            }

            // Parse regular pattern
            patterns.push(self.parse_pattern()?);

            if self.accept(TokenType::Comma) {
                // Check for trailing comma
                if self.check(TokenType::RightBracket) {
                    break;
                }
                continue;
            } else {
                break;
            }
        }

        self.expect(
            TokenType::RightBracket,
            "Expected ']' to close list pattern",
        )?;

        if rest_var.is_some() {
            Ok(Pattern::ListRest(patterns, rest_var))
        } else {
            Ok(Pattern::List(patterns))
        }
    }

    fn parse_dict_pattern(&mut self) -> Result<Pattern> {
        self.expect(TokenType::LeftBrace, "Expected '{'")?;

        let mut dict_patterns = Vec::new();

        // Handle empty dict pattern: {}
        if self.accept(TokenType::RightBrace) {
            return Ok(Pattern::Dict(dict_patterns));
        }

        loop {
            // Parse key identifier
            let key_token =
                self.expect(TokenType::Ident, "Expected identifier for dict pattern key")?;
            let key = key_token
                .text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;

            let alias = if self.accept(TokenType::Colon) {
                // {key: alias} form
                let alias_token = self.expect(TokenType::Ident, "Expected alias after ':'")?;
                Some(
                    alias_token
                        .text
                        .ok_or_else(|| anyhow!("Alias token missing text"))?,
                )
            } else {
                // {key} form - use key as the variable name
                None
            };

            dict_patterns.push(DictPattern { key, alias });

            if self.accept(TokenType::Comma) {
                // Check for trailing comma
                if self.check(TokenType::RightBrace) {
                    break;
                }
                continue;
            } else {
                break;
            }
        }

        self.expect(TokenType::RightBrace, "Expected '}' to close dict pattern")?;
        Ok(Pattern::Dict(dict_patterns))
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
            let name = name_token
                .text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            LValue::Identifier(name)
        } else {
            return Err(anyhow!("Expected lvalue"));
        };

        // Handle chained property/index access
        loop {
            if self.accept(TokenType::Dot) {
                let property_token =
                    self.expect(TokenType::Ident, "Expected property name after '.'")?;
                let property = property_token
                    .text
                    .ok_or_else(|| anyhow!("Identifier token missing text"))?;

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
        self.with_checkpoint(|parser| {
            let is_pub = parser.accept(TokenType::Pub);

            if !parser.accept(TokenType::Fn) {
                return Ok(None);
            }

            Ok(Some(parser.parse_function_body(is_pub)?))
        })
    }

    fn parse_function_body(&mut self, is_pub: bool) -> Result<Statement> {
        let name_token = self.expect(TokenType::Ident, "Expected function name")?;
        let name = name_token
            .text
            .ok_or_else(|| anyhow!("Function name token missing text"))?;

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

        Ok(Statement::FnDecl {
            name,
            params,
            body,
            is_pub,
        })
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
        let name = name_token
            .text
            .ok_or_else(|| anyhow!("Parameter name token missing text"))?;

        // Check for default value
        let default = if self.accept(TokenType::Equal) {
            Some(self.parse_literal_value()?)
        } else {
            None
        };

        Ok(Parameter {
            name,
            default,
            kind,
        })
    }

    pub fn try_parse_class_declaration(&mut self) -> Result<Option<Statement>> {
        self.with_checkpoint(|parser| {
            let is_pub = parser.accept(TokenType::Pub);

            if !parser.accept(TokenType::Class) {
                return Ok(None);
            }

            Ok(Some(parser.parse_class_body(is_pub)?))
        })
    }

    fn parse_class_body(&mut self, is_pub: bool) -> Result<Statement> {
        let name_token = self.expect(TokenType::Ident, "Expected class name")?;
        let name = name_token
            .text
            .ok_or_else(|| anyhow!("Class name token missing text"))?;

        self.expect(TokenType::LeftBrace, "Expected '{' after class name")?;

        let mut members = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(member) = self.parse_class_member()? {
                members.push(member);
            }
        }

        self.expect(TokenType::RightBrace, "Expected '}' after class body")?;

        Ok(Statement::ClassDecl {
            name,
            members,
            is_pub,
        })
    }

    fn parse_class_member(&mut self) -> Result<Option<ClassMember>> {
        // Check for static methods
        if self.accept(TokenType::Static) {
            self.expect(TokenType::Fn, "Expected 'fn' after 'static'")?;
            let name_token = self.expect(TokenType::Ident, "Expected method name")?;
            let name = name_token
                .text
                .ok_or_else(|| anyhow!("Method name token missing text"))?;

            self.expect(TokenType::LeftParen, "Expected '(' after method name")?;
            let mut params = Vec::new();

            // Parse parameters
            while !self.check(TokenType::RightParen) && !self.is_at_end() {
                params.push(self.parse_parameter()?);
                if !self.accept(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::RightParen, "Expected ')' after parameters")?;

            self.expect(TokenType::LeftBrace, "Expected '{' after method signature")?;
            let body = self.parse_block()?;

            return Ok(Some(ClassMember {
                name,
                kind: ClassMemberKind::StaticMethod { params, body },
            }));
        }

        // Check for regular methods
        if self.accept(TokenType::Fn) {
            let name_token = self.expect(TokenType::Ident, "Expected method name")?;
            let name = name_token
                .text
                .ok_or_else(|| anyhow!("Method name token missing text"))?;

            self.expect(TokenType::LeftParen, "Expected '(' after method name")?;
            let mut params = Vec::new();

            // Parse parameters
            while !self.check(TokenType::RightParen) && !self.is_at_end() {
                params.push(self.parse_parameter()?);
                if !self.accept(TokenType::Comma) {
                    break;
                }
            }
            self.expect(TokenType::RightParen, "Expected ')' after parameters")?;

            self.expect(TokenType::LeftBrace, "Expected '{' after method signature")?;
            let body = self.parse_block()?;

            return Ok(Some(ClassMember {
                name,
                kind: ClassMemberKind::Method { params, body },
            }));
        }

        // Check for field declarations
        if self.accept(TokenType::Var) {
            let name_token = self.expect(TokenType::Ident, "Expected field name")?;
            let name = name_token
                .text
                .ok_or_else(|| anyhow!("Field name token missing text"))?;

            let initializer = if self.accept(TokenType::Equal) {
                Some(self.parse_expression()?)
            } else {
                None
            };

            self.expect(TokenType::Semicolon, "Expected ';' after field declaration")?;

            return Ok(Some(ClassMember {
                name,
                kind: ClassMemberKind::Field(initializer),
            }));
        }

        // No valid member found
        Ok(None)
    }

    pub fn try_parse_import_statement(&mut self) -> Result<Option<Statement>> {
        if !self.accept(TokenType::Use) {
            return Ok(None);
        }

        // Parse module path (e.g., "std::io")
        let mut module_parts = Vec::new();

        let first_part = self.expect(TokenType::Ident, "Expected module name")?;
        module_parts.push(
            first_part
                .text
                .ok_or_else(|| anyhow!("Module name token missing text"))?,
        );

        while self.accept(TokenType::DoubleColon) {
            // Check if this is the final :: before import items
            if self.check(TokenType::Star)
                || self.check(TokenType::LeftBrace)
                || self.check(TokenType::Ident)
            {
                // This is the final :: before import items
                break;
            } else {
                // This is part of the module path
                let part = self.expect(TokenType::Ident, "Expected module name after '::'")?;
                module_parts.push(
                    part.text
                        .ok_or_else(|| anyhow!("Module name token missing text"))?,
                );
            }
        }

        let module = module_parts.join("::");

        let items = if self.accept(TokenType::Star) {
            // use module::*
            ImportItems::All
        } else if self.accept(TokenType::LeftBrace) {
            // use module::{item1, item2 as alias}
            let mut import_items = Vec::new();

            loop {
                if self.check(TokenType::RightBrace) {
                    break;
                }

                let name_token = self.expect(TokenType::Ident, "Expected import item name")?;
                let name = name_token
                    .text
                    .ok_or_else(|| anyhow!("Import item name token missing text"))?;

                let alias = if self.accept(TokenType::As) {
                    let alias_token = self.expect(TokenType::Ident, "Expected alias after 'as'")?;
                    Some(
                        alias_token
                            .text
                            .ok_or_else(|| anyhow!("Alias token missing text"))?,
                    )
                } else {
                    None
                };

                import_items.push(ImportItem { name, alias });

                if !self.accept(TokenType::Comma) {
                    break;
                }
            }

            self.expect(TokenType::RightBrace, "Expected '}' after import items")?;
            ImportItems::Specific(import_items)
        } else {
            // use module::item
            let name_token = self.expect(TokenType::Ident, "Expected import item name")?;
            let name = name_token
                .text
                .ok_or_else(|| anyhow!("Import item name token missing text"))?;

            let alias = if self.accept(TokenType::As) {
                let alias_token = self.expect(TokenType::Ident, "Expected alias after 'as'")?;
                Some(
                    alias_token
                        .text
                        .ok_or_else(|| anyhow!("Alias token missing text"))?,
                )
            } else {
                None
            };

            ImportItems::Specific(vec![ImportItem { name, alias }])
        };

        self.expect(TokenType::Semicolon, "Expected ';' after import statement")?;

        Ok(Some(Statement::Import(ImportSpec { module, items })))
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

        self.expect(
            TokenType::Semicolon,
            "Expected ';' after continue statement",
        )?;
        Ok(Some(Statement::Continue))
    }

    pub fn try_parse_block_like_expression_statement(&mut self) -> Result<Option<Statement>> {
        // Check for block-like expressions that don't require semicolons
        if self.check(TokenType::If)
            || self.check(TokenType::Loop)
            || self.check(TokenType::While)
            || self.check(TokenType::For)
            || self.check(TokenType::Match)
            || self.check(TokenType::Try)
            || self.check(TokenType::With)
        {
            // Parse as expression and wrap in Statement::Expression
            if let Ok(expr) = self.parse_expression() {
                return Ok(Some(Statement::Expression(expr)));
            }
        }

        // Try to parse a block expression (will backtrack if it's actually a dictionary)
        if self.check(TokenType::LeftBrace) {
            if let Some(expr) = self.with_checkpoint(|parser| {
                let expr = parser.parse_expression()?;
                // Check if this parsed as a block (not a dictionary)
                if matches!(expr, Expression::Block(_)) {
                    Ok(Some(expr))
                } else {
                    // It was a dictionary - signal to backtrack
                    Ok(None)
                }
            })? {
                return Ok(Some(Statement::Expression(expr)));
            }
        }

        Ok(None)
    }
}
