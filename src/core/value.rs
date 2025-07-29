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

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: i64,
    pub inclusive: bool,
}

#[derive(Debug)]
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
    RustValue(Box<dyn RustValue>),
    Raised(Box<Value>),
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Null => Value::Null,
            Value::Unit => Value::Unit,
            Value::Bool(b) => Value::Bool(*b),
            Value::Int(i) => Value::Int(*i),
            Value::Float(f) => Value::Float(*f),
            Value::String(s) => Value::String(s.clone()),
            Value::List(list) => Value::List(list.clone()),
            Value::Dict(dict) => Value::Dict(dict.clone()),
            Value::Class(class) => Value::Class(class.clone()),
            Value::ClassInstance(instance) => Value::ClassInstance(instance.clone()),
            Value::Range(range) => Value::Range(range.clone()),
            Value::RustValue(rust_val) => Value::RustValue(rust_val.dyn_clone()),
            Value::Raised(raised) => Value::Raised(raised.clone()),
        }
    }
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
            (Value::RustValue(a), Value::RustValue(b)) => std::ptr::eq(a.as_ref(), b.as_ref()),
            (Value::Raised(a), Value::Raised(b)) => a == b,
            _ => false,
        }
    }
}

pub trait RustValue: fmt::Debug + 'static {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn dyn_clone(&self) -> Box<dyn RustValue>;

    /// Get the type name for this RustValue
    fn type_name(&self) -> Option<&str> {
        None
    }

    fn get_attr(&self, _name: &str) -> Option<Value> {
        None
    }

    fn set_attr(&mut self, _name: &str, _value: Value) -> Result<(), String> {
        Err("Cannot set attributes on this type".to_string())
    }

    fn call(&self, _args: Args) -> Result<Value> {
        Err(anyhow!("This type is not callable"))
    }

    fn op_is(&self, _other: &Value) -> Result<Value> {
        Err(anyhow!("This type does not support 'is' operations"))
    }

    fn op_iter(&self) -> Result<Value> {
        Err(anyhow!("This type is not iterable"))
    }

    fn str(&self) -> String {
        format!("{:?}", self)
    }

    fn op_next(&mut self) -> Result<Option<Value>> {
        Err(anyhow!("This type is not an iterator"))
    }

    fn eval(&self, _evaluator: &mut crate::eval::Evaluator) -> Result<crate::eval::EvalResult> {
        Err(anyhow!("This type cannot be evaluated"))
    }
}

