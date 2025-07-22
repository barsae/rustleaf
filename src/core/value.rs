use anyhow::{anyhow, Result};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use indexmap::IndexMap;

use crate::core::Args;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ListRef(Rc<RefCell<Vec<Value>>>);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DictRef(Rc<RefCell<IndexMap<String, Value>>>);

#[derive(Clone, Debug)]
pub struct RustValueRef(Rc<RefCell<Box<dyn RustValue>>>);

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
    fn get_attr(&self, name: &str) -> Option<Value>;
    fn set_attr(&mut self, name: &str, value: Value) -> Result<(), String>;
    fn call(&self, args: Args) -> Result<Value> {
        Err(anyhow!("this isn't callable"))
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

    pub fn rust_value(val: Box<dyn RustValue>) -> Self {
        Value::RustValue(RustValueRef(Rc::new(RefCell::new(val))))
    }

    pub fn get_attr(&self, name: &str) -> Option<Value> {
        match self {
            Value::RustValue(rv) => rv.0.borrow().get_attr(name),
            _ => None,
        }
    }
}