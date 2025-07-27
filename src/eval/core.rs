/// Core evaluation types - simplified AST for execution
use crate::core::{ImportItems, ParameterKind, Value};

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

    // Pattern-based declaration
    DeclarePattern(EvalPattern, Box<Eval>),

    // Function declaration
    Function(
        String,
        Vec<(String, Option<Value>, ParameterKind)>,
        Box<Eval>,
    ), // name, params with defaults and kinds, body
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

    // Program - sequence of statements at the same scope level (no scope boundary)
    Program(Vec<Eval>),

    // Block - with optional terminal expression (creates new scope)
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

    // Module import
    Import {
        module: String,
        items: ImportItems,
    },

    // Match expression
    Match {
        expr: Box<Eval>,
        cases: Vec<EvalMatchCase>,
    },

    // Macro application - transforms an Eval node using a macro function
    Macro {
        macro_fn: Box<Eval>, // The macro function to call
        target: Box<Eval>,   // The Eval node to transform
        args: Vec<Eval>,     // Macro arguments
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Eval,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalPattern {
    Variable(String),
    List(Vec<EvalPattern>),
    ListRest(Vec<EvalPattern>, Option<String>),
    Dict(Vec<EvalDictPattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvalDictPattern {
    pub key: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EvalMatchCase {
    pub pattern: EvalMatchPattern,
    pub guard: Option<Eval>,
    pub body: Eval,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalMatchPattern {
    Literal(Value),
    Variable(String),
    Wildcard,
    // Future: structured patterns like List, Dict
}
