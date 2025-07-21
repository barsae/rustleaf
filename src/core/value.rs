/// Value representation and type operations for RustLeaf
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use indexmap::IndexMap;

use super::ast;
use crate::eval::Scope;

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<IndexMap<String, Value>>>),
    Function(Function),
    Object(Rc<RefCell<Object>>),
    RustValue(Rc<RefCell<Box<dyn RustValue>>>),

    // Internal values for method dispatch
    BuiltinMethod(BuiltinMethod),
    BoundMethod(Box<Value>, BuiltinMethod), // self, method
}

#[derive(Clone, Debug)]
pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<ast::Statement>,
    pub closure: Rc<RefCell<Scope>>,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Function>,
}

#[derive(Clone, Debug)]
pub enum BuiltinMethod {
    // String methods
    StringLen,
    StringUpper,
    StringLower,
    StringSplit,

    // List methods
    ListLen,
    ListAppend,
    ListPop,

    // Dict methods
    DictLen,
    DictGet,
    DictSet,

    // Arithmetic operators
    IntAdd,
    IntSub,
    IntMul,
    IntDiv,
    IntMod,
    IntPow,
    IntNeg,
    FloatAdd,
    FloatSub,
    FloatMul,
    FloatDiv,
    FloatMod,
    FloatPow,
    FloatNeg,

    // Comparison operators
    IntEq,
    IntNe,
    IntLt,
    IntGt,
    IntLe,
    IntGe,
    FloatEq,
    FloatNe,
    FloatLt,
    FloatGt,
    FloatLe,
    FloatGe,

    // Type conversion
    IntStr,
    FloatStr,
    BoolStr,

    // Special methods
    OpGetAttr,
}

/// Trait for Rust-implemented custom types
pub trait RustValue: fmt::Debug {
    fn type_name(&self) -> &str;
    fn get_attr(&self, name: &str) -> Option<Value>;
    fn set_attr(&mut self, name: &str, value: Value) -> Result<(), String>;
}

/// Trait for type operations - all types implement this
pub trait TypeOps {
    fn get_attr(&self, name: &str) -> Option<Value>;
}

/// Get the type name of a value
pub fn type_of(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Unit => "unit",
        Value::Bool(_) => "bool",
        Value::Int(_) => "int",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::List(_) => "list",
        Value::Dict(_) => "dict",
        Value::Function(_) => "function",
        Value::Object(obj) => {
            // TODO: Return the actual class name
            "object"
        }
        Value::RustValue(rv) => {
            // TODO: Call rv.borrow().type_name()
            "rustvalue"
        }
        Value::BuiltinMethod(_) => "builtin_method",
        Value::BoundMethod(_, _) => "bound_method",
    }
}

/// Convert a value to a string representation
pub fn to_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Unit => "unit".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Int(n) => n.to_string(),
        Value::Float(f) => f.to_string(),
        Value::String(s) => s.clone(),
        Value::List(list) => {
            // TODO: Implement proper list display
            "[...]".to_string()
        }
        Value::Dict(dict) => {
            // TODO: Implement proper dict display
            "{...}".to_string()
        }
        Value::Function(_) => "<function>".to_string(),
        Value::Object(obj) => {
            // TODO: Implement proper object display
            "<object>".to_string()
        }
        Value::RustValue(rv) => {
            // TODO: Use Debug trait
            "<rustvalue>".to_string()
        }
        Value::BuiltinMethod(method) => format!("<builtin_method: {:?}>", method),
        Value::BoundMethod(_, method) => format!("<bound_method: {:?}>", method),
    }
}

// Type implementations for built-in types
impl TypeOps for i64 {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "op_add" => Some(Value::BuiltinMethod(BuiltinMethod::IntAdd)),
            "op_sub" => Some(Value::BuiltinMethod(BuiltinMethod::IntSub)),
            "op_mul" => Some(Value::BuiltinMethod(BuiltinMethod::IntMul)),
            "op_div" => Some(Value::BuiltinMethod(BuiltinMethod::IntDiv)),
            "op_mod" => Some(Value::BuiltinMethod(BuiltinMethod::IntMod)),
            "op_pow" => Some(Value::BuiltinMethod(BuiltinMethod::IntPow)),
            "op_neg" => Some(Value::BuiltinMethod(BuiltinMethod::IntNeg)),
            "op_eq" => Some(Value::BuiltinMethod(BuiltinMethod::IntEq)),
            "op_ne" => Some(Value::BuiltinMethod(BuiltinMethod::IntNe)),
            "op_lt" => Some(Value::BuiltinMethod(BuiltinMethod::IntLt)),
            "op_gt" => Some(Value::BuiltinMethod(BuiltinMethod::IntGt)),
            "op_le" => Some(Value::BuiltinMethod(BuiltinMethod::IntLe)),
            "op_ge" => Some(Value::BuiltinMethod(BuiltinMethod::IntGe)),
            "op_str" => Some(Value::BuiltinMethod(BuiltinMethod::IntStr)),
            "op_get_attr" => Some(Value::BuiltinMethod(BuiltinMethod::OpGetAttr)),
            _ => None,
        }
    }
}

impl TypeOps for f64 {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "op_add" => Some(Value::BuiltinMethod(BuiltinMethod::FloatAdd)),
            "op_sub" => Some(Value::BuiltinMethod(BuiltinMethod::FloatSub)),
            "op_mul" => Some(Value::BuiltinMethod(BuiltinMethod::FloatMul)),
            "op_div" => Some(Value::BuiltinMethod(BuiltinMethod::FloatDiv)),
            "op_mod" => Some(Value::BuiltinMethod(BuiltinMethod::FloatMod)),
            "op_pow" => Some(Value::BuiltinMethod(BuiltinMethod::FloatPow)),
            "op_neg" => Some(Value::BuiltinMethod(BuiltinMethod::FloatNeg)),
            "op_eq" => Some(Value::BuiltinMethod(BuiltinMethod::FloatEq)),
            "op_ne" => Some(Value::BuiltinMethod(BuiltinMethod::FloatNe)),
            "op_lt" => Some(Value::BuiltinMethod(BuiltinMethod::FloatLt)),
            "op_gt" => Some(Value::BuiltinMethod(BuiltinMethod::FloatGt)),
            "op_le" => Some(Value::BuiltinMethod(BuiltinMethod::FloatLe)),
            "op_ge" => Some(Value::BuiltinMethod(BuiltinMethod::FloatGe)),
            "op_str" => Some(Value::BuiltinMethod(BuiltinMethod::FloatStr)),
            "op_get_attr" => Some(Value::BuiltinMethod(BuiltinMethod::OpGetAttr)),
            _ => None,
        }
    }
}

impl TypeOps for String {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "len" => Some(Value::BuiltinMethod(BuiltinMethod::StringLen)),
            "upper" => Some(Value::BuiltinMethod(BuiltinMethod::StringUpper)),
            "lower" => Some(Value::BuiltinMethod(BuiltinMethod::StringLower)),
            "split" => Some(Value::BuiltinMethod(BuiltinMethod::StringSplit)),
            "op_get_attr" => Some(Value::BuiltinMethod(BuiltinMethod::OpGetAttr)),
            _ => None,
        }
    }
}

impl TypeOps for bool {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "op_str" => Some(Value::BuiltinMethod(BuiltinMethod::BoolStr)),
            "op_get_attr" => Some(Value::BuiltinMethod(BuiltinMethod::OpGetAttr)),
            _ => None,
        }
    }
}
