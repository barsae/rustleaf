use crate::lexer::{TokenType, LiteralValue};
use crate::parser::ast::*;
use crate::parser::Parser;

impl Parser {
    pub fn parse_expression(&mut self) -> Option<AstNode> {
        self.parse_assignment()
    }
    
    pub fn parse_assignment(&mut self) -> Option<AstNode> {
        let expr = self.parse_conditional()?;
        
        if let Some(op) = self.match_assignment_operator() {
            let location = self.current_location();
            let value = Box::new(self.parse_assignment()?);
            return Some(AstNode::Assignment {
                target: Box::new(expr),
                operator: op,
                value,
                location,
            });
        }
        
        Some(expr)
    }
    
    pub fn parse_conditional(&mut self) -> Option<AstNode> {
        let expr = self.parse_logical_or()?;
        
        // Note: Ternary operator ?: not in the grammar, using if expression instead
        Some(expr)
    }

    pub fn parse_exponentiation(&mut self) -> Option<AstNode> {
        let expr = self.parse_unary()?;
        
        if self.match_token(&TokenType::StarStar) {
            let location = self.current_location();
            let right = self.parse_exponentiation()?; // Right associative
            return Some(AstNode::BinaryOp {
                left: Box::new(expr),
                operator: BinaryOperator::Power,
                right: Box::new(right),
                location,
            });
        }
        
        Some(expr)
    }
    
    pub fn parse_unary(&mut self) -> Option<AstNode> {
        if let Some(op) = self.match_unary_operator() {
            let location = self.current_location();
            let operand = Box::new(self.parse_unary()?);
            return Some(AstNode::UnaryOp {
                operator: op,
                operand,
                location,
            });
        }
        
        self.parse_postfix()
    }
    
    pub fn parse_postfix(&mut self) -> Option<AstNode> {
        let mut expr = self.parse_primary()?;
        
        loop {
            match &self.peek().token_type {
                TokenType::Dot => {
                    self.advance();
                    let location = self.current_location();
                    let property = self.consume_identifier("Expected property name")?;
                    expr = AstNode::PropertyAccess {
                        object: Box::new(expr),
                        property,
                        location,
                    };
                }
                TokenType::LeftBracket => {
                    self.advance();
                    let location = self.current_location();
                    let index = Box::new(self.parse_expression()?);
                    self.consume(TokenType::RightBracket, "Expected ']'")?;
                    expr = AstNode::IndexAccess {
                        object: Box::new(expr),
                        index,
                        location,
                    };
                }
                TokenType::LeftParen => {
                    self.advance();
                    let location = self.current_location();
                    let arguments = self.parse_argument_list();
                    self.consume(TokenType::RightParen, "Expected ')'")?;
                    expr = AstNode::FunctionCall {
                        function: Box::new(expr),
                        arguments,
                        location,
                    };
                }
                _ => break,
            }
        }
        
        Some(expr)
    }
    
    pub fn parse_argument_list(&mut self) -> Vec<Argument> {
        let mut arguments = Vec::new();
        
        if self.check(&TokenType::RightParen) {
            return arguments;
        }
        
        loop {
            let mut spread = false;
            let mut keyword_spread = false;
            
            if self.match_token(&TokenType::Star) {
                spread = true;
            } else if self.match_token(&TokenType::StarStar) {
                keyword_spread = true;
            }
            
            if let Some(value) = self.parse_expression() {
                arguments.push(Argument {
                    value,
                    spread,
                    keyword_spread,
                });
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightParen) {
                break;
            }
        }
        
        arguments
    }
    
    pub fn parse_primary(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        
        match &self.peek().token_type {
            TokenType::IntegerLiteral | TokenType::FloatLiteral | 
            TokenType::StringLiteral | TokenType::BooleanLiteral | 
            TokenType::NullLiteral => {
                let token = self.advance();
                token.value.map(|value| AstNode::Literal(value, location))
            }
            TokenType::True => {
                self.advance();
                Some(AstNode::Literal(LiteralValue::Boolean(true), location))
            }
            TokenType::False => {
                self.advance();
                Some(AstNode::Literal(LiteralValue::Boolean(false), location))
            }
            TokenType::Identifier => {
                let name = self.advance().lexeme;
                Some(AstNode::Identifier(name, location))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.consume(TokenType::RightParen, "Expected ')'")?;
                Some(expr)
            }
            TokenType::LeftBracket => {
                self.parse_list_literal()
            }
            TokenType::LeftBrace => {
                // Try to parse as block expression first, fall back to dict literal
                let saved_position = self.current;
                
                // Try parsing as block expression
                if let Some(block) = self.parse_block() {
                    Some(block)
                } else {
                    // Restore position and try as dict literal
                    self.current = saved_position;
                    self.parse_dict_literal()
                }
            }
            TokenType::If => {
                self.parse_if_expression()
            }
            TokenType::Match => {
                self.parse_match_expression()
            }
            TokenType::Try => {
                self.parse_try_expression()
            }
            TokenType::Fn => {
                self.parse_anonymous_function()
            }
            _ => {
                self.error("Unexpected token in expression");
                None
            }
        }
    }
    
