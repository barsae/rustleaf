use crate::core::*;
use crate::lexer::TokenType;
use anyhow::{anyhow, Result};
use super::stream::TokenStream;
use crate::trace;

/// Parse a single statement
pub fn parse_statement(s: &mut TokenStream) -> Result<Statement> {
    trace!("parse_statement: starting at position {} ({})", s.position(), s.current_token_info());
    
    // Try each statement type in order
    if let Some(stmt) = s.try_parse(parse_macro)? {
        trace!("parse_statement: success - parsed macro");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_var_declaration)? {
        trace!("parse_statement: success - parsed var declaration");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_function_declaration)? {
        trace!("parse_statement: success - parsed function declaration");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_class_declaration)? {
        trace!("parse_statement: success - parsed class declaration");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_import_statement)? {
        trace!("parse_statement: success - parsed import statement");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_return_statement)? {
        trace!("parse_statement: success - parsed return statement");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_break_statement)? {
        trace!("parse_statement: success - parsed break statement");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_continue_statement)? {
        trace!("parse_statement: success - parsed continue statement");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_assignment)? {
        trace!("parse_statement: success - parsed assignment");
        return Ok(stmt);
    }
    if let Some(stmt) = s.try_parse(parse_block_like_expression_statement)? {
        trace!("parse_statement: success - parsed block-like expression statement");
        return Ok(stmt);
    }
    
    // Fall back to expression statement
    trace!("parse_statement: falling back to expression statement");
    let result = parse_expression_statement(s);
    if result.is_err() {
        trace!("parse_statement: failed - {}", result.as_ref().err().unwrap());
    }
    result
}

fn parse_expression_statement(s: &mut TokenStream) -> Result<Statement> {
    let expr = parse_expression(s)?;
    s.expect_type(TokenType::Semicolon)?;
    Ok(Statement::Expression(expr))
}

fn parse_macro(s: &mut TokenStream) -> Result<Statement> {
    s.expect_type(TokenType::Hash)?;
    s.expect_type(TokenType::LeftBracket)?;
    
    // Accept either identifier or macro keyword
    let name = if let Some(token) = s.accept_type(TokenType::Ident)? {
        token.text.ok_or_else(|| anyhow!("Identifier token missing text"))?
    } else if s.accept_type(TokenType::Macro)?.is_some() {
        "macro".to_string()
    } else {
        return Err(anyhow!("Expected macro name: identifier or 'macro'"));
    };
    
    let mut args = Vec::new();
    
    if s.accept_type(TokenType::LeftParen)?.is_some() {
        loop {
            if s.is_at_end() {
                return Err(anyhow!("Unexpected EOF in macro arguments"));
            }
            
            if s.accept_type(TokenType::RightParen)?.is_some() {
                break;
            }
            
            if let Some(arg) = s.try_parse(parse_macro_arg)? {
                args.push(arg);
            }
            
            if s.accept_type(TokenType::Comma)?.is_none() {
                s.expect_type(TokenType::RightParen)?;
                break;
            }
        }
    }
    
    s.expect_type(TokenType::RightBracket)?;
    let statement = Box::new(parse_statement(s)?);
    
    let result = Statement::Macro {
        name,
        args,
        statement,
    };
    Ok(result)
}

fn parse_macro_arg(s: &mut TokenStream) -> Result<MacroArg> {
    // Try named argument first: ident:value
    if let Some((name, value)) = s.try_parse(|s| {
        let name_token = s.accept_type(TokenType::Ident)?
            .ok_or_else(|| anyhow!("Expected identifier"))?;
        let name = name_token.text
            .ok_or_else(|| anyhow!("Identifier token missing text"))?;
        s.expect_type(TokenType::Colon)?;
        let value = parse_literal_value(s)?;
        Ok((name, value))
    })? {
        Ok(MacroArg::Named(name, value))
    } else {
        // Positional argument
        let value = parse_literal_value(s)?;
        Ok(MacroArg::Positional(value))
    }
}

