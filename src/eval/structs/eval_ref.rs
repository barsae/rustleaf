/// Data structures for evaluation AST nodes
use crate::core::{ImportItems, ParameterKind, RustValueRef, Value};

#[derive(Debug, Clone)]
pub struct ClassMethod {
    pub name: String,
    pub params: Vec<String>,
    pub body: RustValueRef,
    pub is_static: bool,
}

#[derive(Debug, Clone)]
pub struct ClassDeclData {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_defaults: Vec<Option<RustValueRef>>,
    pub methods: Vec<ClassMethod>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportData {
    pub module: String,
    pub items: ImportItems,
}

#[derive(Debug, Clone)]
pub struct MatchData {
    pub expr: RustValueRef,
    pub cases: Vec<EvalMatchCase>,
}

#[derive(Debug, Clone)]
pub struct MacroData {
    pub macro_fn: RustValueRef,  // The macro function to call
    pub target: RustValueRef,    // The Eval node to transform
    pub args: Vec<RustValueRef>, // Macro arguments
}

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub name: String,
    pub params: Vec<(String, Option<Value>, ParameterKind)>,
    pub body: RustValueRef,
}

#[derive(Debug, Clone)]
pub struct LambdaData {
    pub params: Vec<String>,
    pub body: RustValueRef,
}

#[derive(Debug, Clone)]
pub struct WithData {
    pub resources: Vec<(String, RustValueRef)>,
    pub body: RustValueRef,
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

#[derive(Debug, Clone)]
pub struct EvalMatchCase {
    pub pattern: EvalMatchPattern,
    pub guard: Option<RustValueRef>,
    pub body: RustValueRef,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EvalMatchPattern {
    Literal(Value),
    Variable(String),
    Wildcard,
}
