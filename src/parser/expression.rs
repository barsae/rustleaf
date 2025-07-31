use crate::core::*;
use crate::lexer::TokenType;
use anyhow::{anyhow, Result};
use super::stream::TokenStream;
use super::statement::{parse_literal_value, parse_pattern};

/// Main entry point for parsing expressions
pub fn parse_expression(s: &mut TokenStream) -> Result<Expression> {
    let result = parse_precedence(s, 0);
    result
}

/// Parse expressions with precedence climbing
fn parse_precedence(s: &mut TokenStream, min_precedence: u8) -> Result<Expression> {
    let mut left = parse_unary(s)?;

    while !s.is_at_end() {
        let op_precedence = get_binary_precedence(s.peek_type());
        if op_precedence < min_precedence {
            break;
        }

        // Binary operators
        if let Some(expr_constructor) = get_binary_expression_constructor(s.peek_type()) {
            s.accept_type(s.peek_type())?; // Consume the operator
            let right_precedence = if is_right_associative_token(s.peek_type()) {
                op_precedence
            } else {
                op_precedence + 1
            };
            let right = parse_precedence(s, right_precedence)?;
            left = expr_constructor(Box::new(left), Box::new(right));
        } else {
            break;
        }
    }

    Ok(left)
}

/// Parse unary expressions
fn parse_unary(s: &mut TokenStream) -> Result<Expression> {
    // Handle unary prefix operators
    if s.accept_type(TokenType::Not)?.is_some() {
        let expr = Box::new(parse_postfix(s)?);
        return Ok(Expression::Not(expr));
    }
    if s.accept_type(TokenType::Minus)?.is_some() {
        let expr = Box::new(parse_postfix(s)?);
        return Ok(Expression::Neg(expr));
    }
    if s.accept_type(TokenType::Plus)?.is_some() {
        // Unary plus is not in the AST, just return the expression
        return parse_postfix(s);
    }

    let result = parse_postfix(s);
    result
}

/// Parse postfix expressions (function calls, array access, property access)
fn parse_postfix(s: &mut TokenStream) -> Result<Expression> {
    let mut expr = parse_primary(s)?;
    
    loop {
        if s.accept_type(TokenType::Dot)?.is_some() {
            let property_token = s.expect_type(TokenType::Ident)?;
            let property = property_token.text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            expr = Expression::GetAttr(Box::new(expr), property);
        } else if s.accept_type(TokenType::LeftBracket)?.is_some() {
            let index = parse_expression(s)?;
            s.expect_type(TokenType::RightBracket)?;
            expr = Expression::GetItem(Box::new(expr), Box::new(index));
        } else if s.accept_type(TokenType::LeftParen)?.is_some() {
            let mut args = Vec::new();

            loop {
                if s.is_at_end() {
                    return Err(anyhow!("Unexpected EOF in function call arguments"));
                }

                if s.accept_type(TokenType::RightParen)?.is_some() {
                    break;
                }

                args.push(parse_expression(s)?);
                if s.accept_type(TokenType::Comma)?.is_none() {
                    s.expect_type(TokenType::RightParen)?;
                    break;
                }
            }

            // Check if this is a method call (function call on a property access)
            expr = match expr {
                Expression::GetAttr(obj, method_name) => {
                    // Convert obj.method(args) to MethodCall
                    Expression::MethodCall(obj, method_name, args)
                }
                _ => {
                    // Regular function call
                    Expression::FunctionCall(Box::new(expr), args)
                }
            };
        } else {
            break;
        }
    }
    
    Ok(expr)
}