    pub fn parse_list_literal(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::LeftBracket, "Expected '['")?;
        
        let mut elements = Vec::new();
        
        while !self.check(&TokenType::RightBracket) && !self.is_at_end() {
            let position_before = self.current;
            
            if let Some(expr) = self.parse_expression() {
                elements.push(expr);
            }
            
            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!("Parser stuck in array literal: no progress made at position {}", self.current);
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightBracket) {
                break;
            }
        }
        
        self.consume(TokenType::RightBracket, "Expected ']'")?;
        
        Some(AstNode::ListLiteral {
            elements,
            location,
        })
    }
    
    pub fn parse_dict_literal(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        
        let mut entries = Vec::new();
        
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let position_before = self.current;
            
            let key = self.parse_expression()?;
            self.consume(TokenType::Colon, "Expected ':' after dict key")?;
            let value = self.parse_expression()?;
            
            entries.push((key, value));
            
            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!("Parser stuck in dict literal: no progress made at position {}", self.current);
            }
            
            if !self.match_token(&TokenType::Comma) {
                break;
            }
            
            if self.check(&TokenType::RightBrace) {
                break;
            }
        }
        
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        
        Some(AstNode::DictLiteral {
            entries,
            location,
        })
    }
    
    pub fn parse_if_expression(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::If, "Expected 'if'")?;
        
        let condition = Box::new(self.parse_expression()?);
        let then_branch = Box::new(self.parse_block()?);
        
        let mut else_ifs = Vec::new();
        
        while self.match_token(&TokenType::Else) && self.check(&TokenType::If) {
            self.advance(); // consume 'if'
            let else_if_condition = self.parse_expression()?;
            let else_if_body = self.parse_block()?;
            else_ifs.push((else_if_condition, else_if_body));
        }
        
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
    
    pub fn parse_match_expression(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Match, "Expected 'match'")?;
        
        let value = Box::new(self.parse_expression()?);
        
        self.consume(TokenType::LeftBrace, "Expected '{'")?;
        
        let mut arms = Vec::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let position_before = self.current;
            
            if let Some(arm) = self.parse_match_arm() {
                arms.push(arm);
            }
            
            // Infinite loop protection: ensure we always make progress
            if self.current == position_before {
                panic!("Parser stuck in match expression arms: no progress made at position {}", self.current);
            }
            
            self.skip_newlines();
        }
        
        self.consume(TokenType::RightBrace, "Expected '}'")?;
        
        Some(AstNode::Match {
            value,
            arms,
            location,
        })
    }
    
    pub fn parse_try_expression(&mut self) -> Option<AstNode> {
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
        
        Some(AstNode::Try {
            body,
            catch_clause: catch_clause.map(Box::new),
            location,
        })
    }
    
    pub fn parse_anonymous_function(&mut self) -> Option<AstNode> {
        let location = self.current_location();
        self.consume(TokenType::Fn, "Expected 'fn'")?;
        
        self.consume(TokenType::LeftParen, "Expected '(' after 'fn'")?;
        let parameters = self.parse_parameter_list();
        self.consume(TokenType::RightParen, "Expected ')'")?;
        
        let body = Box::new(self.parse_block()?);
        
        Some(AstNode::AnonymousFunction {
            parameters,
            body,
            location,
        })
    }

    // Helper methods for matching operators
    pub fn match_assignment_operator(&mut self) -> Option<AssignmentOperator> {
        match &self.peek().token_type {
            TokenType::Equal => { self.advance(); Some(AssignmentOperator::Assign) }
            TokenType::PlusEqual => { self.advance(); Some(AssignmentOperator::AddAssign) }
            TokenType::MinusEqual => { self.advance(); Some(AssignmentOperator::SubtractAssign) }
            TokenType::StarEqual => { self.advance(); Some(AssignmentOperator::MultiplyAssign) }
            TokenType::SlashEqual => { self.advance(); Some(AssignmentOperator::DivideAssign) }
            TokenType::PercentEqual => { self.advance(); Some(AssignmentOperator::ModuloAssign) }
            _ => None,
        }
    }
    
    pub fn match_unary_operator(&mut self) -> Option<UnaryOperator> {
        match &self.peek().token_type {
            TokenType::Plus => { self.advance(); Some(UnaryOperator::Plus) }
            TokenType::Minus => { self.advance(); Some(UnaryOperator::Minus) }
            TokenType::Not => { self.advance(); Some(UnaryOperator::Not) }
            TokenType::Tilde => { self.advance(); Some(UnaryOperator::BitwiseNot) }
            _ => None,
        }
    }
}