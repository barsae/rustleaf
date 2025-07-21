use crate::core::*;
use crate::lexer::Token;
/// Parser implementation for RustLeaf
use anyhow::Result;

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

    fn parse_statement(&mut self) -> Result<Statement> {
        // TODO: Implement statement parsing
        // Match on current token to determine statement type
        // Handle var, fn, class, if, while, for, etc.

        Err(anyhow::anyhow!("Statement parsing not yet implemented"))
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
}