/// Parse primary expressions
fn parse_primary(s: &mut TokenStream) -> Result<Expression> {
    // Literals
    if s.accept_type(TokenType::True)?.is_some() {
        return Ok(Expression::Literal(LiteralValue::Bool(true)));
    }
    if s.accept_type(TokenType::False)?.is_some() {
        return Ok(Expression::Literal(LiteralValue::Bool(false)));
    }
    if s.accept_type(TokenType::Null)?.is_some() {
        return Ok(Expression::Literal(LiteralValue::Null));
    }

    // Identifiers (must come before literal parsing to avoid conflicts)
    if let Some(token) = s.accept_type(TokenType::Ident)? {
        let name = token.text
            .ok_or_else(|| anyhow!("Identifier token missing text"))?;
        return Ok(Expression::Identifier(name));
    }
    
    // Try numeric and string literals
    if let Some(literal) = s.try_parse(|s| {
        let value = parse_literal_value(s)?;
        Ok(Expression::Literal(value))
    })? {
        return Ok(literal);
    }

    // Control flow expressions
    if s.accept_type(TokenType::If)?.is_some() {
        return parse_if_expression(s);
    }
    if s.accept_type(TokenType::Match)?.is_some() {
        return parse_match_expression(s);
    }
    if s.accept_type(TokenType::While)?.is_some() {
        return parse_while_expression(s);
    }
    if s.accept_type(TokenType::For)?.is_some() {
        return parse_for_expression(s);
    }
    if s.accept_type(TokenType::Loop)?.is_some() {
        return parse_loop_expression(s);
    }
    if s.accept_type(TokenType::Try)?.is_some() {
        return parse_try_expression(s);
    }
    if s.accept_type(TokenType::With)?.is_some() {
        return parse_with_expression(s);
    }

    // Lambda expressions
    if s.peek_type() == TokenType::Pipe {
        return parse_lambda_expression(s);
    }

    // Collections
    if s.accept_type(TokenType::LeftBracket)?.is_some() {
        return parse_list_literal(s);
    }
    if s.peek_type() == TokenType::LeftBrace {
        // Could be block or dict - need to check
        return parse_block_or_dict(s);
    }

    // Parenthesized expressions
    if s.accept_type(TokenType::LeftParen)?.is_some() {
        let expr = parse_expression(s)?;
        s.expect_type(TokenType::RightParen)?;
        return Ok(expr);
    }

    // Interpolated strings
    if s.peek_type() == TokenType::StringPart || s.peek_type() == TokenType::InterpolationStart {
        return parse_interpolated_string(s);
    }

    Err(anyhow!("Expected expression, found {:?}", s.peek_type()))
}

// Helper functions

fn get_binary_precedence(token_type: TokenType) -> u8 {
    match token_type {
        TokenType::Or => 1,
        TokenType::And => 2,
        TokenType::EqualEqual | TokenType::BangEqual => 3,
        TokenType::Less | TokenType::Greater | TokenType::LessEqual | TokenType::GreaterEqual => 4,
        TokenType::In | TokenType::Is | TokenType::IsNot => 5,
        TokenType::DotDot | TokenType::DotDotEqual => 6, // Range operators
        TokenType::Plus | TokenType::Minus => 11,
        TokenType::Star | TokenType::Slash | TokenType::Percent => 12,
        TokenType::StarStar => 13,
        TokenType::Pipe => 2, // Pipe (|) operator - low precedence
        TokenType::Dot | TokenType::LeftBracket | TokenType::LeftParen => 15,
        _ => 0, // Not a binary operator
    }
}

fn is_right_associative_token(token_type: TokenType) -> bool {
    matches!(token_type, TokenType::StarStar)
}

fn get_binary_expression_constructor(token_type: TokenType) -> Option<fn(Box<Expression>, Box<Expression>) -> Expression> {
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
        TokenType::In => Some(Expression::In),
        TokenType::Is => Some(Expression::Is),
        TokenType::IsNot => Some(Expression::IsNot),
        TokenType::Pipe => Some(Expression::Pipe), // Pipe (|) operator
        TokenType::DotDot => Some(Expression::RangeExclusive),
        TokenType::DotDotEqual => Some(Expression::RangeInclusive),
        _ => None,
    }
}

// Expression parsing functions - placeholders for now

pub fn parse_block_expression(s: &mut TokenStream) -> Result<Expression> {
    let mut statements = Vec::new();
    let mut final_expr = None;
    
    // Parse statements and possible final expression
    while !s.is_at_end() && s.peek_type() != TokenType::RightBrace {
        // Debug: Check for specific problematic case
        if s.peek_type() == TokenType::LeftBracket {
            println!("DEBUG: Found [, after parsing statements: {:?}", statements.last());
        }
        
        // Try to parse a statement first
        if let Some(stmt) = s.try_parse(super::statement::parse_statement)? {
            statements.push(stmt);
        } else {
            // Failed to parse as statement, try as expression
            if let Some(expr) = s.try_parse(parse_expression)? {
                // Check if this is followed by a semicolon
                if s.accept_type(TokenType::Semicolon)?.is_some() {
                    // It's an expression statement
                    statements.push(Statement::Expression(expr));
                } else {
                    // It's the final expression
                    final_expr = Some(Box::new(expr));
                    break;
                }
            } else {
                // Neither statement nor expression worked
                return Err(anyhow!("Expected statement or expression"));
            }
        }
    }
    
    s.expect_type(TokenType::RightBrace)?;
    Ok(Expression::Block(Block { statements, final_expr }))
}

