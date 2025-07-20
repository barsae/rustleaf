use crate::lexer::{LiteralValue, TokenType};
use crate::parser::ast::*;
use crate::parser::Parser;

impl Parser {
    pub fn parse_module_item(&mut self) -> Option<AstNode> {
        // Skip comments
        if self.peek().token_type == TokenType::Comment {
            self.advance();
            return None;
        }

        match self.peek().token_type {
            TokenType::Use => self.parse_import_statement(),
            TokenType::Fn => self.parse_function_declaration(Visibility::Private),
            TokenType::Pub => {
                self.advance(); // consume 'pub'
                match self.peek().token_type {
                    TokenType::Fn => self.parse_function_declaration(Visibility::Public),
                    TokenType::Class => self.parse_class_declaration(Visibility::Public),
                    TokenType::Var => {
                        self.parse_variable_declaration_with_visibility(Visibility::Public)
                    }
                    _ => None,
                }
            }
            TokenType::Class => self.parse_class_declaration(Visibility::Private),
            TokenType::Var => self.parse_variable_declaration(),
            // Tokens that are clearly not valid at module level - skip them
            TokenType::RightBrace | TokenType::RightBracket | TokenType::RightParen => None,
            _ => self.parse_statement(),
        }
    }

    pub fn parse_import_statement(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Use, "Expected 'use'")?;

        let (path, clause) = self.parse_import_path_and_clause()?;