fn parse_var_declaration(s: &mut TokenStream) -> Result<Statement> {
    s.expect_type(TokenType::Var)?;
    let pattern = parse_pattern(s)?;
    let value = if s.accept_type(TokenType::Equal)?.is_some() {
        Some(parse_expression(s)?)
    } else {
        None
    };
    s.expect_type(TokenType::Semicolon)?;
    
    Ok(Statement::VarDecl { pattern, value })
}

fn parse_function_declaration(s: &mut TokenStream) -> Result<Statement> {
    let is_pub = s.accept_type(TokenType::Pub)?.is_some();
    s.expect_type(TokenType::Fn)?;
    
    let name_token = s.expect_type(TokenType::Ident)?;
    let name = name_token.text
        .ok_or_else(|| anyhow!("Function name token missing text"))?;
    
    s.expect_type(TokenType::LeftParen)?;
    
    let mut params = Vec::new();
    loop {
        if s.is_at_end() {
            return Err(anyhow!("Unexpected EOF in function parameters"));
        }
        
        if s.accept_type(TokenType::RightParen)?.is_some() {
            break;
        }
        
        params.push(parse_parameter(s)?);
        
        if s.accept_type(TokenType::Comma)?.is_none() {
            s.expect_type(TokenType::RightParen)?;
            break;
        }
    }
    
    s.expect_type(TokenType::LeftBrace)?;
    let body_expr = super::expression::parse_block_expression(s)?;
    let body = match body_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    Ok(Statement::FnDecl {
        name,
        params,
        body,
        is_pub,
    })
}

fn parse_parameter(s: &mut TokenStream) -> Result<Parameter> {
    let kind = if s.accept_type(TokenType::Star)?.is_some() {
        if s.accept_type(TokenType::Star)?.is_some() {
            ParameterKind::Keyword
        } else {
            ParameterKind::Rest
        }
    } else {
        ParameterKind::Regular
    };
    
    let name_token = s.expect_type(TokenType::Ident)?;
    let name = name_token.text
        .ok_or_else(|| anyhow!("Parameter name token missing text"))?;
    
    let default = if s.accept_type(TokenType::Equal)?.is_some() {
        Some(parse_literal_value(s)?)
    } else {
        None
    };
    
    Ok(Parameter {
        name,
        default,
        kind,
    })
}

// Placeholder functions - to be implemented
fn parse_class_declaration(s: &mut TokenStream) -> Result<Statement> {
    let is_pub = s.accept_type(TokenType::Pub)?.is_some();
    s.expect_type(TokenType::Class)?;
    
    let name_token = s.expect_type(TokenType::Ident)?;
    let name = name_token.text
        .ok_or_else(|| anyhow!("Class name token missing text"))?;
    
    // Handle optional constructor parameters: class Name() or class Name(params)
    if s.accept_type(TokenType::LeftParen)?.is_some() {
        // Skip constructor parameters for now - just consume until )
        while s.peek_type() != TokenType::RightParen && !s.is_at_end() {
            // Consume any token until we find )
            s.accept_type(s.peek_type())?;
        }
        s.expect_type(TokenType::RightParen)?;
    }
    
    s.expect_type(TokenType::LeftBrace)?;
    
    let mut members = Vec::new();
    
    while s.peek_type() != TokenType::RightBrace && !s.is_at_end() {
        if let Some(member) = s.try_parse(parse_class_member)? {
            members.push(member);
        } else {
            // If we can't parse a member, skip to avoid infinite loop
            break;
        }
    }
    
    s.expect_type(TokenType::RightBrace)?;
    
    Ok(Statement::ClassDecl {
        name,
        members,
        is_pub,
    })
}

