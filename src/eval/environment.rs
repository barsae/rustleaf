use std::collections::HashMap;
use crate::value::value::{Value, RuntimeError, ErrorType, Function};
use crate::value::function::{get_builtin_functions, BuiltinFunctionInfo};
use crate::parser::AstNode;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
    builtins: HashMap<String, BuiltinFunctionInfo>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            scopes: vec![HashMap::new()], // Global scope
            builtins: HashMap::new(),
        };
        
        // Add builtin functions
        for builtin in get_builtin_functions() {
            env.builtins.insert(builtin.name.to_string(), builtin);
        }
        
        env
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Ok(value.clone());
            }
        }

        // Check builtins
        if let Some(_builtin) = self.builtins.get(name) {
            return Ok(Value::Function(Function {
                name: Some(name.to_string()),
                parameters: vec![], // Builtins handle their own parameters
                body: AstNode::Block { statements: vec![], location: Default::default() },
                closure: None,
                is_builtin: true,
            }));
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'", name),
            ErrorType::NameError,
        ))
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{}'", name),
            ErrorType::NameError,
        ))
    }

    pub fn get_builtin(&self, name: &str) -> Option<&BuiltinFunctionInfo> {
        self.builtins.get(name)
    }
}

impl Default for crate::parser::SourceLocation {
    fn default() -> Self {
        crate::parser::SourceLocation {
            line: 0,
            column: 0,
            byte_offset: 0,
        }
    }
}