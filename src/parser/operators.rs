use crate::lexer::TokenType;
use crate::parser::ast::*;
use crate::parser::Parser;
use rustleaf_macros::binary_ops;

#[binary_ops(
    parse_logical_or -> parse_logical_xor: [Or],
    parse_logical_xor -> parse_logical_and: [Xor],
    parse_logical_and -> parse_bitwise_or: [And],
    parse_bitwise_or -> parse_bitwise_xor: [Pipe => BitwiseOr],
    parse_bitwise_xor -> parse_bitwise_and: [Caret => BitwiseXor],
    parse_bitwise_and -> parse_equality: [Ampersand => BitwiseAnd],
    parse_equality -> parse_relational: [EqualEqual => Equal, BangEqual => NotEqual],
    parse_relational -> parse_shift: [Less, Greater, LessEqual, GreaterEqual, In, Is],
    parse_shift -> parse_additive: [LessLess => LeftShift, GreaterGreater => RightShift],
    parse_additive -> parse_multiplicative: [Plus => Add, Minus => Subtract],
    parse_multiplicative -> parse_exponentiation: [Star => Multiply, Slash => Divide, Percent => Modulo]
)]
impl Parser {}
