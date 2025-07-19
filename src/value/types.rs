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
    BoundMethod(Function, Box<Value>), // method + bound object
    UnboundMethod(Function),           // method that needs binding
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

    // Property/field assignment method
    fn op_set(&mut self, _property: &str, _value: Value) -> Result<(), RuntimeError> {
        Err(RuntimeError::new(
            format!(
                "'{}' object does not support property assignment",
                self.type_name()
            ),
            ErrorType::TypeError,
        ))
    }

    // Index assignment method
    fn op_index_set(&mut self, _index: Value, _value: Value) -> Result<(), RuntimeError> {
        Err(RuntimeError::new(
            format!(
                "'{}' object does not support index assignment",
                self.type_name()
            ),
            ErrorType::TypeError,
        ))
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
            Value::BoundMethod(_, _) => "bound_method",
            Value::UnboundMethod(_) => "unbound_method",
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
            Value::BoundMethod(f, _) => match &f.name {
                Some(name) => format!("<bound method {}>", name),
                None => "<bound method>".to_string(),
            },
            Value::UnboundMethod(f) => match &f.name {
                Some(name) => format!("<unbound method {}>", name),
                None => "<unbound method>".to_string(),
            },
        }
    }

    pub fn get_class(&self, env: &crate::eval::environment::Environment) -> Option<Value> {
        let class_name = match self {
            Value::String(_) => "String",
            Value::List(_) => "List",
            Value::Dict(_) => "Dict",
            Value::RustValue(rv) => {
                // Check if this is a range object
                if rv.type_name() == "range" {
                    "Range"
                } else {
                    // Other RustValue types don't have classes yet
                    return None;
                }
            }
            // Other types don't have classes yet
            _ => return None,
        };

        // Look up class object in global scope
        env.get(class_name).ok()
    }

    pub fn op_get_attr(
        &self,
        attr_name: &str,
        env: &crate::eval::environment::Environment,
    ) -> Result<Value, RuntimeError> {
        // 1. Check instance properties first (for objects)
        if let Value::Object(obj) = self {
            if let Some(value) = obj.fields.get(attr_name) {
                return Ok(value.clone());
            }
        }

        // 2. Check class object
        if let Some(Value::Object(class)) = self.get_class(env) {
            if let Some(value) = class.fields.get(attr_name) {
                return Ok(value.clone());
            }
        }

        // 3. Not found
        Err(RuntimeError::new(
            format!(
                "'{}' object has no attribute '{}'",
                self.type_name(),
                attr_name
            ),
            ErrorType::AttributeError,
        ))
    }

    pub fn bind_method(&self, method: Function) -> Value {
        Value::BoundMethod(method, Box::new(self.clone()))
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
