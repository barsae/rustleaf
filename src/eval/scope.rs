use crate::value::types::{ErrorType, RuntimeError, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    variables: HashMap<String, Rc<RefCell<Value>>>,
    parent: Option<Rc<Scope>>,
}

impl Scope {
    /// Create a new root scope
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            parent: None,
        }
    }

    /// Create a new scope with a parent
    pub fn with_parent(parent: Rc<Scope>) -> Self {
        Self {
            variables: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Define a new variable with a value
    pub fn define(&mut self, name: String, value: Value) {
        self.variables.insert(name, Rc::new(RefCell::new(value)));
    }

    /// Define a variable with an existing reference (for closures)
    pub fn define_ref(&mut self, name: String, value_ref: Rc<RefCell<Value>>) {
        self.variables.insert(name, value_ref);
    }

    /// Get a variable's value (copying from RefCell)
    pub fn get(&self, name: &str) -> Option<Value> {
        // First check local scope
        if let Some(value_ref) = self.variables.get(name) {
            return Some(value_ref.borrow().clone());
        }

        // Then check parent scopes
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }

        None
    }

    /// Get a mutable reference to a variable for modification
    pub fn get_mut(&self, name: &str) -> Option<Rc<RefCell<Value>>> {
        // First check local scope
        if let Some(value_ref) = self.variables.get(name) {
            return Some(value_ref.clone());
        }

        // Then check parent scopes
        if let Some(parent) = &self.parent {
            return parent.get_mut(name);
        }

        None
    }

    /// Set a variable's value (fails if variable doesn't exist)
    pub fn set(&self, name: &str, value: Value) -> Result<(), RuntimeError> {
        if let Some(value_ref) = self.get_mut(name) {
            *value_ref.borrow_mut() = value;
            Ok(())
        } else {
            Err(RuntimeError::new(
                format!("Undefined variable '{}'", name),
                ErrorType::NameError,
            ))
        }
    }

    /// Check if a variable exists in this scope or parent scopes
    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name) || self.parent.as_ref().is_some_and(|p| p.contains(name))
    }

    /// Check if a variable exists in this scope only (not parent scopes)
    pub fn contains_local(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    /// Capture all variables from this scope and parent scopes for closure creation
    pub fn capture_closure(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut captured = HashMap::new();

        // Walk up the scope chain
        let mut current = Some(self);
        while let Some(scope) = current {
            // Add variables from current scope (inner scopes override outer scopes)
            for (name, value_ref) in &scope.variables {
                if !captured.contains_key(name) {
                    captured.insert(name.clone(), value_ref.clone());
                }
            }

            // Move to parent scope
            current = scope.parent.as_deref();
        }

        captured
    }

    /// Get all variables from the local scope only (for module exports)
    pub fn get_local_variables(&self) -> HashMap<String, Value> {
        self.variables
            .iter()
            .map(|(name, value_ref)| (name.clone(), value_ref.borrow().clone()))
            .collect()
    }

    /// Create a scope from captured closure variables
    pub fn from_closure(closure_vars: HashMap<String, Rc<RefCell<Value>>>) -> Self {
        Self {
            variables: closure_vars,
            parent: None,
        }
    }

    /// Get the parent scope
    pub fn parent(&self) -> Option<&Rc<Scope>> {
        self.parent.as_ref()
    }

    /// Check if this is a root scope (no parent)
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}
