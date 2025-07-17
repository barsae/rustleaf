use crate::lexer::{LiteralValue, TokenType};
use crate::parser::ast::*;
use crate::parser::Parser;

impl Parser {
    pub fn parse_module_item(&mut self) -> Option<AstNode> {
        match self.peek().token_type {
            TokenType::Use => self.parse_import_statement(),
            TokenType::Fn => self.parse_function_declaration(Visibility::Private),
            TokenType::Pub => {
                self.advance(); // consume 'pub'
                match self.peek().token_type {
                    TokenType::Fn => self.parse_function_declaration(Visibility::Public),
                    TokenType::Class => self.parse_class_declaration(Visibility::Public),
                    _ => {
                        self.error("Expected function or class declaration after 'pub'");
                        None
                    }
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

        let path = vec![self.consume_identifier("Expected module name")?];

        if self.match_token(&TokenType::DoubleColon) {
            if self.match_token(&TokenType::Star) {
                let clause = ImportClause::All;
                self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
                return Some(AstNode::ImportStatement {
                    path,
                    clause: Some(clause),
                    location,
                });
            } else if self.check(&TokenType::LeftBrace) {
                self.advance(); // consume '{'
                let mut names = Vec::new();
                names.push(self.consume_identifier("Expected identifier")?);

                while self.match_token(&TokenType::Comma) {
                    if self.check(&TokenType::RightBrace) {
                        break;
                    }
                    names.push(self.consume_identifier("Expected identifier")?);
                }

                self.consume(TokenType::RightBrace, "Expected '}'")?;
                let clause = ImportClause::Named(names);
                self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
                return Some(AstNode::ImportStatement {
                    path,
                    clause: Some(clause),
                    location,
                });
            } else {
                let name = self.consume_identifier("Expected identifier")?;
                let clause = ImportClause::Single(name);
                self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
                return Some(AstNode::ImportStatement {
                    path,
                    clause: Some(clause),
                    location,
                });
            }
        }

        self.consume(TokenType::Semicolon, "Expected ';' after import statement")?;
        Some(AstNode::ImportStatement {
            path,
            clause: None,
            location,
        })
    }

    pub fn parse_function_declaration(&mut self, visibility: Visibility) -> Option<AstNode> {
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
        let location = self.current_location();
        self.consume(TokenType::Class, "Expected 'class'")?;
        let name = self.consume_identifier("Expected class name")?;

        self.consume(TokenType::LeftBrace, "Expected '{' after class name")?;

        let mut members = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let position_before = self.current;

            if let Some(member) = self.parse_class_member() {
                members.push(member);
            }

            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!(
                    "Parser stuck in class members: no progress made at position {}",
                    self.current
                );
            }

            self.skip_newlines();
        }

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
                    self.parse_function_declaration(member_visibility.clone())
                {
                    return Some(ClassMember::Method {
                        visibility: member_visibility,
                        is_static: true,
                        declaration,
                    });
                }
            }
        } else if self.check(&TokenType::Fn) {
            if let Some(declaration) = self.parse_function_declaration(member_visibility.clone()) {
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

    pub fn parse_variable_declaration(&mut self) -> Option<AstNode> {
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
            name,
            value,
            location,
        })
    }
}
