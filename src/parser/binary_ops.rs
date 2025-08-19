//! Binary operator handling for parser
#![allow(missing_docs)]

use crate::core::*;
use crate::lexer::TokenType;

use super::Parser;

pub type BinaryExprConstructor = fn(Box<Expression>, Box<Expression>) -> Expression;

impl Parser {
    pub fn get_binary_precedence(&self, token_type: TokenType) -> u8 {
        match token_type {
            // Postfix operators (highest precedence)
            TokenType::Dot | TokenType::LeftBracket | TokenType::LeftParen => 16,

            // Exponentiation
            TokenType::StarStar => 15,

            // Unary operators would be 14, but handled separately

            // Multiplicative
            TokenType::Star | TokenType::Slash | TokenType::Percent => 14,

            // Pipe operator - low precedence
            TokenType::Pipe => 2,

            // Additive
            TokenType::Plus | TokenType::Minus => 12,

            // Shift

            // Range
            TokenType::DotDot | TokenType::DotDotEqual => 10,

            // Relational
            TokenType::Less
            | TokenType::Greater
            | TokenType::LessEqual
            | TokenType::GreaterEqual => 9,
            TokenType::In | TokenType::Is | TokenType::NotIn | TokenType::IsNot => 8,

            // Equality
            TokenType::EqualEqual | TokenType::BangEqual => 7,

            // Logical AND
            TokenType::And => 3,

            // Logical XOR
            TokenType::Xor => 2,

            // Logical OR
            TokenType::Or => 1,

            _ => 0, // Not a binary operator
        }
    }

    pub fn get_binary_expression_constructor(
        &self,
        token_type: TokenType,
    ) -> Option<BinaryExprConstructor> {
        match token_type {
            TokenType::Plus => Some(Expression::Add),
            TokenType::Minus => Some(Expression::Sub),
            TokenType::Star => Some(Expression::Mul),
            TokenType::Slash => Some(Expression::Div),
            TokenType::Percent => Some(Expression::Mod),
            TokenType::StarStar => Some(Expression::Pow),
            TokenType::Pipe => Some(Expression::Pipe),
            TokenType::EqualEqual => Some(Expression::Eq),
            TokenType::BangEqual => Some(Expression::Ne),
            TokenType::Less => Some(Expression::Lt),
            TokenType::Greater => Some(Expression::Gt),
            TokenType::LessEqual => Some(Expression::Le),
            TokenType::GreaterEqual => Some(Expression::Ge),
            TokenType::In => Some(Expression::In),
            TokenType::Is => Some(Expression::Is),
            TokenType::NotIn => Some(Expression::NotIn),
            TokenType::IsNot => Some(Expression::IsNot),
            TokenType::And => Some(Expression::And),
            TokenType::Or => Some(Expression::Or),
            TokenType::Xor => Some(Expression::Xor),
            TokenType::DotDot => Some(Expression::RangeExclusive),
            TokenType::DotDotEqual => Some(Expression::RangeInclusive),
            _ => None,
        }
    }

    pub fn is_right_associative_token(&self, token_type: TokenType) -> bool {
        // In RustLeaf, only exponentiation is right-associative
        matches!(token_type, TokenType::StarStar)
    }
}
