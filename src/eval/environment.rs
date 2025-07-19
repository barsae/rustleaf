use crate::parser::{AstNode, Visibility};
use crate::value::function::{get_builtin_functions, BuiltinFunctionInfo};
use crate::value::types::{ErrorType, Function, RuntimeError, Value};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Rc<RefCell<Value>>>>,
    visibility: Vec<HashMap<String, Visibility>>,
    builtins: HashMap<String, BuiltinFunctionInfo>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Environment {
            scopes: vec![HashMap::new()],     // Global scope
            visibility: vec![HashMap::new()], // Global visibility scope
            builtins: HashMap::new(),
        };

        // Add builtin functions
        for builtin in get_builtin_functions() {
            env.builtins.insert(builtin.name.to_string(), builtin);
        }

        // Add class objects to global scope
        env.add_builtin_classes();

        env
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.visibility.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
            self.visibility.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.define_with_visibility(name, value, Visibility::Private);
    }

    pub fn define_with_visibility(&mut self, name: String, value: Value, visibility: Visibility) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.clone(), Rc::new(RefCell::new(value)));
        }
        if let Some(vis_scope) = self.visibility.last_mut() {
            vis_scope.insert(name, visibility);
        }
    }

    /// Define a variable with an existing reference (for closures)
    pub fn define_ref(&mut self, name: String, value_ref: Rc<RefCell<Value>>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value_ref);
        }
    }

    /// Define a variable in the global (file) scope - used for imports
    pub fn define_global(&mut self, name: String, value: Value) {
        if let Some(global_scope) = self.scopes.first_mut() {
            global_scope.insert(name.clone(), Rc::new(RefCell::new(value)));
        }
        if let Some(global_vis_scope) = self.visibility.first_mut() {
            global_vis_scope.insert(name, Visibility::Private);
        }
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(value_ref) = scope.get(name) {
                return Ok(value_ref.borrow().clone());
            }
        }

        // Check builtins
        if let Some(_builtin) = self.builtins.get(name) {
            return Ok(Value::Function(Function {
                name: Some(name.to_string()),
                parameters: vec![], // Builtins handle their own parameters
                body: AstNode::Block {
                    statements: vec![],
                    location: Default::default(),
                },
                closure: None,
                is_builtin: true,
            }));
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{name}'"),
            ErrorType::NameError,
        ))
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), RuntimeError> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter_mut().rev() {
            if let Some(value_ref) = scope.get(name) {
                *value_ref.borrow_mut() = value;
                return Ok(());
            }
        }

        Err(RuntimeError::new(
            format!("Undefined variable '{name}'"),
            ErrorType::NameError,
        ))
    }

    pub fn get_builtin(&self, name: &str) -> Option<&BuiltinFunctionInfo> {
        self.builtins.get(name)
    }

    pub fn is_nested_scope(&self) -> bool {
        self.scopes.len() > 1
    }

    pub fn capture_closure(&self) -> HashMap<String, Rc<RefCell<Value>>> {
        let mut closure = HashMap::new();

        // Capture all variables from all scopes (except global)
        for scope in &self.scopes[..self.scopes.len()] {
            for (name, value_ref) in scope {
                // Only capture if not already captured (inner scopes override outer)
                if !closure.contains_key(name) {
                    closure.insert(name.clone(), value_ref.clone());
                }
            }
        }

        closure
    }

    /// Get all public bindings from the current (innermost) scope
    pub fn get_public_bindings(&self) -> HashMap<String, Value> {
        let mut public_bindings = HashMap::new();

        if let (Some(current_scope), Some(current_visibility)) =
            (self.scopes.last(), self.visibility.last())
        {
            for (name, value_ref) in current_scope {
                if let Some(Visibility::Public) = current_visibility.get(name) {
                    public_bindings.insert(name.clone(), value_ref.borrow().clone());
                }
            }
        }

        public_bindings
    }

    /// Get all bindings from the current (innermost) scope with their visibility
    pub fn get_all_bindings_with_visibility(&self) -> HashMap<String, (Value, Visibility)> {
        let mut all_bindings = HashMap::new();

        if let (Some(current_scope), Some(current_visibility)) =
            (self.scopes.last(), self.visibility.last())
        {
            for (name, value_ref) in current_scope {
                let visibility = current_visibility
                    .get(name)
                    .cloned()
                    .unwrap_or(Visibility::Private);
                all_bindings.insert(name.clone(), (value_ref.borrow().clone(), visibility));
            }
        }

        all_bindings
    }

    /// Add builtin class objects to global scope
    fn add_builtin_classes(&mut self) {
        use crate::lexer::{LiteralValue, SourceLocation};
        use crate::parser::AstNode;
        use crate::value::types::{Function, Object, Value};

        // Create len method for all classes
        let len_method = Value::UnboundMethod(Function {
            name: Some("len".to_string()),
            parameters: vec![],
            body: AstNode::Literal(LiteralValue::Null, SourceLocation::new(0, 0, 0)),
            closure: None,
            is_builtin: true,
        });

        // Create filter method for collections
        let filter_method = Value::UnboundMethod(Function {
            name: Some("filter".to_string()),
            parameters: vec![],
            body: AstNode::Literal(LiteralValue::Null, SourceLocation::new(0, 0, 0)),
            closure: None,
            is_builtin: true,
        });

        // Create sum method for collections
        let sum_method = Value::UnboundMethod(Function {
            name: Some("sum".to_string()),
            parameters: vec![],
            body: AstNode::Literal(LiteralValue::Null, SourceLocation::new(0, 0, 0)),
            closure: None,
            is_builtin: true,
        });

        // String class
        let mut string_fields = HashMap::new();
        string_fields.insert("len".to_string(), len_method.clone());
        let string_class = Value::Object(Object {
            class_name: "Class".to_string(),
            fields: string_fields,
            methods: HashMap::new(),
        });
        self.define("String".to_string(), string_class);

        // List class
        let mut list_fields = HashMap::new();
        list_fields.insert("len".to_string(), len_method.clone());
        list_fields.insert("filter".to_string(), filter_method.clone());
        list_fields.insert("sum".to_string(), sum_method.clone());
        let list_class = Value::Object(Object {
            class_name: "Class".to_string(),
            fields: list_fields,
            methods: HashMap::new(),
        });
        self.define("List".to_string(), list_class);

        // Dict class
        let mut dict_fields = HashMap::new();
        dict_fields.insert("len".to_string(), len_method.clone());
        let dict_class = Value::Object(Object {
            class_name: "Class".to_string(),
            fields: dict_fields,
            methods: HashMap::new(),
        });
        self.define("Dict".to_string(), dict_class);

        // Range class
        let mut range_fields = HashMap::new();
        range_fields.insert("len".to_string(), len_method);
        range_fields.insert("filter".to_string(), filter_method);
        let range_class = Value::Object(Object {
            class_name: "Class".to_string(),
            fields: range_fields,
            methods: HashMap::new(),
        });
        self.define("Range".to_string(), range_class);
    }
}
