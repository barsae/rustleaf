/// Parser implementation for RustLeaf
use crate::core::*;
use crate::lexer::{Token, TokenType};
use anyhow::{anyhow, Result};

type BinaryExprConstructor = fn(Box<Expression>, Box<Expression>) -> Expression;

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

        match self.peek().token_type {
            TokenType::Var => self.parse_var_declaration(macros),
            TokenType::Fn => self.parse_function_declaration(macros),
            TokenType::Pub => {
                self.expect(TokenType::Pub, "Expected 'pub'")?;
                match self.peek().token_type {
                    TokenType::Fn => self.parse_function_declaration_pub(macros),
                    TokenType::Class => self.parse_class_declaration_pub(macros),
                    _ => Err(anyhow!("Expected function or class after 'pub'")),
                }
            }
            TokenType::Class => self.parse_class_declaration(macros),
            TokenType::Use => self.parse_import_statement(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Break => self.parse_break_statement(),
            TokenType::Continue => self.parse_continue_statement(),
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
                    self.expect(TokenType::Semicolon, "Expected ';'")?;
                    Ok(Statement::Expression(expr))
                }
            }
        }
    }

    fn parse_macro_annotations(&mut self) -> Result<Vec<MacroAnnotation>> {
        let mut macros = Vec::new();

        while self.accept(TokenType::Hash).is_some() {
            self.expect(TokenType::LeftBracket, "Expected '[' after '#'")?;

            let name_token = self.expect(TokenType::Ident, "Expected macro name")?;
            let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;
            let mut args = Vec::new();

            if self.accept(TokenType::LeftParen).is_some() {
                while !self.check(TokenType::RightParen) && !self.is_at_end() {
                    if let Some(arg) = self.parse_macro_arg()? {
                        args.push(arg);
                    }

                    if !self.check(TokenType::RightParen) {
                        self.expect(TokenType::Comma, "Expected ',' between macro arguments")?;
                    }
                }

                self.expect(TokenType::RightParen, "Expected ')' after macro arguments")?;
            }

            self.expect(TokenType::RightBracket, "Expected ']' after macro")?;

            macros.push(MacroAnnotation { name, args });
        }

        Ok(macros)
    }

    fn parse_macro_arg(&mut self) -> Result<Option<MacroArg>> {
        match self.peek().token_type {
            TokenType::Ident => {
                let name_token = self.expect(TokenType::Ident, "Expected identifier")?;
                let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;

                if self.accept(TokenType::Colon).is_some() {
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
        self.expect(TokenType::Var, "Expected 'var'")?;
        let pattern = self.parse_pattern()?;

        let value = if self.accept(TokenType::Equal).is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect(TokenType::Semicolon, "Expected ';'")?;

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
        match self.peek().token_type {
            TokenType::Ident => {
                self.advance();
                // Could be obj.field or obj[key]
                loop {
                    if self.accept(TokenType::Dot).is_some() {
                        if !matches!(self.peek().token_type, TokenType::Ident) {
                            return false;
                        }
                        self.advance();
                    } else if self.accept(TokenType::LeftBracket).is_some() {
                        // Skip the index expression
                        let mut bracket_depth = 1;
                        while bracket_depth > 0 && !self.is_at_end() {
                            match self.peek().token_type {
                                TokenType::LeftBracket => bracket_depth += 1,
                                TokenType::RightBracket => bracket_depth -= 1,
                                _ => {}
                            }
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn is_assign_op(&self) -> bool {
        matches!(
            self.peek().token_type,
            TokenType::Equal
                | TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual
                | TokenType::PercentEqual
        )
    }

    fn parse_expression(&mut self) -> Result<Expression> {
        self.parse_precedence(0)
    }

    /// Parse expressions with precedence climbing
    fn parse_precedence(&mut self, min_precedence: u8) -> Result<Expression> {
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

    fn parse_unary(&mut self) -> Result<Expression> {
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

    fn parse_primary(&mut self) -> Result<Expression> {
        if let Some(_) = self.accept(TokenType::True) {
            Ok(Expression::Literal(LiteralValue::Bool(true)))
        } else if let Some(_) = self.accept(TokenType::False) {
            Ok(Expression::Literal(LiteralValue::Bool(false)))
        } else if let Some(_) = self.accept(TokenType::Null) {
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

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // ===== Helper Functions =====

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().token_type == token_type
        }
    }

    fn accept(&mut self, token_type: TokenType) -> Option<Token> {
        if self.check(token_type) {
            Some(self.advance().clone())
        } else {
            None
        }
    }

    fn expect(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if let Some(token) = self.accept(token_type) {
            Ok(token)
        } else {
            Err(anyhow!("{}: expected {:?}, found {:?}", message, token_type, self.peek().token_type))
        }
    }

    fn parse_literal_value(&mut self) -> Result<LiteralValue> {
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

    // ===== Expression Helper Functions =====

    fn get_binary_precedence(&self, token_type: TokenType) -> u8 {
        match token_type {
            // Postfix operators (highest precedence)
            TokenType::Dot | TokenType::LeftBracket | TokenType::LeftParen => 16,

            // Exponentiation
            TokenType::StarStar => 15,

            // Unary operators would be 14, but handled separately

            // Multiplicative
            TokenType::Star | TokenType::Slash | TokenType::Percent => 13,

            // Additive
            TokenType::Plus | TokenType::Minus => 12,

            // Shift
            TokenType::LessLess | TokenType::GreaterGreater => 11,

            // Range
            TokenType::DotDot | TokenType::DotDotEqual => 10,

            // Relational
            TokenType::Less | TokenType::Greater | TokenType::LessEqual | TokenType::GreaterEqual => 9,
            TokenType::In | TokenType::Is => 8,

            // Equality
            TokenType::EqualEqual | TokenType::BangEqual => 7,

            // Bitwise AND
            TokenType::Ampersand => 6,

            // Bitwise XOR
            TokenType::Caret => 5,

            // Bitwise OR
            TokenType::Pipe => 4,

            // Logical AND
            TokenType::And => 3,

            // Logical XOR
            TokenType::Xor => 2,

            // Logical OR
            TokenType::Or => 1,

            _ => 0, // Not a binary operator
        }
    }

    fn get_binary_expression_constructor(&self, token_type: TokenType) -> Option<BinaryExprConstructor> {
        match token_type {
            TokenType::Plus => Some(Expression::Add),
            TokenType::Minus => Some(Expression::Sub),
            TokenType::Star => Some(Expression::Mul),
            TokenType::Slash => Some(Expression::Div),
            TokenType::Percent => Some(Expression::Mod),
            TokenType::StarStar => Some(Expression::Pow),
            TokenType::EqualEqual => Some(Expression::Eq),
            TokenType::BangEqual => Some(Expression::Ne),
            TokenType::Less => Some(Expression::Lt),
            TokenType::Greater => Some(Expression::Gt),
            TokenType::LessEqual => Some(Expression::Le),
            TokenType::GreaterEqual => Some(Expression::Ge),
            TokenType::And => Some(Expression::And),
            TokenType::Or => Some(Expression::Or),
            TokenType::Xor => Some(Expression::Xor),
            TokenType::Ampersand => Some(Expression::BitAnd),
            TokenType::Pipe => Some(Expression::BitOr),
            TokenType::Caret => Some(Expression::BitXor),
            TokenType::LessLess => Some(Expression::LeftShift),
            TokenType::GreaterGreater => Some(Expression::RightShift),
            _ => None,
        }
    }

    fn is_right_associative_token(&self, token_type: TokenType) -> bool {
        // In RustLeaf, only exponentiation is right-associative
        matches!(token_type, TokenType::StarStar)
    }

    // ===== Stub Functions =====
    // These will be implemented next

    fn parse_pattern(&mut self) -> Result<Pattern> {
        match self.peek().token_type {
            TokenType::Ident => {
                let name_token = self.expect(TokenType::Ident, "Expected identifier")?;
                let name = name_token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?;
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
