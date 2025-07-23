use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use indexmap::IndexMap;

use crate::core::Args;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListRef(Rc<RefCell<Vec<Value>>>);

impl ListRef {
    pub fn borrow(&self) -> std::cell::Ref<Vec<Value>> {
        self.0.borrow()
    }
    
    pub fn borrow_mut(&self) -> std::cell::RefMut<Vec<Value>> {
        self.0.borrow_mut()
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DictRef(Rc<RefCell<IndexMap<String, Value>>>);

impl DictRef {
    pub fn borrow(&self) -> std::cell::Ref<IndexMap<String, Value>> {
        self.0.borrow()
    }
    
    pub fn borrow_mut(&self) -> std::cell::RefMut<IndexMap<String, Value>> {
        self.0.borrow_mut()
    }
}

#[derive(Clone, Debug)]
pub struct RustValueRef(Rc<RefCell<Box<dyn RustValue>>>);

impl RustValueRef {
    pub fn borrow_mut(&self) -> std::cell::RefMut<Box<dyn RustValue>> {
        self.0.borrow_mut()
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    List(ListRef),
    Dict(DictRef),
    RustValue(RustValueRef),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) => true,
            (Value::Unit, Value::Unit) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::List(a), Value::List(b)) => Rc::ptr_eq(&a.0, &b.0),
            (Value::Dict(a), Value::Dict(b)) => Rc::ptr_eq(&a.0, &b.0),
            (Value::RustValue(a), Value::RustValue(b)) => Rc::ptr_eq(&a.0, &b.0),
            _ => false,
        }
    }
}

pub trait RustValue: fmt::Debug {
    fn get_attr(&self, _name: &str) -> Option<Value> {
        None
    }
    fn set_attr(&mut self, _name: &str, _value: Value) -> Result<(), String> {
        Err("Cannot set attributes on this type".to_string())
    }
    fn call(&self, args: Args) -> Result<Value> {
        Err(anyhow!("Cannot call this type"))
    }
}

impl Value {
    pub fn is_truthy(&self) -> Option<bool> {
        match self {
            Value::Null => Some(false),
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn new_list() -> Self {
        Value::List(ListRef::default())
    }

    pub fn new_dict() -> Self {
        Value::Dict(DictRef::default())
    }

    pub fn new_list_with_values(values: Vec<Value>) -> Self {
        Value::List(ListRef(Rc::new(RefCell::new(values))))
    }

    pub fn new_dict_with_map(map: indexmap::IndexMap<String, Value>) -> Self {
        Value::Dict(DictRef(Rc::new(RefCell::new(map))))
    }

    pub fn rust_value(val: Box<dyn RustValue>) -> Self {
        Value::RustValue(RustValueRef(Rc::new(RefCell::new(val))))
    }

    pub fn from_rust<T: RustValue + 'static>(val: T) -> Self {
        Value::RustValue(RustValueRef(Rc::new(RefCell::new(Box::new(val)))))
    }

    pub fn bind_method(&self, method_func: fn(&Value, crate::core::Args) -> anyhow::Result<Value>) -> Self {
        use crate::core::builtin_ops::BoundMethod;
        Value::from_rust(BoundMethod::new(self, method_func))
    }

    pub fn get_attr(&self, name: &str) -> Option<Value> {
        match self {
            Value::RustValue(rv) => rv.0.borrow().get_attr(name),
            Value::Int(_) => self.get_int_attr(name),
            Value::Float(_) => self.get_float_attr(name),
            Value::String(_) => self.get_string_attr(name),
            Value::Bool(_) => self.get_bool_attr(name),
            Value::List(_) => self.get_list_attr(name),
            Value::Dict(_) => self.get_dict_attr(name),
            _ => None,
        }
    }

    pub fn call(&self, args: Args) -> Result<Value> {
        match self {
            Value::RustValue(rv) => rv.0.borrow().call(args),
            _ => Err(anyhow!("Value is not callable: {:?}", self)),
        }
    }

    // Operator method implementations for built-in types
    fn get_int_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_add" => Some(self.bind_method(op_add)),
            "op_sub" => Some(self.bind_method(op_sub)),
            "op_mul" => Some(self.bind_method(op_mul)),
            "op_div" => Some(self.bind_method(op_div)),
            "op_neg" => Some(self.bind_method(op_neg)),
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            "op_lt" => Some(self.bind_method(op_lt)),
            "op_le" => Some(self.bind_method(op_le)),
            "op_gt" => Some(self.bind_method(op_gt)),
            "op_ge" => Some(self.bind_method(op_ge)),
            "op_mod" => Some(self.bind_method(op_mod)),
            "op_pow" => Some(self.bind_method(op_pow)),
            "op_bitwise_and" => Some(self.bind_method(op_bitwise_and)),
            "op_bitwise_or" => Some(self.bind_method(op_bitwise_or)),
            "op_bitwise_xor" => Some(self.bind_method(op_bitwise_xor)),
            "op_bitwise_not" => Some(self.bind_method(op_bitwise_not)),
            "op_lshift" => Some(self.bind_method(op_lshift)),
            "op_rshift" => Some(self.bind_method(op_rshift)),
            _ => None,
        }
    }

    fn get_float_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_add" => Some(self.bind_method(op_add)),
            "op_sub" => Some(self.bind_method(op_sub)),
            "op_mul" => Some(self.bind_method(op_mul)),
            "op_div" => Some(self.bind_method(op_div)),
            "op_neg" => Some(self.bind_method(op_neg)),
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            "op_lt" => Some(self.bind_method(op_lt)),
            "op_le" => Some(self.bind_method(op_le)),
            "op_gt" => Some(self.bind_method(op_gt)),
            "op_ge" => Some(self.bind_method(op_ge)),
            "op_mod" => Some(self.bind_method(op_mod)),
            "op_pow" => Some(self.bind_method(op_pow)),
            _ => None,
        }
    }

    fn get_string_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_add" => Some(self.bind_method(op_add)), // String concatenation
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            "op_lt" => Some(self.bind_method(op_lt)),
            "op_le" => Some(self.bind_method(op_le)),
            "op_gt" => Some(self.bind_method(op_gt)),
            "op_ge" => Some(self.bind_method(op_ge)),
            "op_contains" => Some(self.bind_method(op_contains)),
            _ => None,
        }
    }

    fn get_bool_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            _ => None,
        }
    }

    fn get_list_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_contains" => Some(self.bind_method(op_contains)),
            "op_get_item" => Some(self.bind_method(op_get_item_list)),
            "op_set_item" => Some(self.bind_method(op_set_item_list)),
            "op_eq" => Some(self.bind_method(op_eq)),
            _ => None,
        }
    }

    fn get_dict_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_contains" => Some(self.bind_method(op_contains)),
            "op_get_item" => Some(self.bind_method(op_get_item_dict)),
            "op_set_item" => Some(self.bind_method(op_set_item_dict)),
            _ => None,
        }
    }
}