#![allow(clippy::result_large_err)]
#![allow(unused_imports)]

use crate::eval::scope::Scope;
use crate::parser::{AstNode, Parameter};
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(Rc<RefCell<Vec<Value>>>),
    Dict(Rc<RefCell<HashMap<String, Value>>>),
    Function(Function),
    Object(Rc<RefCell<Object>>),
    ClassInstance(Rc<RefCell<crate::eval::statements::ClassInstance>>),
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
    pub fn type_name(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Unit => "unit".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Int(_) => "int".to_string(),
            Value::Float(_) => "float".to_string(),
            Value::String(_) => "string".to_string(),
            Value::List(_) => "list".to_string(),
            Value::Dict(_) => "dict".to_string(),
            Value::Function(_) => "function".to_string(),
            Value::Object(obj_ref) => obj_ref.borrow().class_name.clone(),
            Value::ClassInstance(instance_ref) => instance_ref.borrow().class_name.clone(),
            Value::RustValue(rv) => rv.type_name().to_string(),
            Value::BoundMethod(_, _) => "bound_method".to_string(),
            Value::UnboundMethod(_) => "unbound_method".to_string(),
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
            Value::List(list_ref) => {
                let list = list_ref.borrow();
                let items: Vec<String> = list.iter().map(|v| v.to_display_string()).collect();
                format!("[{}]", items.join(", "))
            }
            Value::Dict(dict_ref) => {
                let dict = dict_ref.borrow();
                let items: Vec<String> = dict
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.to_display_string()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Function(f) => match &f.name {
                Some(name) => format!("<function {}>", name),
                None => "<anonymous function>".to_string(),
            },
            Value::Object(obj_ref) => {
                let obj = obj_ref.borrow();
                format!("<{} object>", obj.class_name)
            }
            Value::ClassInstance(instance_ref) => {
                let instance = instance_ref.borrow();
                format!("<{} object>", instance.class_name)
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
            Value::ClassInstance(_) => {
                // ClassInstance objects don't have a meta-class yet
                return None;
            }
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
        if let Value::Object(obj_ref) = self {
            let obj = obj_ref.borrow();
            if let Some(value) = obj.fields.get(attr_name) {
                return Ok(value.clone());
            }
        }

        // 1.1. Check ClassInstance properties
        if let Value::ClassInstance(instance_ref) = self {
            let instance = instance_ref.borrow();
            if let Some(value) = instance.fields.get(attr_name) {
                return Ok(value.clone());
            }
        }

        // 2. Check class object
        if let Some(Value::Object(class_ref)) = self.get_class(env) {
            let class = class_ref.borrow();
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

    pub fn new_list(items: Vec<Value>) -> Self {
        Value::List(Rc::new(RefCell::new(items)))
    }

    pub fn new_dict(items: HashMap<String, Value>) -> Self {
        Value::Dict(Rc::new(RefCell::new(items)))
    }

    pub fn new_object(object: Object) -> Self {
        Value::Object(Rc::new(RefCell::new(object)))
    }

    pub fn with_list<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&Vec<Value>) -> R,
    {
        match self {
            Value::List(list_ref) => Ok(f(&list_ref.borrow())),
            _ => Err(RuntimeError::new(
                format!("Expected list, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn with_list_mut<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&mut Vec<Value>) -> R,
    {
        match self {
            Value::List(list_ref) => Ok(f(&mut list_ref.borrow_mut())),
            _ => Err(RuntimeError::new(
                format!("Expected list, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn with_dict<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&HashMap<String, Value>) -> R,
    {
        match self {
            Value::Dict(dict_ref) => Ok(f(&dict_ref.borrow())),
            _ => Err(RuntimeError::new(
                format!("Expected dict, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn with_dict_mut<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&mut HashMap<String, Value>) -> R,
    {
        match self {
            Value::Dict(dict_ref) => Ok(f(&mut dict_ref.borrow_mut())),
            _ => Err(RuntimeError::new(
                format!("Expected dict, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn with_object<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&Object) -> R,
    {
        match self {
            Value::Object(obj_ref) => Ok(f(&obj_ref.borrow())),
            _ => Err(RuntimeError::new(
                format!("Expected object, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn with_object_mut<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&mut Object) -> R,
    {
        match self {
            Value::Object(obj_ref) => Ok(f(&mut obj_ref.borrow_mut())),
            _ => Err(RuntimeError::new(
                format!("Expected object, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn new_class_instance(instance: crate::eval::statements::ClassInstance) -> Self {
        Value::ClassInstance(Rc::new(RefCell::new(instance)))
    }

    pub fn with_class_instance<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&crate::eval::statements::ClassInstance) -> R,
    {
        match self {
            Value::ClassInstance(instance_ref) => Ok(f(&instance_ref.borrow())),
            _ => Err(RuntimeError::new(
                format!("Expected class instance, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
        }
    }

    pub fn with_class_instance_mut<F, R>(&self, f: F) -> Result<R, RuntimeError>
    where
        F: FnOnce(&mut crate::eval::statements::ClassInstance) -> R,
    {
        match self {
            Value::ClassInstance(instance_ref) => Ok(f(&mut instance_ref.borrow_mut())),
            _ => Err(RuntimeError::new(
                format!("Expected class instance, found {}", self.type_name()),
                ErrorType::TypeError,
            )),
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
