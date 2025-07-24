/// Core evaluation types - simplified AST for execution
use crate::core::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Eval {
    // Primitives
    Literal(Value),
    Variable(String),

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
    // Lambda expression
    Lambda(Vec<String>, Box<Eval>),

    // Control flow
    If(Box<Eval>, Box<Eval>, Option<Box<Eval>>),
    While(Box<Eval>, Box<Eval>),       // condition, body
    For(String, Box<Eval>, Box<Eval>), // variable name, iterator, body
    Loop(Box<Eval>),
    Return(Option<Box<Eval>>),
    Break(Option<Box<Eval>>),
    Continue,

    // Error handling
    Try(Box<Eval>, String, Box<Eval>), // body, catch_var, catch_body

    // Resource management
    With(Vec<(String, Eval)>, Box<Eval>), // resources as (name, value) pairs, body

    // Collections
    List(Vec<Eval>),
    Dict(Vec<(Eval, Eval)>),

    // Block - with optional terminal expression
    Block(Vec<Eval>, Option<Box<Eval>>),

    // Built-in operations that don't use method dispatch
    LogicalAnd(Box<Eval>, Box<Eval>),
    LogicalOr(Box<Eval>, Box<Eval>),
    LogicalNot(Box<Eval>),
    Is(Box<Eval>, Box<Eval>),

    // Class support
    ClassDecl {
        name: String,
        field_names: Vec<String>,
        field_defaults: Vec<Option<Eval>>,
        methods: Vec<ClassMethod>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Eval,
    pub is_static: bool,
}