fn parse_class_member(s: &mut TokenStream) -> Result<ClassMember> {
    // Check for static methods
    if s.accept_type(TokenType::Static)?.is_some() {
        s.expect_type(TokenType::Fn)?;
        let name_token = s.expect_type(TokenType::Ident)?;
        let name = name_token.text
            .ok_or_else(|| anyhow!("Method name token missing text"))?;
        
        s.expect_type(TokenType::LeftParen)?;
        let mut params = Vec::new();
        
        // Parse parameters
        while s.peek_type() != TokenType::RightParen && !s.is_at_end() {
            params.push(parse_parameter(s)?);
            if s.accept_type(TokenType::Comma)?.is_none() {
                break;
            }
        }
        s.expect_type(TokenType::RightParen)?;
        
        s.expect_type(TokenType::LeftBrace)?;
        let body_expr = super::expression::parse_block_expression(s)?;
        let body = match body_expr {
            Expression::Block(block) => block,
            _ => unreachable!("parse_block_expression should return a Block"),
        };
        
        return Ok(ClassMember {
            name,
            kind: ClassMemberKind::StaticMethod { params, body },
        });
    }
    
    // Check for regular methods
    if s.accept_type(TokenType::Fn)?.is_some() {
        let name_token = s.expect_type(TokenType::Ident)?;
        let name = name_token.text
            .ok_or_else(|| anyhow!("Method name token missing text"))?;
        
        s.expect_type(TokenType::LeftParen)?;
        let mut params = Vec::new();
        
        // Parse parameters
        while s.peek_type() != TokenType::RightParen && !s.is_at_end() {
            params.push(parse_parameter(s)?);
            if s.accept_type(TokenType::Comma)?.is_none() {
                break;
            }
        }
        s.expect_type(TokenType::RightParen)?;
        
        s.expect_type(TokenType::LeftBrace)?;
        let body_expr = super::expression::parse_block_expression(s)?;
        let body = match body_expr {
            Expression::Block(block) => block,
            _ => unreachable!("parse_block_expression should return a Block"),
        };
        
        return Ok(ClassMember {
            name,
            kind: ClassMemberKind::Method { params, body },
        });
    }
    
    // Check for field declarations
    if s.accept_type(TokenType::Var)?.is_some() {
        let name_token = s.expect_type(TokenType::Ident)?;
        let name = name_token.text
            .ok_or_else(|| anyhow!("Field name token missing text"))?;
        
        let initializer = if s.accept_type(TokenType::Equal)?.is_some() {
            Some(parse_expression(s)?)
        } else {
            None
        };
        
        s.expect_type(TokenType::Semicolon)?;
        
        return Ok(ClassMember {
            name,
            kind: ClassMemberKind::Field(initializer),
        });
    }
    
    Err(anyhow!("Expected class member"))
}

fn parse_import_statement(s: &mut TokenStream) -> Result<Statement> {
    s.expect_type(TokenType::Use)?;
    
    // Parse module path (e.g., "std::io")
    let mut module_parts = Vec::new();
    
    let first_part = s.expect_type(TokenType::Ident)?;
    module_parts.push(
        first_part.text
            .ok_or_else(|| anyhow!("Module name token missing text"))?,
    );
    
    while s.accept_type(TokenType::DoubleColon)?.is_some() {
        // Check if this is the final :: before import items
        if s.peek_type() == TokenType::Star ||
           s.peek_type() == TokenType::LeftBrace ||
           s.peek_type() == TokenType::Ident {
            // This is the final :: before import items
            break;
        } else {
            // This is part of the module path
            let part = s.expect_type(TokenType::Ident)?;
            module_parts.push(
                part.text
                    .ok_or_else(|| anyhow!("Module name token missing text"))?,
            );
        }
    }
    
    let module = module_parts.join("::");
    
    let items = if s.accept_type(TokenType::Star)?.is_some() {
        // use module::*
        ImportItems::All
    } else if s.accept_type(TokenType::LeftBrace)?.is_some() {
        // use module::{item1, item2 as alias}
        let mut import_items = Vec::new();
        
        loop {
            if s.peek_type() == TokenType::RightBrace {
                break;
            }
            
            let name_token = s.expect_type(TokenType::Ident)?;
            let name = name_token.text
                .ok_or_else(|| anyhow!("Import item name token missing text"))?;
            
            let alias = if s.accept_type(TokenType::As)?.is_some() {
                let alias_token = s.expect_type(TokenType::Ident)?;
                Some(alias_token.text
                    .ok_or_else(|| anyhow!("Alias token missing text"))?)
            } else {
                None
            };
            
            import_items.push(ImportItem { name, alias });
            
            if s.accept_type(TokenType::Comma)?.is_none() {
                break;
            }
        }
        
        s.expect_type(TokenType::RightBrace)?;
        ImportItems::Specific(import_items)
    } else {
        // use module::item
        let name_token = s.expect_type(TokenType::Ident)?;
        let name = name_token.text
            .ok_or_else(|| anyhow!("Import item name token missing text"))?;
        
        let alias = if s.accept_type(TokenType::As)?.is_some() {
            let alias_token = s.expect_type(TokenType::Ident)?;
            Some(alias_token.text
                .ok_or_else(|| anyhow!("Alias token missing text"))?)
        } else {
            None
        };
        
        ImportItems::Specific(vec![ImportItem { name, alias }])
    };
    
    s.expect_type(TokenType::Semicolon)?;
    
    Ok(Statement::Import(ImportSpec { module, items }))
}