fn parse_if_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'if' already consumed
    let condition = Box::new(parse_expression(s)?);
    
    s.expect_type(TokenType::LeftBrace)?;
    let then_block_expr = parse_block_expression(s)?;
    let then_expr = match then_block_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    let else_expr = if s.accept_type(TokenType::Else)?.is_some() {
        if s.peek_type() == TokenType::If {
            // else if - convert the if expression to a block
            s.accept_type(TokenType::If)?;
            let else_if_expr = parse_if_expression(s)?;
            Some(Block {
                statements: vec![Statement::Expression(else_if_expr)],
                final_expr: None,
            })
        } else {
            s.expect_type(TokenType::LeftBrace)?;
            let else_block_expr = parse_block_expression(s)?;
            match else_block_expr {
                Expression::Block(block) => Some(block),
                _ => unreachable!("parse_block_expression should return a Block"),
            }
        }
    } else {
        None
    };
    
    Ok(Expression::If {
        condition,
        then_expr,
        else_expr,
    })
}

fn parse_match_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'match' already consumed
    let expr = Box::new(parse_expression(s)?);
    s.expect_type(TokenType::LeftBrace)?;
    
    let mut cases = Vec::new();
    
    while !s.is_at_end() && s.peek_type() != TokenType::RightBrace {
        let pattern = parse_pattern(s)?;
        
        let guard = if s.accept_type(TokenType::If)?.is_some() {
            Some(parse_expression(s)?)
        } else {
            None
        };
        
        s.expect_type(TokenType::Colon)?;
        s.expect_type(TokenType::LeftBrace)?;
        let body_expr = parse_block_expression(s)?;
        let body = match body_expr {
            Expression::Block(block) => block,
            _ => unreachable!("parse_block_expression should return a Block"),
        };
        
        cases.push(MatchCase { pattern, guard, body });
        
        // Optional comma between cases
        s.accept_type(TokenType::Comma)?;
    }
    
    s.expect_type(TokenType::RightBrace)?;
    Ok(Expression::Match { expr, cases })
}

fn parse_while_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'while' already consumed
    let condition = Box::new(parse_expression(s)?);
    s.expect_type(TokenType::LeftBrace)?;
    // Parse as block expression to support final expressions
    let block_expr = parse_block_expression(s)?;
    let body = match block_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    Ok(Expression::While { condition, body })
}

fn parse_for_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'for' already consumed
    let pattern = parse_pattern(s)?;
    s.expect_type(TokenType::In)?;
    let iterable = Box::new(parse_expression(s)?);
    s.expect_type(TokenType::LeftBrace)?;
    // Parse as block expression to support final expressions
    let block_expr = parse_block_expression(s)?;
    let body = match block_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    Ok(Expression::For { pattern, iter: iterable, body })
}

fn parse_loop_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'loop' already consumed
    s.expect_type(TokenType::LeftBrace)?;
    // Parse as block expression to support final expressions and break values
    let block_expr = parse_block_expression(s)?;
    let body = match block_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    Ok(Expression::Loop { body })
}

fn parse_try_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'try' already consumed
    s.expect_type(TokenType::LeftBrace)?;
    let body_expr = parse_block_expression(s)?;
    let body = match body_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    s.expect_type(TokenType::Catch)?;
    let pattern = parse_pattern(s)?;
    s.expect_type(TokenType::LeftBrace)?;
    let catch_expr = parse_block_expression(s)?;
    let catch_body = match catch_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    Ok(Expression::Try {
        body,
        catch: CatchClause {
            pattern,
            body: catch_body,
        },
    })
}

fn parse_with_expression(s: &mut TokenStream) -> Result<Expression> {
    // 'with' already consumed
    let resources;
    
    // Helper to parse resource form (name = expr)
    let parse_resource_form = |s: &mut TokenStream| -> Result<Vec<WithResource>> {
        let mut resources = Vec::new();
        loop {
            let name_token = s.expect_type(TokenType::Ident)?;
            let name = name_token.text
                .ok_or_else(|| anyhow!("Identifier token missing text"))?;
            s.expect_type(TokenType::Equal)?;
            let value = parse_expression(s)?;
            
            resources.push(WithResource { name, value });
            
            if !s.accept_type(TokenType::Comma)?.is_some() {
                break;
            }
        }
        // Optional colon before block
        s.accept_type(TokenType::Colon)?;
        Ok(resources)
    };
    
    // Try to parse as simple expression form first
    if let Some(result) = s.try_parse(|s| {
        let expr = parse_unary(s)?;
        // Check if next token is a block start
        if s.peek_type() == TokenType::LeftBrace {
            // Simple form: with expr { ... }
            Ok(vec![WithResource { 
                name: String::new(), 
                value: expr 
            }])
        } else if s.peek_type() == TokenType::Comma || s.peek_type() == TokenType::Colon {
            // Actually resource form, fail this parse attempt
            Err(anyhow!("Not simple expression form"))
        } else {
            Err(anyhow!("Expected {{ or : after with expression"))
        }
    })? {
        resources = result;
    } else {
        // Must be resource form (name = expr)
        resources = parse_resource_form(s)?;
    }
    
    // Parse body
    s.expect_type(TokenType::LeftBrace)?;
    let body_expr = parse_block_expression(s)?;
    let body = match body_expr {
        Expression::Block(block) => block,
        _ => unreachable!("parse_block_expression should return a Block"),
    };
    
    Ok(Expression::With { resources, body })
}

