/// Scope for variable scoping in RustLeaf
use std::cell::RefCell;
use std::rc::Rc;

use crate::core::Value;
use indexmap::IndexMap;

#[derive(Clone, Debug, Default)]
pub struct ScopeRef(Rc<RefCell<Scope>>);

#[derive(Debug, Clone)]
pub struct Scope {
    vars: IndexMap<String, Value>,
    parent: Option<ScopeRef>,
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Scope {
    /// Create a new global scope
    pub fn new() -> Self {
        Scope {
            vars: IndexMap::new(),
            parent: None,
        }
    }

    /// Create a new scope with a parent
    pub fn with_parent(parent: ScopeRef) -> Self {
        Scope {
            vars: IndexMap::new(),
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
            parent.0.borrow().get(name)
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
            parent.0.borrow_mut().set(name, value)
        } else {
            Err(format!("Undefined variable: {}", name))
        }
    }
}

impl ScopeRef {
    /// Create a new global scope
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Scope::new())))
    }

    /// Create a new scope with this scope as parent
    pub fn child(&self) -> Self {
        Self(Rc::new(RefCell::new(Scope::with_parent(self.clone()))))
    }

    /// Define a new variable in this scope
    pub fn define(&self, name: String, value: Value) {
        self.0.borrow_mut().define(name, value);
    }

    /// Get a variable value, checking parent scopes if needed
    pub fn get(&self, name: &str) -> Option<Value> {
        self.0.borrow().get(name)
    }

    /// Set a variable value, checking parent scopes if needed
    pub fn set(&self, name: &str, value: Value) -> Result<(), String> {
        self.0.borrow_mut().set(name, value)
    }
}