impl Value {
    pub fn str(&self) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::Unit => "unit".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::RustValue(rust_val) => rust_val.str(),
            _ => format!("{:?}", self), // Fallback to debug representation for other types
        }
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null | Value::Unit => false,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::List(list) => !list.borrow().is_empty(),
            Value::Dict(dict) => !dict.borrow().is_empty(),
            _ => true, // Other values (Range, RustValue, Class, ClassInstance) are truthy
        }
    }

    /// Convert Value to f64 if it's a numeric type
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Int(n) => Some(*n as f64),
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Convert Value to i64 if it's an integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }

    /// Convert Value to bool if it's a boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert Value to String if it's a string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Extract f64 with error message for method calls
    pub fn expect_f64(&self, method_name: &str, arg_name: &str) -> Result<f64> {
        self.as_f64().ok_or_else(|| {
            anyhow!(
                "{}(): {} must be a number, got {}",
                method_name,
                arg_name,
                self.type_name()
            )
        })
    }

    /// Extract i64 with error message for method calls  
    pub fn expect_i64(&self, method_name: &str, arg_name: &str) -> Result<i64> {
        self.as_i64().ok_or_else(|| {
            anyhow!(
                "{}(): {} must be an integer, got {}",
                method_name,
                arg_name,
                self.type_name()
            )
        })
    }

    /// Extract bool with error message for method calls
    pub fn expect_bool(&self, method_name: &str, arg_name: &str) -> Result<bool> {
        self.as_bool().ok_or_else(|| {
            anyhow!(
                "{}(): {} must be a boolean, got {}",
                method_name,
                arg_name,
                self.type_name()
            )
        })
    }

    /// Extract string with error message for method calls
    pub fn expect_string(&self, method_name: &str, arg_name: &str) -> Result<&str> {
        self.as_string().ok_or_else(|| {
            anyhow!(
                "{}(): {} must be a string, got {}",
                method_name,
                arg_name,
                self.type_name()
            )
        })
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
        Value::RustValue(val)
    }

    pub fn from_rust<T: RustValue + 'static>(val: T) -> Self {
        Value::RustValue(Box::new(val))
    }

    pub fn bind_method(
        &self,
        method_func: fn(&Value, crate::core::Args) -> anyhow::Result<Value>,
    ) -> Self {
        use crate::core::builtins::BoundMethod;
        Value::from_rust(BoundMethod::new(self, method_func))
    }

    /// Try to downcast a RustValue to a concrete type
    pub fn downcast_rust_value<T: RustValue + 'static>(&self) -> Option<&T> {
        if let Value::RustValue(b) = self {
            Some(b.as_any().downcast_ref::<T>().unwrap())
        } else {
            None
        }
    }

    /// Downcast a RustValue with better error messages for method calls
    pub fn expect_rust_value<T: RustValue + 'static>(
        &self,
        method_name: &str,
        expected_type: &str,
    ) -> Result<&T> {
        self.downcast_rust_value::<T>().ok_or_else(|| {
            anyhow!(
                "{}() called on {}, expected {}",
                method_name,
                self.type_name(),
                expected_type
            )
        })
    }

    /// Helper for implementing no-argument RustValue methods
    pub fn with_rust_value_no_args<T, R, F>(
        &self,
        method_name: &str,
        expected_type: &str,
        mut args: crate::core::Args,
        f: F,
    ) -> Result<Value>
    where
        T: RustValue + 'static,
        R: Into<Value>,
        F: FnOnce(&T) -> Result<R>,
    {
        args.no_args(method_name)?;
        let rust_val = self.expect_rust_value::<T>(method_name, expected_type)?;
        let result = f(rust_val)?;
        Ok(result.into())
    }

    /// Helper for implementing no-argument mutating RustValue methods
    pub fn with_rust_value_no_args_mut<T, R, F>(
        &mut self,
        method_name: &str,
        expected_type: &str,
        mut args: crate::core::Args,
        f: F,
    ) -> Result<Value>
    where
        T: RustValue + 'static,
        R: Into<Value>,
        F: FnOnce(&mut T) -> Result<R>,
    {
        args.no_args(method_name)?;
        let type_name = self.type_name().to_string(); // Get type name before borrowing

        if let Value::RustValue(rust_val) = self {
            let downcasted = rust_val.as_any_mut().downcast_mut::<T>().ok_or_else(|| {
                anyhow!(
                    "{}() called on {}, expected {}",
                    method_name,
                    type_name,
                    expected_type
                )
            })?;
            let result = f(downcasted)?;
            Ok(result.into())
        } else {
            Err(anyhow!(
                "{}() called on {}, expected {}",
                method_name,
                type_name,
                expected_type
            ))
        }
    }

    /// Helper for implementing RustValue methods that take another RustValue of the same type
    pub fn with_rust_value_same_type<T, R, F>(
        &self,
        method_name: &str,
        expected_type: &str,
        arg_name: &str,
        mut args: crate::core::Args,
        f: F,
    ) -> Result<Value>
    where
        T: RustValue + 'static,
        R: Into<Value>,
        F: FnOnce(&T, &T) -> Result<R>,
    {
        args.set_function_name(method_name);
        let arg_value = args.expect(arg_name)?;
        args.complete()?;

        let rust_val = self.expect_rust_value::<T>(method_name, expected_type)?;
        let other_rust_val = arg_value.expect_rust_value::<T>(method_name, expected_type)?;

        let result = f(rust_val, other_rust_val)?;
        Ok(result.into())
    }

    /// Helper for implementing RustValue methods that take an f64 argument
    pub fn with_rust_value_f64_arg<T, R, F>(
        &self,
        method_name: &str,
        expected_type: &str,
        arg_name: &str,
        mut args: crate::core::Args,
        f: F,
    ) -> Result<Value>
    where
        T: RustValue + 'static,
        R: Into<Value>,
        F: FnOnce(&T, f64) -> Result<R>,
    {
        args.set_function_name(method_name);
        let arg_value = args.expect(arg_name)?;
        args.complete()?;

        let rust_val = self.expect_rust_value::<T>(method_name, expected_type)?;
        let f64_arg = arg_value.expect_f64(method_name, arg_name)?;

        let result = f(rust_val, f64_arg)?;
        Ok(result.into())
    }

    /// Get the type name for better error messages
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
            Value::Range(_) => "range",
            Value::Class(_) => "class",
            Value::ClassInstance(_) => "class_instance",
            Value::RustValue(rv) => rv.type_name().unwrap_or("rust_value"),
            Value::Raised(_) => "raised",
        }
    }

    /// Try to downcast a RustValue to a concrete type with mutable access
    pub fn downcast_rust_value_mut<T: RustValue + 'static>(&mut self) -> Option<&mut T> {
        if let Value::RustValue(rust_val) = self {
            rust_val.as_any_mut().downcast_mut::<T>()
        } else {
            None
        }
    }

    pub fn get_attr(&self, name: &str, eval: &mut crate::eval::Evaluator) -> Option<Value> {
        match self {
            Value::RustValue(rv) => rv.get_attr(name),
            // TODO: move to a get_class_attr
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

    pub fn set_attr(&mut self, name: &str, value: Value) -> Result<(), String> {
        match self {
            Value::RustValue(rv) => rv.set_attr(name, value),
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
            Value::RustValue(rv) => rv.call(args),
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

    pub fn op_iter(&self) -> Result<Value> {
        match self {
            Value::List(list) => Ok(Value::from_rust(crate::core::ListIter::new(list.clone()))),
            Value::Dict(dict) => Ok(Value::from_rust(crate::core::DictIter::new(dict.clone()))),
            Value::Range(range) => Ok(Value::from_rust(crate::core::RangeIter::new(range.clone()))),
            Value::RustValue(rv) => rv.op_iter(),
            _ => Err(anyhow!("Value is not iterable: {:?}", self)),
        }
    }

    pub fn op_next(&mut self) -> Result<Option<Value>> {
        match self {
            Value::RustValue(rv) => rv.op_next(),
            _ => Err(anyhow!("Value is not an iterator: {:?}", self)),
        }
    }

    // Operator method implementations for built-in types
    fn get_int_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtins::*;

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
        use crate::core::builtins::*;

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
        use crate::core::builtins::*;

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
        use crate::core::builtins::*;

        match name {
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            _ => None,
        }
    }

    fn get_list_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtins::*;

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
        use crate::core::builtins::*;

        match name {
            "op_contains" => Some(self.bind_method(op_contains)),
            "op_get_item" => Some(self.bind_method(op_get_item_dict)),
            "op_set_item" => Some(self.bind_method(op_set_item_dict)),
            "op_eq" => Some(self.bind_method(op_eq)),
            _ => None,
        }
    }

    fn get_range_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtins::*;

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
        use crate::core::builtins::*;

        match name {
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            _ => None,
        }
    }

    fn get_null_attr(&self, name: &str) -> Option<Value> {
        use crate::core::builtins::*;

        match name {
            "op_eq" => Some(self.bind_method(op_eq)),
            "op_ne" => Some(self.bind_method(op_ne)),
            _ => None,
        }
    }

    pub fn eval(
        &self,
        evaluator: &mut crate::eval::Evaluator,
    ) -> anyhow::Result<crate::eval::EvalResult> {
        match self {
            Value::RustValue(rust_val) => rust_val.eval(evaluator),
            _ => Err(anyhow!("Cannot evaluate this value type: {:?}", self)),
        }
    }
}

// Convenient Into<Value> implementations for common types
impl From<f64> for Value {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Int(i)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Unit
    }
}
