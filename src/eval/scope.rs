/// Scope for variable scoping in RustLeaf
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::core::Value;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    /// Create a new global scope
    pub fn new() -> Self {
        Scope {
            vars: HashMap::new(),
            parent: None,
        }
    }
    
    /// Create a new scope with a parent
    pub fn with_parent(parent: Rc<RefCell<Scope>>) -> Self {
        Scope {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }
    
    /// Define a new variable in this scope
    pub fn define(&mut self, name: String, value: Value) {
        self.vars.insert(name, value);
    }
    
    /// Get a variable value, checking parent scopes if needed
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.vars.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }
    
    /// Set a variable value, checking parent scopes if needed
    pub fn set(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
            Ok(())
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().set(name, value)
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
    
    /// Check if a variable exists in this scope or any parent
    pub fn exists(&self, name: &str) -> bool {
        self.vars.contains_key(name) || 
            self.parent.as_ref().map_or(false, |p| p.borrow().exists(name))
    }
}