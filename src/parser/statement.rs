use crate::core::*;
use crate::lexer::TokenType;
use anyhow::{anyhow, Result};

use super::Parser;

macro_rules! try_parsers {
    ($self:expr, $($parser:ident),+ $(,)?) => {
        $(
            if let Some(stmt) = $self.$parser()? {
                return Ok(stmt);
            }
        )+
    };
}

impl Parser {
    pub fn parse_statement(&mut self) -> Result<Statement> {
        try_parsers!(
            self,
            try_parse_macro,
            try_parse_var_declaration,
            try_parse_function_declaration,
            try_parse_pub_declaration,
            try_parse_class_declaration,
            try_parse_import_statement,
            try_parse_return_statement,
            try_parse_break_statement,
            try_parse_continue_statement,
            try_parse_assignment,
        );

        // Fall back to expression statement
        let expr = self.parse_expression()?;
        self.expect(TokenType::Semicolon, "Expected ';'")?;
        Ok(Statement::Expression(expr))
    }

    pub fn try_parse_macro(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Hash).is_none() {
            return Ok(None);
        }

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
        // Recurse
        let statement = Box::new(self.parse_statement()?);
        Ok(Some(Statement::Macro { name, args, statement }))
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

    pub fn try_parse_var_declaration(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Var).is_none() {
            return Ok(None);
        }

        let pattern = self.parse_pattern()?;
        let value = if self.accept(TokenType::Equal).is_some() {
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
        // TODO: Implement assignment parsing
        Ok(None)
    }

    pub fn try_parse_function_declaration(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Fn).is_none() {
            return Ok(None);
        }

        // TODO: Implement function declaration parsing
        Err(anyhow!("Function declaration parsing not implemented yet"))
    }

    pub fn try_parse_pub_declaration(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Pub).is_none() {
            return Ok(None);
        }

        // TODO: Implement pub declaration parsing
        Err(anyhow!("Public declaration parsing not implemented yet"))
    }

    pub fn try_parse_class_declaration(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Class).is_none() {
            return Ok(None);
        }

        // TODO: Implement class declaration parsing
        Err(anyhow!("Class declaration parsing not implemented yet"))
    }

    pub fn try_parse_import_statement(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Use).is_none() {
            return Ok(None);
        }

        // TODO: Implement import statement parsing
        Err(anyhow!("Import statement parsing not implemented yet"))
    }

    pub fn try_parse_return_statement(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Return).is_none() {
            return Ok(None);
        }

        // TODO: Implement return statement parsing
        Err(anyhow!("Return statement parsing not implemented yet"))
    }

    pub fn try_parse_break_statement(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Break).is_none() {
            return Ok(None);
        }

        // TODO: Implement break statement parsing
        Err(anyhow!("Break statement parsing not implemented yet"))
    }

    pub fn try_parse_continue_statement(&mut self) -> Result<Option<Statement>> {
        if self.accept(TokenType::Continue).is_none() {
            return Ok(None);
        }

        // TODO: Implement continue statement parsing
        Err(anyhow!("Continue statement parsing not implemented yet"))
    }
}