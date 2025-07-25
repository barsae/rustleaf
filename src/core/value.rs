use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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
pub struct ClassInstanceRef(Rc<RefCell<crate::eval::ClassInstance>>);

impl ClassInstanceRef {
    pub fn borrow(&self) -> std::cell::Ref<crate::eval::ClassInstance> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<crate::eval::ClassInstance> {
        self.0.borrow_mut()
    }
}

#[derive(Clone, Debug)]
pub struct ClassRef(Rc<RefCell<crate::eval::Class>>);

impl ClassRef {
    pub fn borrow(&self) -> std::cell::Ref<crate::eval::Class> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<crate::eval::Class> {
        self.0.borrow_mut()
    }
}

#[derive(Clone, Debug)]
pub struct RustValueRef(Rc<RefCell<Box<dyn RustValue>>>);

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
    pub inclusive: bool,
}

impl RustValueRef {
    pub fn borrow(&self) -> std::cell::Ref<Box<dyn RustValue>> {
        self.0.borrow()
    }

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
    Class(ClassRef),
    ClassInstance(ClassInstanceRef),
    Range(Range),
    RustValue(RustValueRef),
    Raised(Box<Value>),
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
            (Value::Class(a), Value::Class(b)) => Rc::ptr_eq(&a.0, &b.0),
            (Value::ClassInstance(a), Value::ClassInstance(b)) => Rc::ptr_eq(&a.0, &b.0),
            (Value::Range(a), Value::Range(b)) => a == b,
            (Value::RustValue(a), Value::RustValue(b)) => Rc::ptr_eq(&a.0, &b.0),
            (Value::Raised(a), Value::Raised(b)) => a == b,
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

    fn call(&self, _args: Args) -> Result<Value> {
        Err(anyhow!("Cannot call this type"))
    }

    fn op_is(&self, _other: &Value) -> Result<Value> {
        Err(anyhow!("This type does not support 'is' operations"))
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

    pub fn new_class_instance(instance: crate::eval::ClassInstance) -> Self {
        Value::ClassInstance(ClassInstanceRef(Rc::new(RefCell::new(instance))))
    }

    pub fn new_class(class: crate::eval::Class) -> Self {
        Value::Class(ClassRef(Rc::new(RefCell::new(class))))
    }

    pub fn rust_value(val: Box<dyn RustValue>) -> Self {
        Value::RustValue(RustValueRef(Rc::new(RefCell::new(val))))
    }

    pub fn from_rust<T: RustValue + 'static>(val: T) -> Self {
        Value::RustValue(RustValueRef(Rc::new(RefCell::new(Box::new(val)))))
    }

    pub fn bind_method(
        &self,
        method_func: fn(&Value, crate::core::Args) -> anyhow::Result<Value>,
    ) -> Self {
        use crate::core::builtin_ops::BoundMethod;
        Value::from_rust(BoundMethod::new(self, method_func))
    }

    pub fn get_attr(&self, name: &str, eval: &mut crate::eval::Evaluator) -> Option<Value> {
        match self {
            Value::RustValue(rv) => rv.0.borrow().get_attr(name),
            Value::ClassInstance(ci) => {
                let borrowed = ci.borrow();

                // First check for fields
                if let Some(field_value) = borrowed.fields.get(name) {
                    return Some(field_value.clone());
                }

                // Check for methods
                if let Some(method) = borrowed.find_method(name) {
                    // Create BoundMethod with the ClassInstanceRef (not a clone)
                    let bound_method = crate::eval::BoundMethod {
                        instance: self.clone(), // Clone the Value::ClassInstance, which shares the Rc
                        method: method.clone(),
                        closure_env: eval.globals.clone(),
                    };
                    return Some(Value::from_rust(bound_method));
                }

                None
            }
            Value::Int(_) => self.get_int_attr(name),
            Value::Float(_) => self.get_float_attr(name),
            Value::String(_) => self.get_string_attr(name),
            Value::Bool(_) => self.get_bool_attr(name),
            Value::List(_) => self.get_list_attr(name),
            Value::Dict(_) => self.get_dict_attr(name),
            Value::Class(_) => None, // Classes don't have attributes, they're only callable
            Value::Range(_) => self.get_range_attr(name),
            Value::Unit => self.get_unit_attr(name),
            Value::Null => self.get_null_attr(name),
            Value::Raised(_) => None, // Error values don't have attributes
        }
    }

    pub fn set_attr(&self, name: &str, value: Value) -> Result<(), String> {
        match self {
            Value::RustValue(rv) => rv.0.borrow_mut().set_attr(name, value),
            Value::ClassInstance(ci) => {
                // Allow setting fields on class instances
                ci.borrow_mut().fields.insert(name.to_string(), value);
                Ok(())
            }
            _ => Err("Cannot set attributes on this type".to_string()),
        }
    }

    pub fn call(&self, args: Args) -> Result<Value> {
        match self {
            Value::RustValue(rv) => rv.0.borrow().call(args),
            Value::Class(_) => {
                // Class constructor calls should be handled by the evaluator
                // This method shouldn't be called directly for classes
                Err(anyhow!(
                    "Class constructor calls must be handled by evaluator"
                ))
            }
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
            "length" => {
                if let Value::List(list_ref) = self {
                    let list = list_ref.borrow();
                    Some(Value::Int(list.len() as i64))
                } else {
                    None
                }
            }
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
            "op_eq" => Some(self.bind_method(op_eq)),
            _ => None,
        }
    }

    fn get_range_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "start" => {
                if let Value::Range(range) = self {
                    Some(Value::Int(range.start))
                } else {
                    None
                }
            }
            "end" => {
                if let Value::Range(range) = self {
                    Some(Value::Int(range.end))
                } else {
                    None
                }
            }
            "inclusive" => {
                if let Value::Range(range) = self {
                    Some(Value::Bool(range.inclusive))
                } else {
                    None
                }
            }
            "op_contains" => Some(self.bind_method(op_contains)),
            "op_eq" => Some(self.bind_method(op_eq)),
            "to_list" => Some(self.bind_method(range_to_list)),
            _ => None,
        }
    }

    fn get_unit_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            _ => None,
        }
    }

    fn get_null_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtin_ops::*;

        match name {
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            _ => None,
        }
    }
}