fn parse_return_statement(s: &mut TokenStream) -> Result<Statement> {
    s.expect_type(TokenType::Return)?;
    
    let expr = if let Some(_token) = s.accept_type(TokenType::Semicolon)? {
        // Put the semicolon back for the caller to consume
        // This is a hack - ideally we'd peek at the next token
        // For now, we'll just not consume it here
        None
    } else {
        Some(parse_expression(s)?)
    };
    
    s.expect_type(TokenType::Semicolon)?;
    Ok(Statement::Return(expr))
}

fn parse_break_statement(s: &mut TokenStream) -> Result<Statement> {
    s.expect_type(TokenType::Break)?;
    
    let expr = if let Some(_) = s.accept_type(TokenType::Semicolon)? {
        None
    } else {
        let expr = Some(parse_expression(s)?);
        s.expect_type(TokenType::Semicolon)?;
        expr
    };
    
    Ok(Statement::Break(expr))
}

fn parse_continue_statement(s: &mut TokenStream) -> Result<Statement> {
    s.expect_type(TokenType::Continue)?;
    s.expect_type(TokenType::Semicolon)?;
    Ok(Statement::Continue)
}

fn parse_assignment(s: &mut TokenStream) -> Result<Statement> {
    // Try to parse an lvalue
    let target = parse_lvalue(s)?;
    
    // Parse assignment operator
    let op = if s.accept_type(TokenType::Equal)?.is_some() {
        AssignOp::Assign
    } else if s.accept_type(TokenType::PlusEqual)?.is_some() {
        AssignOp::AddAssign
    } else if s.accept_type(TokenType::MinusEqual)?.is_some() {
        AssignOp::SubAssign
    } else if s.accept_type(TokenType::StarEqual)?.is_some() {
        AssignOp::MulAssign
    } else if s.accept_type(TokenType::SlashEqual)?.is_some() {
        AssignOp::DivAssign
    } else if s.accept_type(TokenType::PercentEqual)?.is_some() {
        AssignOp::ModAssign
    } else {
        return Err(anyhow!("Expected assignment operator"));
    };
    
    let value = parse_expression(s)?;
    s.expect_type(TokenType::Semicolon)?;
    
    Ok(Statement::Assignment { target, op, value })
}