fn parse_lambda_expression(s: &mut TokenStream) -> Result<Expression> {
    // Parse opening |
    s.expect_type(TokenType::Pipe)?;
    
    let mut params = Vec::new();
    
    // Parse parameters
    while s.peek_type() != TokenType::Pipe {
        let param_token = s.expect_type(TokenType::Ident)?;
        let param = param_token.text
            .ok_or_else(|| anyhow!("Identifier token missing text"))?;
        params.push(param);
        
        if s.peek_type() != TokenType::Pipe {
            s.expect_type(TokenType::Comma)?;
        }
    }
    
    // Parse closing |
    s.expect_type(TokenType::Pipe)?;
    
    // Parse body - either an expression or a block
    let body = if s.peek_type() == TokenType::LeftBrace {
        s.expect_type(TokenType::LeftBrace)?;
        // Parse as block expression with potential final expression
        let block_expr = parse_block_expression(s)?;
        match block_expr {
            Expression::Block(block) => LambdaBody::Block(block),
            _ => unreachable!("parse_block_expression should return a Block"),
        }
    } else {
        LambdaBody::Expression(Box::new(parse_expression(s)?))
    };
    
    Ok(Expression::Lambda { params, body })
}

fn parse_list_literal(s: &mut TokenStream) -> Result<Expression> {
    // We already consumed the [
    let mut elements = Vec::new();
    
    // Handle empty list
    if s.accept_type(TokenType::RightBracket)?.is_some() {
        return Ok(Expression::List(elements));
    }
    
    loop {
        elements.push(parse_expression(s)?);
        
        if s.accept_type(TokenType::Comma)?.is_some() {
            // Check for trailing comma
            if s.peek_type() == TokenType::RightBracket {
                break;
            }
        } else {
            break;
        }
    }
    
    s.expect_type(TokenType::RightBracket)?;
    Ok(Expression::List(elements))
}

fn parse_block_or_dict(s: &mut TokenStream) -> Result<Expression> {
    s.expect_type(TokenType::LeftBrace)?;
    
    // Handle empty dict: {}
    if s.accept_type(TokenType::RightBrace)?.is_some() {
        return Ok(Expression::Dict(Vec::new()));
    }
    
    // Try to parse as dictionary first, fall back to block
    if let Some(dict_expr) = s.try_parse(|s| {
        let mut pairs = Vec::new();
        
        loop {
            // Parse key expression - use high precedence to avoid consuming colons
            let key = parse_precedence(s, 14)?; // Higher than pipe precedence (13)
            
            // Must be followed by ':'
            s.expect_type(TokenType::Colon)?;
            
            // Parse value expression
            let value = parse_expression(s)?;
            
            pairs.push((key, value));
            
            // Check for continuation
            if s.accept_type(TokenType::Comma)?.is_some() {
                // Allow trailing comma before }
                if s.peek_type() == TokenType::RightBrace {
                    break;
                }
                continue;
            } else {
                break;
            }
        }
        
        s.expect_type(TokenType::RightBrace)?;
        Ok(Expression::Dict(pairs))
    })? {
        return Ok(dict_expr);
    }
    
    // Parse as block expression (opening { already consumed)
    let result = parse_block_expression(s)?;
    Ok(result)
}

fn parse_interpolated_string(s: &mut TokenStream) -> Result<Expression> {
    let mut parts = Vec::new();
    
    // Parse string parts and interpolations
    while !s.is_at_end() {
        // Check for string part
        if let Some(token) = s.accept_type(TokenType::StringPart)? {
            if let Some(text) = token.text {
                parts.push(InterpolationPart::Text(text));
            }
        }
        
        // Check for interpolation start
        if s.accept_type(TokenType::InterpolationStart)?.is_some() {
            let expr = parse_expression(s)?;
            s.expect_type(TokenType::InterpolationEnd)?;
            parts.push(InterpolationPart::Expression(expr));
        } else {
            // No more interpolations
            break;
        }
    }
    
    if parts.is_empty() {
        return Err(anyhow!("Expected interpolated string"));
    }
    
    Ok(Expression::InterpolatedString(parts))
}