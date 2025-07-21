/// Parser implementation for RustLeaf
use crate::core::*;
use crate::lexer::Token;
use anyhow::{anyhow, Result};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Parse source code string directly into an AST
    pub fn parse_str(source: &str) -> Result<Program> {
        let tokens = crate::lexer::Lexer::tokenize(source)?;
        Self::parse(tokens)
    }

    /// Parse tokens into an AST
    pub fn parse(tokens: Vec<Token>) -> Result<Program> {
        let mut parser = Self::new(tokens);
        parser.parse_program()
    }

    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program(statements))
    }

    // ===== Statement Parsing =====

    fn parse_statement(&mut self) -> Result<Statement> {
        // Parse macro annotations first
        let macros = self.parse_macro_annotations()?;
        
        match self.peek() {
            Token::Var => self.parse_var_declaration(macros),
            Token::Fn => self.parse_function_declaration(macros),
            Token::Pub => {
                self.advance(); // consume 'pub'
                match self.peek() {
                    Token::Fn => self.parse_function_declaration_pub(macros),
                    Token::Class => self.parse_class_declaration_pub(macros),
                    _ => Err(anyhow!("Expected function or class after 'pub'")),
                }
            }
            Token::Class => self.parse_class_declaration(macros),
            Token::Use => self.parse_import_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Break => self.parse_break_statement(),
            Token::Continue => self.parse_continue_statement(),
            _ => {
                if !macros.is_empty() {
                    return Err(anyhow!("Macros can only be applied to declarations"));
                }
                // Check if this looks like an assignment
                if self.is_assignment() {
                    self.parse_assignment()
                } else {
                    // Expression statement
                    let expr = self.parse_expression()?;
                    self.consume_semicolon()?;
                    Ok(Statement::Expression(expr))
                }
            }
        }
    }

    fn parse_macro_annotations(&mut self) -> Result<Vec<MacroAnnotation>> {
        let mut macros = Vec::new();
        
        while self.check(&Token::Hash) {
            self.advance(); // consume '#'
            self.consume(&Token::LeftBracket, "Expected '[' after '#'")?;
            
            let name = self.consume_identifier("Expected macro name")?;
            let mut args = Vec::new();
            
            if self.check(&Token::LeftParen) {
                self.advance(); // consume '('
                
                while !self.check(&Token::RightParen) && !self.is_at_end() {
                    if let Some(arg) = self.parse_macro_arg()? {
                        args.push(arg);
                    }
                    
                    if !self.check(&Token::RightParen) {
                        self.consume(&Token::Comma, "Expected ',' between macro arguments")?;
                    }
                }
                
                self.consume(&Token::RightParen, "Expected ')' after macro arguments")?;
            }
            
            self.consume(&Token::RightBracket, "Expected ']' after macro")?;
            
            macros.push(MacroAnnotation { name, args });
        }
        
        Ok(macros)
    }

    fn parse_macro_arg(&mut self) -> Result<Option<MacroArg>> {
        match self.peek() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                
                if self.check(&Token::Colon) {
                    self.advance(); // consume ':'
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

    fn parse_var_declaration(&mut self, macros: Vec<MacroAnnotation>) -> Result<Statement> {
        self.advance(); // consume 'var'
        let pattern = self.parse_pattern()?;
        
        let value = if self.check(&Token::Equal) {
            self.advance(); // consume '='
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        self.consume_semicolon()?;
        
        Ok(Statement::VarDecl {
            pattern,
            value,
            macros,
        })
    }

    fn is_assignment(&mut self) -> bool {
        // Look ahead to see if this is an assignment
        let saved_current = self.current;
        
        // Try to parse an lvalue
        let is_assign = if self.parse_lvalue_lookahead() {
            self.is_assign_op()
        } else {
            false
        };
        
        self.current = saved_current;
        is_assign
    }

    fn parse_lvalue_lookahead(&mut self) -> bool {
        match self.peek() {
            Token::Ident(_) => {
                self.advance();
                // Could be obj.field or obj[key]
                while self.check(&Token::Dot) || self.check(&Token::LeftBracket) {
                    if self.check(&Token::Dot) {
                        self.advance();
                        if !matches!(self.peek(), Token::Ident(_)) {
                            return false;
                        }
                        self.advance();
                    } else {
                        self.advance(); // consume '['
                        // Skip the index expression
                        let mut bracket_depth = 1;
                        while bracket_depth > 0 && !self.is_at_end() {
                            match self.peek() {
                                Token::LeftBracket => bracket_depth += 1,
                                Token::RightBracket => bracket_depth -= 1,
                                _ => {}
                            }
                            self.advance();
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn is_assign_op(&self) -> bool {
        matches!(
            self.peek(),
            Token::Equal
                | Token::PlusEqual
                | Token::MinusEqual
                | Token::StarEqual
                | Token::SlashEqual
                | Token::PercentEqual
        )
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        // TODO: Implement expression parsing with precedence
        // This is where operator desugaring happens:
        // - Binary operators → MethodCall(GetAttr(...))
        // - Property access → GetAttr
        // - Array access → MethodCall(GetAttr(..., "op_get_item"))
        // - Function calls → MethodCall

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression> {
        match &self.tokens[self.current] {
            Token::True => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Bool(true)))
            }
            Token::False => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Bool(false)))
            }
            Token::Null => {
                self.advance();
                Ok(Expression::Literal(LiteralValue::Null))
            }
            Token::Int(n) => {
                let n = *n;
                self.advance();
                Ok(Expression::Literal(LiteralValue::Int(n)))
            }
            Token::Float(f) => {
                let f = *f;
                self.advance();
                Ok(Expression::Literal(LiteralValue::Float(f)))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(Expression::Literal(LiteralValue::String(s)))
            }
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expression::Identifier(name))
            }
            _ => Err(anyhow::anyhow!(
                "Unexpected token: {:?}",
                self.tokens[self.current]
            )),
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // ===== Helper Functions =====

    fn check(&self, token_type: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
        }
    }

    fn match_token(&mut self, token_types: &[Token]) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, token_type: &Token, message: &str) -> Result<()> {
        if self.check(token_type) {
            self.advance();
            Ok(())
        } else {
            Err(anyhow!("{}: expected {:?}, found {:?}", message, token_type, self.peek()))
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String> {
        match self.peek() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(anyhow!("{}: expected identifier, found {:?}", message, self.peek())),
        }
    }

    fn consume_semicolon(&mut self) -> Result<()> {
        self.consume(&Token::Semicolon, "Expected ';'")
    }

    fn parse_literal_value(&mut self) -> Result<LiteralValue> {
        match self.peek() {
            Token::True => {
                self.advance();
                Ok(LiteralValue::Bool(true))
            }
            Token::False => {
                self.advance();
                Ok(LiteralValue::Bool(false))
            }
            Token::Null => {
                self.advance();
                Ok(LiteralValue::Null)
            }
            Token::Int(n) => {
                let n = *n;
                self.advance();
                Ok(LiteralValue::Int(n))
            }
            Token::Float(f) => {
                let f = *f;
                self.advance();
                Ok(LiteralValue::Float(f))
            }
            Token::String(s) => {
                let s = s.clone();
                self.advance();
                Ok(LiteralValue::String(s))
            }
            Token::RawString(s) => {
                let s = s.clone();
                self.advance();
                Ok(LiteralValue::RawString(s))
            }
            _ => Err(anyhow!("Expected literal value, found {:?}", self.peek())),
        }
    }

    // ===== Stub Functions =====
    // These will be implemented next

    fn parse_pattern(&mut self) -> Result<Pattern> {
        match self.peek() {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Pattern::Variable(name))
            }
            _ => Err(anyhow!("Pattern parsing not fully implemented yet")),
        }
    }

    fn parse_assignment(&mut self) -> Result<Statement> {
        Err(anyhow!("Assignment parsing not implemented yet"))
    }

    fn parse_function_declaration(&mut self, _macros: Vec<MacroAnnotation>) -> Result<Statement> {
        Err(anyhow!("Function declaration parsing not implemented yet"))
    }

    fn parse_function_declaration_pub(&mut self, _macros: Vec<MacroAnnotation>) -> Result<Statement> {
        Err(anyhow!("Public function declaration parsing not implemented yet"))
    }

    fn parse_class_declaration(&mut self, _macros: Vec<MacroAnnotation>) -> Result<Statement> {
        Err(anyhow!("Class declaration parsing not implemented yet"))
    }

    fn parse_class_declaration_pub(&mut self, _macros: Vec<MacroAnnotation>) -> Result<Statement> {
        Err(anyhow!("Public class declaration parsing not implemented yet"))
    }

    fn parse_import_statement(&mut self) -> Result<Statement> {
        Err(anyhow!("Import statement parsing not implemented yet"))
    }

    fn parse_return_statement(&mut self) -> Result<Statement> {
        Err(anyhow!("Return statement parsing not implemented yet"))
    }

    fn parse_break_statement(&mut self) -> Result<Statement> {
        Err(anyhow!("Break statement parsing not implemented yet"))
    }

    fn parse_continue_statement(&mut self) -> Result<Statement> {
        Err(anyhow!("Continue statement parsing not implemented yet"))
    }
}
