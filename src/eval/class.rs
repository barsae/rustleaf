use anyhow::Result;
use std::collections::HashMap;

use super::{
    core::{ClassMethod, Eval},
    scope::ScopeRef,
};
use crate::core::{Args, RustValue, Value};

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
        self.find_static_method(name).map(|method| {
            Value::from_rust(StaticMethod {
                class_name: self.name.clone(),
                method: method.clone(),
            })
        })
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

        Ok(Value::from_rust(ClassInstance {
            class_name: self.name.clone(),
            fields,
            methods: self.methods.clone(),
        }))
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

        // Method access is now handled by the evaluator in GetAttr
        // so we don't create BoundMethod here
        None
    }

    fn set_attr(&mut self, name: &str, value: Value) -> Result<(), String> {
        // Allow setting fields (for now, allow setting any field)
        self.fields.insert(name.to_string(), value);
        Ok(())
    }

    fn call(&self, _args: Args) -> Result<Value> {
        Err(anyhow::anyhow!(
            "'{}' object is not callable",
            self.class_name
        ))
    }

    fn is_class_instance(&self) -> bool {
        true
    }

    fn get_class_method(&self, name: &str) -> Option<super::core::ClassMethod> {
        self.find_method(name).cloned()
    }
}

/// A bound method - method bound to a specific instance
#[derive(Debug, Clone)]
pub struct BoundMethod {
    pub instance: Value, // The instance this method is bound to
    pub method: ClassMethod,
    pub closure_env: ScopeRef, // Environment where the method can execute
}

impl RustValue for BoundMethod {
    fn call(&self, mut args: Args) -> Result<Value> {
        use super::evaluator::{ControlFlow, ErrorKind, Evaluator};
        use anyhow::anyhow;

        // Check argument count - method should expect self + provided args
        let expected_args = self.method.params.len(); // This includes 'self' now
        let provided_args = args.len() + 1; // +1 for the instance we'll inject

        if provided_args != expected_args {
            return Err(anyhow!(
                "Method '{}' expects {} arguments, got {}",
                self.method.name,
                expected_args - 1,
                args.len()
            ));
        }

        // Create new scope for method execution
        let method_scope = self.closure_env.child();

        // Set function name for better error messages
        args.set_function_name(&format!("method {}", self.method.name));

        // Bind 'self' parameter to the bound instance
        method_scope.define("self".to_string(), self.instance.clone());

        // Bind remaining parameters to provided arguments using the fluent API
        for param_name in self.method.params.iter().skip(1) {
            let arg_value = args.expect(param_name)?;
            method_scope.define(param_name.clone(), arg_value);
        }

        // Validate all args consumed
        args.complete()?;

        // Create evaluator with method scope
        let mut evaluator = Evaluator {
            globals: self.closure_env.clone(),
            current_env: method_scope,
            current_dir: std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from(".")),
        };

        // Evaluate method body
        match evaluator.eval(&self.method.body) {
            Ok(value) => Ok(value),
            Err(ControlFlow::Return(value)) => Ok(value),
            Err(ControlFlow::Error(ErrorKind::SystemError(err))) => Err(err),
            Err(ControlFlow::Error(ErrorKind::RaisedError(value))) => {
                // Convert raised value to string for error display
                match value {
                    crate::core::Value::String(s) => Err(anyhow!("{}", s)),
                    _ => Err(anyhow!("{:?}", value)),
                }
            }
            Err(other) => Err(anyhow!("Invalid control flow in method: {:?}", other)),
        }
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