fn parse_lvalue(s: &mut TokenStream) -> Result<LValue> {
    let name_token = s.expect_type(TokenType::Ident)?;
    let name = name_token.text
        .ok_or_else(|| anyhow!("Identifier token missing text"))?;
    let mut expr = LValue::Identifier(name);
    
    // Handle chained property/index access
    loop {
        if s.accept_type(TokenType::Dot)?.is_some() {
            let property_token = s.expect_type(TokenType::Ident)?;
            let property = property_token.text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            
            // Convert LValue to Expression for GetAttr
            let base_expr = match expr {
                LValue::Identifier(name) => Expression::Identifier(name),
                LValue::GetAttr(obj, field) => Expression::GetAttr(obj, field),
                LValue::GetItem(obj, key) => Expression::GetItem(obj, key),
            };
            expr = LValue::GetAttr(Box::new(base_expr), property);
        } else if s.accept_type(TokenType::LeftBracket)?.is_some() {
            let index = parse_expression(s)?;
            s.expect_type(TokenType::RightBracket)?;
            
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

fn parse_block_like_expression_statement(s: &mut TokenStream) -> Result<Statement> {
    // Check for block-like expressions that don't require semicolons
    if matches!(s.peek_type(), 
        TokenType::If | TokenType::Loop | TokenType::While | 
        TokenType::For | TokenType::Match | TokenType::Try | TokenType::With
    ) {
        // Parse as expression and wrap in Statement::Expression
        let expr = parse_expression(s)?;
        return Ok(Statement::Expression(expr));
    }
    
    // Try to parse a block expression (will backtrack if it's actually a dictionary)
    if s.peek_type() == TokenType::LeftBrace {
        if let Some(expr) = s.try_parse(|s| {
            let expr = parse_expression(s)?;
            // Check if this parsed as a block (not a dictionary)
            if matches!(expr, Expression::Block(_)) {
                Ok(expr)
            } else {
                // It was a dictionary - signal to backtrack
                Err(anyhow!("Not a block expression"))
            }
        })? {
            return Ok(Statement::Expression(expr));
        }
    }
    
    Err(anyhow!("Not a block-like expression statement"))
}

pub fn parse_pattern(s: &mut TokenStream) -> Result<Pattern> {
    match s.peek_type() {
        TokenType::Ident => {
            let name_token = s.expect_type(TokenType::Ident)?;
            let name = name_token.text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            if name == "_" {
                Ok(Pattern::Wildcard)
            } else {
                Ok(Pattern::Variable(name))
            }
        }
        TokenType::Int | TokenType::Float | TokenType::String | 
        TokenType::True | TokenType::False | TokenType::Null => {
            // Try to parse as range pattern first
            if let Some(range_pattern) = s.try_parse(|s| {
                let first_literal = parse_literal_value(s)?;
                
                // Check for range operators
                if s.accept_type(TokenType::DotDot)?.is_some() || 
                   s.accept_type(TokenType::DotDotEqual)?.is_some() {
                    let second_literal = parse_literal_value(s)?;
                    Ok(Pattern::Range(
                        Box::new(Pattern::Literal(first_literal)),
                        Box::new(Pattern::Literal(second_literal)),
                    ))
                } else {
                    // Signal that this isn't a range pattern by returning an error
                    Err(anyhow!("Not a range pattern"))
                }
            })? {
                Ok(range_pattern)
            } else {
                // Parse as simple literal
                let literal = parse_literal_value(s)?;
                Ok(Pattern::Literal(literal))
            }
        }
        TokenType::LeftBracket => parse_list_pattern(s),
        TokenType::LeftBrace => parse_dict_pattern(s),
        _ => Err(anyhow!("Unsupported pattern type: {:?}", s.peek_type())),
    }
}

fn parse_list_pattern(s: &mut TokenStream) -> Result<Pattern> {
    s.expect_type(TokenType::LeftBracket)?;
    
    let mut patterns = Vec::new();
    let mut rest_var = None;
    
    // Handle empty list pattern: []
    if s.accept_type(TokenType::RightBracket)?.is_some() {
        return Ok(Pattern::List(patterns));
    }
    
    loop {
        // Check for rest pattern: *name
        if s.accept_type(TokenType::Star)?.is_some() {
            let rest_token = s.expect_type(TokenType::Ident)?;
            let rest_name = rest_token.text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            rest_var = Some(rest_name);
            
            // After rest pattern, we can only have closing bracket or comma + closing bracket
            if s.accept_type(TokenType::Comma)?.is_some() {
                // Allow trailing comma
                if s.peek_type() != TokenType::RightBracket {
                    return Err(anyhow!("Rest pattern must be the last element"));
                }
            }
            break;
        }
        
        // Parse regular pattern
        patterns.push(parse_pattern(s)?);
        
        if s.accept_type(TokenType::Comma)?.is_some() {
            // Check for trailing comma
            if s.peek_type() == TokenType::RightBracket {
                break;
            }
            continue;
        } else {
            break;
        }
    }
    
    s.expect_type(TokenType::RightBracket)?;
    
    if rest_var.is_some() {
        Ok(Pattern::ListRest(patterns, rest_var))
    } else {
        Ok(Pattern::List(patterns))
    }
}

fn parse_dict_pattern(s: &mut TokenStream) -> Result<Pattern> {
    s.expect_type(TokenType::LeftBrace)?;
    
    let mut dict_patterns = Vec::new();
    
    // Handle empty dict pattern: {}
    if s.accept_type(TokenType::RightBrace)?.is_some() {
        return Ok(Pattern::Dict(dict_patterns));
    }
    
    loop {
        // Parse key identifier
        let key_token = s.expect_type(TokenType::Ident)?;
        let key = key_token.text
            .ok_or_else(|| anyhow!("Identifier token missing text"))?;
        
        let alias = if s.accept_type(TokenType::Colon)?.is_some() {
            // {key: alias} form
            let alias_token = s.expect_type(TokenType::Ident)?;
            Some(alias_token.text
                .ok_or_else(|| anyhow!("Alias token missing text"))?)
        } else {
            // {key} form - use key as the variable name
            None
        };
        
        dict_patterns.push(DictPattern { key, alias });
        
        if s.accept_type(TokenType::Comma)?.is_some() {
            // Check for trailing comma
            if s.peek_type() == TokenType::RightBrace {
                break;
            }
            continue;
        } else {
            break;
        }
    }
    
    s.expect_type(TokenType::RightBrace)?;
    Ok(Pattern::Dict(dict_patterns))
}

fn parse_expression(s: &mut TokenStream) -> Result<Expression> {
    super::expression::parse_expression(s)
}

pub fn parse_literal_value(s: &mut TokenStream) -> Result<LiteralValue> {
    if s.accept_type(TokenType::True)?.is_some() {
        Ok(LiteralValue::Bool(true))
    } else if s.accept_type(TokenType::False)?.is_some() {
        Ok(LiteralValue::Bool(false))
    } else if s.accept_type(TokenType::Null)?.is_some() {
        Ok(LiteralValue::Null)
    } else if let Some(token) = s.accept_type(TokenType::Int)? {
        let text = token.text
            .ok_or_else(|| anyhow!("Int token missing text"))?;
        let n = text.parse::<i64>()
            .map_err(|e| anyhow!("Failed to parse integer '{}': {}", text, e))?;
        Ok(LiteralValue::Int(n))
    } else if let Some(token) = s.accept_type(TokenType::Float)? {
        let text = token.text
            .ok_or_else(|| anyhow!("Float token missing text"))?;
        let f = text.parse::<f64>()
            .map_err(|e| anyhow!("Failed to parse float '{}': {}", text, e))?;
        Ok(LiteralValue::Float(f))
    } else if let Some(token) = s.accept_type(TokenType::String)? {
        let text = token.text
            .ok_or_else(|| anyhow!("String token missing text"))?;
        Ok(LiteralValue::String(text))
    } else if let Some(token) = s.accept_type(TokenType::RawString)? {
        let text = token.text
            .ok_or_else(|| anyhow!("RawString token missing text"))?;
        Ok(LiteralValue::String(text))
    } else {
        Err(anyhow!("Expected literal value"))
    }
}

#[allow(dead_code)]
pub fn parse_block(s: &mut TokenStream) -> Result<Vec<Statement>> {
    let mut statements = Vec::new();
    
    // Parse statements until we hit the closing brace
    while !s.is_at_end() && s.peek_type() != TokenType::RightBrace {
        statements.push(parse_statement(s)?);
    }
    
    s.expect_type(TokenType::RightBrace)?;
    Ok(statements)
}