#![allow(clippy::result_large_err)]
#![allow(unused_imports)]

use crate::eval::scope::Scope;
use crate::parser::{AstNode, Parameter};
use std::any::Any;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
    Function(Function),
    Object(Object),
    RustValue(Box<dyn RustValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: Option<String>,
    pub parameters: Vec<Parameter>,
    pub body: AstNode,
    pub closure: Option<Scope>,
    pub is_builtin: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub methods: HashMap<String, Function>,
}

pub trait RustValue: fmt::Debug {
    fn type_name(&self) -> &'static str;
    fn to_string(&self) -> String;
    fn clone_box(&self) -> Box<dyn RustValue>;
    fn as_any(&self) -> &dyn std::any::Any;

    // Optional iteration methods - default to None for non-iterable types
    fn iter_next(&mut self) -> Option<Value> {
        None
    }

    fn iter_reset(&mut self) {
        // Default no-op
    }

    fn is_iterable(&self) -> bool {
        false
    }

    fn len(&self) -> Option<i64> {
        None
    }

    fn is_empty(&self) -> bool {
        self.len() == Some(0)
    }
}

impl Clone for Box<dyn RustValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for Box<dyn RustValue> {
    fn eq(&self, other: &Self) -> bool {
        // For now, RustValues are equal only if they're the same instance
        std::ptr::eq(self.as_ref(), other.as_ref())
    }
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Null => "null",
            Value::Unit => "unit",
            Value::Bool(_) => "bool",
            Value::Int(_) => "int",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::List(_) => "list",
            Value::Dict(_) => "dict",
            Value::Function(_) => "function",
            Value::Object(obj) => &obj.class_name,
            Value::RustValue(rv) => rv.type_name(),
        }
    }

    pub fn is_truthy(&self) -> Result<bool, RuntimeError> {
        match self {
            Value::Null => Ok(false),
            Value::Bool(b) => Ok(*b),
            Value::Unit => Err(RuntimeError::new(
                "unit type not allowed in boolean context".to_string(),
                ErrorType::TypeError,
            )),
            _ => Err(RuntimeError::new(
                format!("{} has no truthiness", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Unit => "<unit>".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::List(elements) => {
                let items: Vec<String> = elements.iter().map(|v| v.to_display_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Dict(map) => {
                let items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_display_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Function(f) => match &f.name {
                Some(name) => format!("<function {}>", name),
                None => "<anonymous function>".to_string(),
            },
            Value::Object(obj) => {
                format!("<{} object>", obj.class_name)
            }
            Value::RustValue(rv) => rv.to_string(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_display_string())
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub message: String,
    pub error_type: ErrorType,
    pub stack_trace: Vec<String>,
    pub return_value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    TypeError,
    ValueError,
    IndexError,
    KeyError,
    AttributeError,
    NameError,
    ZeroDivisionError,
    RecursionError,
    ImportError,
    RuntimeError,
    AssertionError,
    RaisedError,
}

impl RuntimeError {
    pub fn new(message: String, error_type: ErrorType) -> Self {
        RuntimeError {
            message,
            error_type,
            stack_trace: Vec::new(),
            return_value: None,
        }
    }

    pub fn with_stack_trace(mut self, trace: Vec<String>) -> Self {
        self.stack_trace = trace;
        self
    }

    pub fn with_return_value(mut self, value: Value) -> Self {
        self.return_value = Some(value);
        self
    }

    pub fn is_return(&self) -> bool {
        self.message == "RETURN_VALUE"
            || self.message == "RETURN_NULL"
            || self.message == "RETURN_UNIT"
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)?;
        if !self.stack_trace.is_empty() {
            write!(f, "\nStack trace:")?;
            for frame in &self.stack_trace {
                write!(f, "\n  {}", frame)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for RuntimeError {}
