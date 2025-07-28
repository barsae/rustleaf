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
    Function(Box<FunctionData>),
    // Lambda expression
    Lambda(Box<LambdaData>),

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
    With(Box<WithData>), // resources as (name, value) pairs, body

    // Collections
    List(Box<Vec<Eval>>),
    Dict(Box<Vec<(Eval, Eval)>>),

    // Program - sequence of statements at the same scope level (no scope boundary)
    Program(Box<Vec<Eval>>),

    // Block - with optional terminal expression (creates new scope)
    Block(Box<Vec<Eval>>, Option<Box<Eval>>),

    // Built-in operations that don't use method dispatch
    LogicalAnd(Box<Eval>, Box<Eval>),
    LogicalOr(Box<Eval>, Box<Eval>),
    LogicalNot(Box<Eval>),
    Is(Box<Eval>, Box<Eval>),

    // Class support
    ClassDecl(Box<ClassDeclData>),

    // Module import
    Import(Box<ImportData>),

    // Match expression
    Match(Box<MatchData>),

    // Macro application - transforms an Eval node using a macro function
    Macro(Box<MacroData>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: Eval,
    pub is_static: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclData {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_defaults: Vec<Option<Eval>>,
    pub methods: Vec<ClassMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportData {
    pub module: String,
    pub items: ImportItems,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchData {
    pub expr: Box<Eval>,
    pub cases: Vec<EvalMatchCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MacroData {
    pub macro_fn: Box<Eval>, // The macro function to call
    pub target: Box<Eval>,   // The Eval node to transform
    pub args: Vec<Eval>,     // Macro arguments
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionData {
    pub name: String,
    pub params: Vec<(String, Option<Value>, ParameterKind)>,
    pub body: Box<Eval>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaData {
    pub params: Vec<String>,
    pub body: Box<Eval>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithData {
    pub resources: Vec<(String, Eval)>,
    pub body: Box<Eval>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn eval_enum_size() {
        println!("Eval enum size: {} bytes", mem::size_of::<Eval>());
        assert!(mem::size_of::<Eval>() > 0);
    }
}