        self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
        Some(AstNode::ImportStatement {
            path,
            clause,
            location,
        })
    }

    fn parse_import_path_and_clause(&mut self) -> Option<(ModulePath, Option<ImportClause>)> {
        // Parse the module path root (super, root, or absolute)
        let (root_type, first_segment) = if self.match_token(&TokenType::Super) {
            self.consume(TokenType::DoubleColon, "Expected '::' after 'super'")?;
            let segment = self.consume_identifier("Expected module name after 'super::'")?;
            (ModulePathRoot::Super, segment)
        } else {
            let segment = self.consume_identifier("Expected module name")?;
            (ModulePathRoot::Relative, segment)
        };

        let mut path_segments = vec![first_segment];

        // Parse path segments until we hit import clause syntax or end
        while self.check(&TokenType::DoubleColon) {
            // Look ahead to see what follows the ::
            let saved_pos = self.current;
            self.advance(); // consume ::

            if self.check(&TokenType::Star) {
                // This is ::* syntax
                self.advance(); // consume *
                let path = ModulePath {
                    root_type,
                    segments: path_segments,
                };
                return Some((path, Some(ImportClause::All)));
            } else if self.check(&TokenType::LeftBrace) {
                // This is ::{...} syntax
                self.advance(); // consume {
                let mut items = Vec::new();
                items.push(self.parse_import_item()?);

                while self.match_token(&TokenType::Comma) {
                    if self.check(&TokenType::RightBrace) {
                        break;
                    }
                    items.push(self.parse_import_item()?);
                }

                self.consume(TokenType::RightBrace, "Expected '}'")?;
                let path = ModulePath {
                    root_type,
                    segments: path_segments,
                };
                return Some((path, Some(ImportClause::Named(items))));
            } else if self.check(&TokenType::Identifier) {
                let next_segment = self.consume_identifier("Expected identifier")?;

                // Check if there's another :: after this identifier
                if self.check(&TokenType::DoubleColon) {
                    // This identifier is part of the path, continue parsing
                    path_segments.push(next_segment);
                    continue;
                } else {
                    // This identifier is a single import clause (import specific item)
                    let path = ModulePath {
                        root_type,
                        segments: path_segments,
                    };
                    return Some((path, Some(ImportClause::Single(next_segment))));
                }
            } else {
                // Invalid syntax, restore position and break
                self.current = saved_pos;
                break;
            }
        }

        // No import clause, just the module path
        let path = ModulePath {
            root_type,
            segments: path_segments,
        };
        Some((path, None))
    }

    fn parse_import_item(&mut self) -> Option<ImportItem> {
        let name = self.consume_identifier("Expected identifier")?;

        // Check for "as" alias
        let alias = if self.check(&TokenType::Identifier) && self.peek().lexeme == "as" {
            self.advance(); // consume "as"
            Some(self.consume_identifier("Expected alias name after 'as'")?)
        } else {
            None
        };

        Some(ImportItem { name, alias })
    }

    pub fn parse_variable_declaration(&mut self) -> Option<AstNode> {
        let visibility = self.try_parse_visibility().unwrap_or(Visibility::Private);

        if !self.check(&TokenType::Var) {
            return None;
        }

        let location = self.current_location();
        self.consume(TokenType::Var, "Expected 'var'")?;
        let name = self.consume_identifier("Expected variable name")?;

        let value = if self.match_token(&TokenType::Equal) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;

        Some(AstNode::VariableDeclaration {
            visibility,
            name,
            value,
            location,
        })
    }

    pub fn parse_variable_declaration_with_visibility(
        &mut self,
        visibility: Visibility,
    ) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Var, "Expected 'var'")?;
        let name = self.consume_identifier("Expected variable name")?;

        let value = if self.match_token(&TokenType::Equal) {
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration",
        )?;

        Some(AstNode::VariableDeclaration {
            visibility,
            name,
            value,
            location,
        })
    }

    pub fn parse_function_declaration(&mut self, visibility: Visibility) -> Option<AstNode> {
        if !self.check(&TokenType::Fn) {
            return None;
        }

        let location = self.current_location();
        self.consume(TokenType::Fn, "Expected 'fn'")?;
        let name = self.consume_identifier("Expected function name")?;

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let parameters = self.parse_parameter_list();
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        let body = self.parse_block()?;

        Some(AstNode::FunctionDeclaration {
            visibility,
            name,
            parameters,
            body: Box::new(body),
            location,
        })
    }

    pub fn parse_class_declaration(&mut self, visibility: Visibility) -> Option<AstNode> {
        if !self.check(&TokenType::Class) {
            return None;
        }

        let location = self.current_location();
        self.consume(TokenType::Class, "Expected 'class'")?;
        let name = self.consume_identifier("Expected class name")?;

        self.consume(TokenType::LeftBrace, "Expected '{' after class name")?;

        let members = self.parse_many(|parser| {
            if parser.check(&TokenType::RightBrace) {
                None
            } else {
                parser.skip_newlines();
                parser.parse_class_member()
            }
        });

        self.consume(TokenType::RightBrace, "Expected '}' after class body")?;

        Some(AstNode::ClassDeclaration {
            visibility,
            name,
            members,
            location,
        })
    }

    pub fn parse_class_member(&mut self) -> Option<ClassMember> {
        let member_visibility = if self.match_token(&TokenType::Pub) {
            Visibility::Public
        } else {
            Visibility::Private
        };

        if self.match_token(&TokenType::Static) {
            if self.check(&TokenType::Fn) {
                if let Some(declaration) =
                    self.parse_function_declaration_with_visibility(member_visibility.clone())
                {
                    return Some(ClassMember::Method {
                        visibility: member_visibility,
                        is_static: true,
                        declaration,
                    });
                }
            }
        } else if self.check(&TokenType::Fn) {
            if let Some(declaration) =
                self.parse_function_declaration_with_visibility(member_visibility.clone())
            {
                return Some(ClassMember::Method {
                    visibility: member_visibility,
                    is_static: false,
                    declaration,
                });
            }
        } else if self.check(&TokenType::Var) {
            self.advance(); // consume 'var'
            let name = self.consume_identifier("Expected field name")?;
            let value = if self.match_token(&TokenType::Equal) {
                Some(self.parse_expression()?)
            } else {
                None
            };
            self.consume(TokenType::Semicolon, "Expected ';' after field declaration")?;
            return Some(ClassMember::Field {
                visibility: member_visibility,
                name,
                value,
            });
        }

        None
    }

    // Helper method for parsing function declarations with explicit visibility (for class members)
    fn parse_function_declaration_with_visibility(
        &mut self,
        visibility: Visibility,
    ) -> Option<AstNode> {
        if !self.check(&TokenType::Fn) {
            return None;
        }

        let location = self.current_location();
        self.consume(TokenType::Fn, "Expected 'fn'")?;
        let name = self.consume_identifier("Expected function name")?;

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        let parameters = self.parse_parameter_list();
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;

        let body = self.parse_block()?;

        Some(AstNode::FunctionDeclaration {
            visibility,
            name,
            parameters,
            body: Box::new(body),
            location,
        })
    }

    pub fn parse_parameter_list(&mut self) -> Vec<Parameter> {
        let mut parameters = Vec::new();

        if self.check(&TokenType::RightParen) {
            return parameters;
        }

        loop {
            let mut variadic = false;
            let mut keyword_variadic = false;

            if self.match_token(&TokenType::Star) {
                variadic = true;
            } else if self.match_token(&TokenType::StarStar) {
                keyword_variadic = true;
            }

            let name = match self.consume_identifier("Expected parameter name") {
                Some(name) => name,
                None => break,
            };

            let default_value = if self.match_token(&TokenType::Equal) {
                Some(self.parse_expression().unwrap_or_else(|| {
                    AstNode::Literal(LiteralValue::Null, self.current_location())
                }))
            } else {
                None
            };

            parameters.push(Parameter {
                name,
                default_value,
                variadic,
                keyword_variadic,
            });

            if !self.match_token(&TokenType::Comma) {
                break;
            }

            if self.check(&TokenType::RightParen) {
                break;
            }
        }

        parameters
    }
}
