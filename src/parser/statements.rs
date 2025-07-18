use crate::lexer::{LiteralValue, TokenType};
use crate::parser::ast::*;
use crate::parser::Parser;

impl Parser {
    pub fn parse_statement(&mut self) -> Option<AstNode> {
        match self.peek().token_type {
            TokenType::LeftBrace => self.parse_block(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Match => self.parse_match_statement(),
            TokenType::Try => self.parse_try_statement(),
            TokenType::With => self.parse_with_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Var => self.parse_variable_declaration(),
            TokenType::Fn => {
                // Function declarations inside blocks
                let location = self.current_location();
                self.consume(TokenType::Fn, "Expected 'fn'")?;
                let name = self.consume_identifier("Expected function name")?;

                self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
                let parameters = self.parse_parameter_list();
                self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

                let body = self.parse_block()?;

                Some(AstNode::FunctionDeclaration {
                    visibility: Visibility::Private,
                    name,
                    parameters,
                    body: Box::new(body),
                    location,
                })
            }
            TokenType::Semicolon => {
                self.advance(); // consume semicolon
                None // Empty statement
            }
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_statement_in_block(&mut self) -> Option<AstNode> {
        match self.peek().token_type {
            TokenType::LeftBrace => self.parse_block(),
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            TokenType::For => self.parse_for_statement(),
            TokenType::Match => self.parse_match_statement(),
            TokenType::Try => self.parse_try_statement(),
            TokenType::With => self.parse_with_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Var => self.parse_variable_declaration(),
            TokenType::Fn => {
                // Check if this is a function declaration (has identifier after fn)
                // or anonymous function (has left paren after fn)
                if self.check_ahead(1, &TokenType::Identifier) {
                    // Function declaration inside blocks
                    let location = self.current_location();
                    self.consume(TokenType::Fn, "Expected 'fn'")?;
                    let name = self.consume_identifier("Expected function name")?;

                    self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
                    let parameters = self.parse_parameter_list();
                    self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

                    let body = self.parse_block()?;

                    Some(AstNode::FunctionDeclaration {
                        visibility: Visibility::Private,
                        name,
                        parameters,
                        body: Box::new(body),
                        location,
                    })
                } else {
                    // Anonymous function - parse as expression
                    let saved_pos = self.current;
                    if let Some(expr) = self.parse_expression() {
                        // If there's a semicolon, treat as expression statement
                        if self.check(&TokenType::Semicolon) {
                            self.advance(); // consume semicolon
                            Some(AstNode::ExpressionStatement {
                                expression: Box::new(expr),
                                location: self.current_location(),
                            })
                        } else {
                            // No semicolon - check if we're at the end of block
                            self.skip_newlines();
                            if self.check(&TokenType::RightBrace) {
                                // Final expression without semicolon
                                Some(expr)
                            } else {
                                // Not at end, this is an error - restore and use normal parsing
                                self.current = saved_pos;
                                self.parse_expression_statement()
                            }
                        }
                    } else {
                        None
                    }
                }
            }
            TokenType::Semicolon => {
                self.advance(); // consume semicolon
                None // Empty statement
            }
            _ => {
                // For expressions in blocks, check if this might be the final expression
                let saved_pos = self.current;
                if let Some(expr) = self.parse_expression() {
                    // If there's a semicolon, treat as expression statement
                    if self.check(&TokenType::Semicolon) {
                        self.advance(); // consume semicolon
                        Some(AstNode::ExpressionStatement {
                            expression: Box::new(expr),
                            location: self.current_location(),
                        })
                    } else {
                        // No semicolon - check if we're at the end of block
                        self.skip_newlines();
                        if self.check(&TokenType::RightBrace) {
                            // Final expression without semicolon
                            Some(expr)
                        } else {
                            // Not at end, this is an error - restore and use normal parsing
                            self.current = saved_pos;
                            self.parse_expression_statement()
                        }
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_block(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::LeftBrace, "Expected '{'")?;

        let mut statements = Vec::new();

        self.skip_newlines();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let position_before = self.current;

            // Try to parse statement, but handle final expression specially
            if let Some(stmt) = self.parse_statement_in_block() {
                statements.push(stmt);
            }

            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!(
                    "Parser stuck in block: no progress made at position {}",
                    self.current
                );
            }

            self.skip_newlines();
        }

        self.consume(TokenType::RightBrace, "Expected '}'")?;

        Some(AstNode::Block {
            statements,
            location,
        })
    }

    pub fn parse_if_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::If, "Expected 'if'")?;

        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_block()?);

        let mut else_ifs = Vec::new();

        while self.check(&TokenType::Else) && self.check_ahead(1, &TokenType::If) {
            self.advance(); // consume 'else'
            self.advance(); // consume 'if'
            let else_if_condition = self.parse_expression()?;
            let else_if_body = self.parse_block()?;
            else_ifs.push((else_if_condition, else_if_body));
        }

        // Skip newlines before looking for else clause
        self.skip_newlines();
        let else_branch = if self.match_token(&TokenType::Else) {
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Some(AstNode::If {
            condition,
            then_branch,
            else_ifs,
            else_branch,
            location,
        })
    }

    pub fn parse_while_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::While, "Expected 'while'")?;

        let condition = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_block()?);

        Some(AstNode::WhileStatement {
            condition,
            body,
            location,
        })
    }

