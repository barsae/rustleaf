use std::collections::HashMap;
use anyhow::Result;

use crate::core::{Value, RustValue, Args};
use super::core::{Eval, ClassMethod};

/// A class definition - the "type" that can be called as a constructor
#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub field_names: Vec<String>,
    pub field_defaults: Vec<Option<Eval>>,
    pub methods: Vec<ClassMethod>,
}

impl Class {
    pub fn new(
        name: String,
        field_names: Vec<String>,
        field_defaults: Vec<Option<Eval>>,
        methods: Vec<ClassMethod>,
    ) -> Self {
        Self {
            name,
            field_names,
            field_defaults,
            methods,
        }
    }

    /// Find a method by name
    pub fn find_method(&self, name: &str) -> Option<&ClassMethod> {
        self.methods.iter().find(|m| m.name == name)
    }

    /// Find a static method by name
    pub fn find_static_method(&self, name: &str) -> Option<&ClassMethod> {
        self.methods.iter().find(|m| m.name == name && m.is_static)
    }
}

impl RustValue for Class {
    fn get_attr(&self, name: &str) -> Option<Value> {
        // Allow access to static methods
        self.find_static_method(name).map(|method| Value::from_rust(
            StaticMethod {
                class_name: self.name.clone(),
                method: method.clone(),
            }
        ))
    }

    fn set_attr(&mut self, _name: &str, _value: Value) -> Result<(), String> {
        Err("Cannot set attributes on class".to_string())
    }

    fn call(&self, args: Args) -> Result<Value> {
        // Constructor call - create new instance
        if !args.is_empty() {
            return Err(anyhow::anyhow!("Class constructor takes no arguments"));
        }

        // Create new instance with default field values
        let mut fields = HashMap::new();
        for (i, field_name) in self.field_names.iter().enumerate() {
            let value = if let Some(_default) = &self.field_defaults[i] {
                // For now, we'll need to evaluate defaults when creating the instance
                // This is a simplification - we'd need access to the evaluator
                Value::Null // TODO: evaluate default expressions
            } else {
                Value::Null
            };
            fields.insert(field_name.clone(), value);
        }

        Ok(Value::from_rust(
            ClassInstance {
                class_name: self.name.clone(),
                fields,
                methods: self.methods.clone(),
            }
        ))
    }
}

/// A class instance - holds field values and provides method access
#[derive(Debug)]
pub struct ClassInstance {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub methods: Vec<ClassMethod>,
}

impl ClassInstance {
    /// Find a method by name
    pub fn find_method(&self, name: &str) -> Option<&ClassMethod> {
        self.methods.iter().find(|m| m.name == name && !m.is_static)
    }
}

impl RustValue for ClassInstance {
    fn get_attr(&self, name: &str) -> Option<Value> {
        // First check for fields
        if let Some(field_value) = self.fields.get(name) {
            return Some(field_value.clone());
        }

        // Then check for methods
        self.find_method(name).map(|method| Value::from_rust(
            BoundMethod {
                instance_class: self.class_name.clone(),
                method: method.clone(),
            }
        ))
    }

    fn set_attr(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Allow setting fields (for now, allow setting any field)
        self.fields.insert(name.to_string(), value);
        Ok(())
    }

    fn call(&self, _args: Args) -> Result<Value> {
        Err(anyhow::anyhow!("'{}' object is not callable", self.class_name))
    }
}

/// A bound method - method bound to a specific instance
#[derive(Debug)]
pub struct BoundMethod {
    pub instance_class: String,
    pub method: ClassMethod,
}

impl RustValue for BoundMethod {
    fn call(&self, _args: Args) -> Result<Value> {
        // This will need to be implemented by the evaluator
        // as it needs to execute the method body with 'self' bound
        Err(anyhow::anyhow!("Bound method call not yet implemented"))
    }
}

/// A static method bound to a class
#[derive(Debug)]
pub struct StaticMethod {
    pub class_name: String,
    pub method: ClassMethod,
}

impl RustValue for StaticMethod {
    fn call(&self, _args: Args) -> Result<Value> {
        // This will need to be implemented by the evaluator
        // as it needs to execute the method body
        Err(anyhow::anyhow!("Static method call not yet implemented"))
    }
}