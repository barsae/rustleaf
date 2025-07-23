/// Core evaluation types - simplified AST for execution
use crate::core::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Eval {
    // Primitives
    Literal(Value),
    Variable(String),

    // Basic operations
    BinaryOp(BinaryOp, Box<Eval>, Box<Eval>),
    UnaryOp(UnaryOp, Box<Eval>),

    // Function calls (all calls - methods, functions, etc.)
    Call(Box<Eval>, Vec<Eval>),

    // Property access
    GetAttr(Box<Eval>, String),
    GetItem(Box<Eval>, Box<Eval>),

    // Assignment
    Assign(String, Box<Eval>),
    SetAttr(Box<Eval>, String, Box<Eval>),
    SetItem(Box<Eval>, Box<Eval>, Box<Eval>),

    // Variable declaration
    Declare(String, Option<Box<Eval>>),

    // Function declaration
    Function(String, Vec<String>, Box<Eval>),

    // Control flow
    If(Box<Eval>, Box<Eval>, Option<Box<Eval>>),
    Loop(Box<Eval>),
    Return(Option<Box<Eval>>),
    Break(Option<Box<Eval>>),
    Continue,

    // Collections
    List(Vec<Eval>),
    Dict(Vec<(Eval, Eval)>),

    // Block - with optional terminal expression
    Block(Vec<Eval>, Option<Box<Eval>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Pow,

    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,

    // Logical
    And, Or,

    // Membership
    In, Is,

    // Bitwise
    BitAnd, BitOr, BitXor, LeftShift, RightShift,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,    // -expr
    Not,    // not expr
    BitNot, // ~expr
}