    pub fn parse_for_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::For, "Expected 'for'")?;

        let variable = self.consume_identifier("Expected loop variable")?;

        let index_variable = if self.match_token(&TokenType::Comma) {
            Some(self.consume_identifier("Expected index variable")?)
        } else {
            None
        };

        self.consume(TokenType::In, "Expected 'in'")?;
        let iterable = Box::new(self.parse_expression()?);
        let body = Box::new(self.parse_block()?);

        Some(AstNode::ForStatement {
            variable,
            index_variable,
            iterable,
            body,
            location,
        })
    }

    pub fn parse_match_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Match, "Expected 'match'")?;

        let value = Box::new(self.parse_expression()?);

        self.consume(TokenType::LeftBrace, "Expected '{' after match value")?;

        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let position_before = self.current;

            if let Some(arm) = self.parse_match_arm() {
                arms.push(arm);
            }

            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!(
                    "Parser stuck in match arms: no progress made at position {}",
                    self.current
                );
            }

            self.skip_newlines();
        }

        self.consume(TokenType::RightBrace, "Expected '}' after match arms")?;

        Some(AstNode::MatchStatement {
            value,
            arms,
            location,
        })
    }

    pub fn parse_match_arm(&mut self) -> Option<MatchArm> {
        self.consume(TokenType::Case, "Expected 'case'")?;
        let pattern = self.parse_pattern()?;

        let guard = if self.match_token(&TokenType::If) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Some(MatchArm {
            pattern,
            guard,
            body,
        })
    }

    pub fn parse_pattern(&mut self) -> Option<Pattern> {
        match &self.peek().token_type {
            TokenType::IntegerLiteral
            | TokenType::FloatLiteral
            | TokenType::StringLiteral
            | TokenType::BooleanLiteral
            | TokenType::NullLiteral => {
                let token = self.advance();
                token.value.map(Pattern::Literal)
            }
            TokenType::True => {
                self.advance();
                Some(Pattern::Literal(LiteralValue::Boolean(true)))
            }
            TokenType::False => {
                self.advance();
                Some(Pattern::Literal(LiteralValue::Boolean(false)))
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme;
                Some(Pattern::Variable(name))
            }
            TokenType::LeftBracket => {
                self.advance(); // consume '['
                let mut patterns = Vec::new();

                while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
                    let position_before = self.current;

                    if let Some(pattern) = self.parse_pattern() {
                        patterns.push(pattern);
                    }

                    // Infinite loop protection: ensure we always make progress
                    if self.current == position_before {
                        panic!(
                            "Parser stuck in array pattern: no progress made at position {}",
                            self.current
                        );
                    }

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }

                self.consume(TokenType::RightBracket, "Expected ']'")?;
                Some(Pattern::List(patterns))
            }
            TokenType::LeftBrace => {
                self.advance(); // consume '{'
                let mut entries = Vec::new();

                while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
                    let position_before = self.current;

                    if let Some(key_token) = self.match_string_literal() {
                        self.consume(TokenType::Colon, "Expected ':' after dict key")?;
                        if let Some(pattern) = self.parse_pattern() {
                            entries.push((key_token, pattern));
                        }
                    }

                    // Infinite loop protection: ensure we always make progress
                    if self.current == position_before {
                        panic!(
                            "Parser stuck in dict pattern: no progress made at position {}",
                            self.current
                        );
                    }

                    if !self.match_token(&TokenType::Comma) {
                        break;
                    }
                }

                self.consume(TokenType::RightBrace, "Expected '}'")?;
                Some(Pattern::Dict(entries))
            }
            _ => None,
        }
    }

    pub fn parse_try_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Try, "Expected 'try'")?;

        let body = Box::new(self.parse_block()?);

        let catch_clause = if self.match_token(&TokenType::Catch) {
            let variable = self.consume_identifier("Expected catch variable")?;
            let catch_body = self.parse_block()?;
            Some(CatchClause {
                variable,
                body: Box::new(catch_body),
            })
        } else {
            None
        };

        let finally_clause = if self.match_token(&TokenType::Finally) {
            Some(Box::new(self.parse_block()?))
        } else {
            None
        };

        Some(AstNode::TryStatement {
            body,
            catch_clause,
            finally_clause,
            location,
        })
    }

    pub fn parse_with_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::With, "Expected 'with'")?;

        let mut bindings = Vec::new();

        loop {
            let name = self.consume_identifier("Expected binding name")?;
            self.consume(TokenType::Equal, "Expected '=' after binding name")?;
            let value = self.parse_expression()?;

            bindings.push(WithBinding { name, value });

            if !self.match_token(&TokenType::Comma) {
                break;
            }
        }

        let body = Box::new(self.parse_block()?);

        Some(AstNode::WithStatement {
            bindings,
            body,
            location,
        })
    }

    pub fn parse_break_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Break, "Expected 'break'")?;

        let value = if !self.check(&TokenType::Semicolon) && !self.check(&TokenType::Newline) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after break statement")?;

        Some(AstNode::BreakStatement { value, location })
    }

    pub fn parse_continue_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Continue, "Expected 'continue'")?;
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after continue statement",
        )?;

        Some(AstNode::ContinueStatement { location })
    }

    pub fn parse_return_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Return, "Expected 'return'")?;

        let value = if !self.check(&TokenType::Semicolon) && !self.check(&TokenType::Newline) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return statement")?;

        Some(AstNode::ReturnStatement { value, location })
    }

    pub fn parse_expression_statement(&mut self) -> Option<AstNode> {
        self.parse_expression_statement_with_semicolon(true)
    }

    pub fn parse_expression_statement_with_semicolon(
        &mut self,
        require_semicolon: bool,
    ) -> Option<AstNode> {
        let location = self.current_location();
        let expression = Box::new(self.parse_expression()?);

        if require_semicolon {
            self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        } else if self.check(&TokenType::Semicolon) {
            self.advance(); // Consume optional semicolon
        }

        Some(AstNode::ExpressionStatement {
            expression,
            location,
        })
    }
}
