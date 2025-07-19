use super::core::Evaluator;
use crate::lexer::SourceLocation;
use crate::parser::{AssignmentOperator, AstNode, ImportClause, ModulePath, Parameter, Visibility};
use crate::value::types::{ErrorType, Function, RuntimeError, RustValue, Value};
use std::collections::HashMap;

// Class implementation
#[derive(Debug, Clone)]
pub struct Class {
    pub name: Option<String>,
    pub members: Vec<crate::parser::ClassMember>,
}

// Class instance implementation
#[derive(Debug, Clone)]
pub struct ClassInstance {
    pub class_name: String,
    pub fields: HashMap<String, Value>,
    pub class_members: Vec<crate::parser::ClassMember>,
}

impl RustValue for Class {
    fn type_name(&self) -> &'static str {
        "class"
    }

    fn to_string(&self) -> String {
        if let Some(name) = &self.name {
            format!("<class '{}'>", name)
        } else {
            format!("<class with {} members>", self.members.len())
        }
    }

    fn clone_box(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl RustValue for ClassInstance {
    fn type_name(&self) -> &'static str {
        // Return the class name as the type name, but for now use a generic type
        "object"
    }

    fn to_string(&self) -> String {
        format!("<{} object>", self.class_name)
    }

    fn clone_box(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Class {
    /// Create a new instance of this class
    pub fn instantiate(&self, evaluator: &mut Evaluator) -> Result<ClassInstance, RuntimeError> {
        let mut fields = HashMap::new();

        // Initialize fields with their default values
        for member in &self.members {
            if let crate::parser::ClassMember::Field { name, value, .. } = member {
                let field_value = if let Some(default_expr) = value {
                    evaluator.evaluate(default_expr)?
                } else {
                    Value::Null
                };
                fields.insert(name.clone(), field_value);
            }
        }

        Ok(ClassInstance {
            class_name: self
                .name
                .clone()
                .unwrap_or_else(|| "AnonymousClass".to_string()),
            fields,
            class_members: self.members.clone(),
        })
    }

    /// Get a static method by name
    pub fn get_static_method(&self, method_name: &str) -> Option<&AstNode> {
        for member in &self.members {
            if let crate::parser::ClassMember::Method {
                is_static: true,
                declaration,
                ..
            } = member
            {
                if let AstNode::FunctionDeclaration { name, .. } = declaration {
                    if name == method_name {
                        return Some(declaration);
                    }
                }
            }
        }
        None
    }
}

impl ClassInstance {
    /// Get a field value by name
    pub fn get_field(&self, field_name: &str) -> Option<&Value> {
        self.fields.get(field_name)
    }

    /// Set a field value by name
    pub fn set_field(&mut self, field_name: &str, value: Value) -> Result<(), RuntimeError> {
        // Check if field exists in class definition
        let field_exists = self.class_members.iter().any(|member| {
            matches!(member, crate::parser::ClassMember::Field { name, .. } if name == field_name)
        });

        if field_exists {
            self.fields.insert(field_name.to_string(), value);
            Ok(())
        } else {
            Err(RuntimeError::new(
                format!("'{}' object has no field '{}'", self.class_name, field_name),
                ErrorType::AttributeError,
            ))
        }
    }

    /// Get an instance method by name
    pub fn get_instance_method(&self, method_name: &str) -> Option<&AstNode> {
        for member in &self.class_members {
            if let crate::parser::ClassMember::Method {
                is_static: false,
                declaration,
                ..
            } = member
            {
                if let AstNode::FunctionDeclaration { name, .. } = declaration {
                    if name == method_name {
                        return Some(declaration);
                    }
                }
            }
        }
        None
    }
}

impl Evaluator {
    pub(crate) fn evaluate_block(&mut self, statements: &[AstNode]) -> Result<Value, RuntimeError> {
        self.environment.push_scope();

        let mut result = Value::Unit;
        for (i, stmt) in statements.iter().enumerate() {
            if i == statements.len() - 1 {
                // Last item determines block value based on its type
                match stmt {
                    // If the last item is an expression statement, the block evaluates to unit
                    AstNode::ExpressionStatement { .. } => {
                        self.evaluate(stmt)?; // Execute the statement
                        result = Value::Unit; // But return unit
                    }
                    // If the last item is any other statement, the block evaluates to unit
                    AstNode::VariableDeclaration { .. }
                    | AstNode::Assignment { .. }
                    | AstNode::FunctionDeclaration { .. }
                    | AstNode::ClassDeclaration { .. }
                    | AstNode::ImportStatement { .. }
                    | AstNode::WhileStatement { .. }
                    | AstNode::ForStatement { .. }
                    | AstNode::MatchStatement { .. }
                    | AstNode::TryStatement { .. }
                    | AstNode::WithStatement { .. }
                    | AstNode::ReturnStatement { .. }
                    | AstNode::BreakStatement { .. }
                    | AstNode::ContinueStatement { .. } => {
                        self.evaluate(stmt)?; // Execute the statement
                        result = Value::Unit; // But return unit
                    }
                    // If the last item is a pure expression, it becomes the block value
                    _ => {
                        result = self.evaluate(stmt)?;
                    }
                }
            } else {
                self.evaluate(stmt)?;
            }
        }

        self.environment.pop_scope();
        Ok(result)
    }

    pub(crate) fn evaluate_variable_declaration(
        &mut self,
        visibility: &Visibility,
        name: &str,
        value: &Option<Box<AstNode>>,
    ) -> Result<Value, RuntimeError> {
        let val = if let Some(value_node) = value {
            self.evaluate(value_node)?
        } else {
            Value::Null
        };

        // Store the variable with its visibility for module exports
        self.environment
            .define_with_visibility(name.to_string(), val, visibility.clone());
        Ok(Value::Unit)
    }

    pub(crate) fn evaluate_import_statement(
        &mut self,
        path: &ModulePath,
        clause: &Option<ImportClause>,
        location: SourceLocation,
    ) -> Result<Value, RuntimeError> {
        // Handle different import clauses
        match clause {
            None => {
                // Import the module as a namespace: use module;
                let module_path = self.load_module(path, location)?;
                let module_name = module_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let module_env = self.get_module(&module_path).unwrap();
                let public_bindings = module_env.get_public_bindings();
                let module_obj = Value::Dict(public_bindings);
                self.environment
                    .define_global(module_name.to_string(), module_obj);
            }
            Some(ImportClause::All) => {
                // Import all public items: use module::*;
                let module_path = self.load_module(path, location)?;
                let module_env = self.get_module(&module_path).unwrap();
                let public_bindings = module_env.get_public_bindings();

                for (name, value) in public_bindings {
                    self.environment.define_global(name, value);
                }
            }
            Some(ImportClause::Single(item_name)) => {
                // First try to import as a submodule by extending the path
                let mut extended_path = path.clone();
                extended_path.segments.push(item_name.clone());

                // Try to load the extended path as a module
                if let Ok(submodule_path) = self.load_module(&extended_path, location.clone()) {
                    // Success! Import the submodule as a namespace
                    let submodule_env = self.get_module(&submodule_path).unwrap();
                    let submodule_bindings = submodule_env.get_public_bindings();
                    let module_obj = Value::Dict(submodule_bindings);
                    self.environment
                        .define_global(item_name.clone(), module_obj);
                } else {
                    // Fall back to importing as an item from the original module
                    let module_path = self.load_module(path, location)?;
                    let module_name = module_path
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown");

                    let module_env = self.get_module(&module_path).unwrap();
                    let public_bindings = module_env.get_public_bindings();

                    if let Some(value) = public_bindings.get(item_name) {
                        self.environment
                            .define_global(item_name.clone(), value.clone());
                    } else {
                        return Err(RuntimeError::new(
                            format!("Item '{}' not found in module '{}'", item_name, module_name),
                            ErrorType::ImportError,
                        ));
                    }
                }
            }
            Some(ImportClause::Named(items)) => {
                // Import specific named items: use module::{item1, item2};
                let module_path = self.load_module(path, location)?;
                let module_name = module_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown");

                let module_env = self.get_module(&module_path).unwrap();
                let public_bindings = module_env.get_public_bindings();

                for item in items {
                    if let Some(value) = public_bindings.get(&item.name) {
                        let import_name = item.alias.as_ref().unwrap_or(&item.name);
                        self.environment
                            .define_global(import_name.clone(), value.clone());
                    } else {
                        return Err(RuntimeError::new(
                            format!("Item '{}' not found in module '{}'", item.name, module_name),
                            ErrorType::ImportError,
                        ));
                    }
                }
            }
        }

        Ok(Value::Unit)
    }

    pub(crate) fn evaluate_assignment(
        &mut self,
        target: &AstNode,
        operator: &AssignmentOperator,
        value: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let new_value = self.evaluate(value)?;

        match target {
            AstNode::Identifier(name, _) => {
                let final_value = match operator {
                    AssignmentOperator::Assign => new_value,
                    _ => {
                        let current = self.environment.get(name)?;
                        match operator {
                            AssignmentOperator::AddAssign => {
                                self.add_values(&current, &new_value)?
                            }
                            AssignmentOperator::SubtractAssign => {
                                self.subtract_values(&current, &new_value)?
                            }
                            AssignmentOperator::MultiplyAssign => {
                                self.multiply_values(&current, &new_value)?
                            }
                            AssignmentOperator::DivideAssign => {
                                self.divide_values(&current, &new_value)?
                            }
                            AssignmentOperator::ModuloAssign => {
                                self.modulo_values(&current, &new_value)?
                            }
                            _ => unreachable!(),
                        }
                    }
                };

                self.environment.set(name, final_value.clone())?;
                Ok(final_value)
            }
            AstNode::PropertyAccess {
                object, property, ..
            } => {
                // For property assignment, we need to handle mutation correctly
                // We need to evaluate and potentially update the object in place

                match operator {
                    AssignmentOperator::Assign => {
                        // For simple assignment, handle different object types
                        match object.as_ref() {
                            AstNode::Identifier(var_name, _) => {
                                // Get the object from the environment
                                let mut target_object = self.environment.get(var_name)?;

                                match target_object {
                                    Value::Dict(ref mut dict) => {
                                        dict.insert(property.clone(), new_value.clone());
                                        self.environment.set(var_name, target_object)?;
                                        Ok(new_value)
                                    }
                                    Value::RustValue(ref mut rust_value) => {
                                        rust_value.op_set(property, new_value.clone())?;
                                        self.environment.set(var_name, target_object)?;
                                        Ok(new_value)
                                    }
                                    _ => Err(RuntimeError::new(
                                        format!(
                                            "'{}' object does not support property assignment",
                                            target_object.type_name()
                                        ),
                                        ErrorType::TypeError,
                                    )),
                                }
                            }
                            _ => {
                                // For complex expressions like nested access, we need a different approach
                                Err(RuntimeError::new(
                                    "Complex nested property assignment not yet supported"
                                        .to_string(),
                                    ErrorType::RuntimeError,
                                ))
                            }
                        }
                    }
                    _ => {
                        // For compound assignment
                        match object.as_ref() {
                            AstNode::Identifier(var_name, _) => {
                                let mut target_object = self.environment.get(var_name)?;

                                let current_value = match &target_object {
                                    Value::Dict(dict) => {
                                        dict.get(property).cloned().unwrap_or(Value::Null)
                                    }
                                    Value::RustValue(_) => {
                                        return Err(RuntimeError::new(
                                            "Compound assignment on RustValue properties not yet supported".to_string(),
                                            ErrorType::RuntimeError,
                                        ));
                                    }
                                    _ => {
                                        return Err(RuntimeError::new(
                                            format!(
                                                "'{}' object does not support property assignment",
                                                target_object.type_name()
                                            ),
                                            ErrorType::TypeError,
                                        ))
                                    }
                                };

                                let final_value = match operator {
                                    AssignmentOperator::AddAssign => {
                                        self.add_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::SubtractAssign => {
                                        self.subtract_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::MultiplyAssign => {
                                        self.multiply_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::DivideAssign => {
                                        self.divide_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::ModuloAssign => {
                                        self.modulo_values(&current_value, &new_value)?
                                    }
                                    _ => unreachable!(),
                                };

                                match target_object {
                                    Value::Dict(ref mut dict) => {
                                        dict.insert(property.clone(), final_value.clone());
                                        self.environment.set(var_name, target_object)?;
                                        Ok(final_value)
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            _ => Err(RuntimeError::new(
                                "Complex nested compound property assignment not yet supported"
                                    .to_string(),
                                ErrorType::RuntimeError,
                            )),
                        }
                    }
                }
            }
            AstNode::IndexAccess { object, index, .. } => {
                // For index assignment, handle mutation correctly
                let index_value = self.evaluate(index)?;

                match operator {
                    AssignmentOperator::Assign => match object.as_ref() {
                        AstNode::Identifier(var_name, _) => {
                            let mut target_object = self.environment.get(var_name)?;

                            match target_object {
                                Value::List(ref mut list) => {
                                    if let Value::Int(idx) = index_value {
                                        let list_len = list.len() as i64;
                                        let normalized_idx =
                                            if idx < 0 { list_len + idx } else { idx };

                                        if normalized_idx >= 0
                                            && (normalized_idx as usize) < list.len()
                                        {
                                            list[normalized_idx as usize] = new_value.clone();
                                            self.environment.set(var_name, target_object)?;
                                            Ok(new_value)
                                        } else {
                                            Err(RuntimeError::new(
                                                "list index out of range".to_string(),
                                                ErrorType::IndexError,
                                            ))
                                        }
                                    } else {
                                        Err(RuntimeError::new(
                                            "list indices must be integers".to_string(),
                                            ErrorType::TypeError,
                                        ))
                                    }
                                }
                                Value::Dict(ref mut dict) => {
                                    if let Value::String(key) = index_value {
                                        dict.insert(key, new_value.clone());
                                        self.environment.set(var_name, target_object)?;
                                        Ok(new_value)
                                    } else {
                                        Err(RuntimeError::new(
                                            "dict keys must be strings".to_string(),
                                            ErrorType::TypeError,
                                        ))
                                    }
                                }
                                Value::RustValue(ref mut rust_value) => {
                                    rust_value.op_index_set(index_value, new_value.clone())?;
                                    self.environment.set(var_name, target_object)?;
                                    Ok(new_value)
                                }
                                _ => Err(RuntimeError::new(
                                    format!(
                                        "'{}' object does not support index assignment",
                                        target_object.type_name()
                                    ),
                                    ErrorType::TypeError,
                                )),
                            }
                        }
                        _ => Err(RuntimeError::new(
                            "Complex nested index assignment not yet supported".to_string(),
                            ErrorType::RuntimeError,
                        )),
                    },
                    _ => {
                        // For compound assignment
                        match object.as_ref() {
                            AstNode::Identifier(var_name, _) => {
                                let mut target_object = self.environment.get(var_name)?;

                                let current_value = match &target_object {
                                    Value::List(list) => {
                                        if let Value::Int(idx) = index_value {
                                            let list_len = list.len() as i64;
                                            let normalized_idx =
                                                if idx < 0 { list_len + idx } else { idx };

                                            if normalized_idx >= 0
                                                && (normalized_idx as usize) < list.len()
                                            {
                                                list[normalized_idx as usize].clone()
                                            } else {
                                                return Err(RuntimeError::new(
                                                    "list index out of range".to_string(),
                                                    ErrorType::IndexError,
                                                ));
                                            }
                                        } else {
                                            return Err(RuntimeError::new(
                                                "list indices must be integers".to_string(),
                                                ErrorType::TypeError,
                                            ));
                                        }
                                    }
                                    Value::Dict(dict) => {
                                        if let Value::String(key) = &index_value {
                                            dict.get(key).cloned().unwrap_or(Value::Null)
                                        } else {
                                            return Err(RuntimeError::new(
                                                "dict keys must be strings".to_string(),
                                                ErrorType::TypeError,
                                            ));
                                        }
                                    }
                                    Value::RustValue(_) => {
                                        return Err(RuntimeError::new(
                                            "Compound assignment on RustValue indices not yet supported".to_string(),
                                            ErrorType::RuntimeError,
                                        ));
                                    }
                                    _ => {
                                        return Err(RuntimeError::new(
                                            format!(
                                                "'{}' object does not support index assignment",
                                                target_object.type_name()
                                            ),
                                            ErrorType::TypeError,
                                        ))
                                    }
                                };

                                let final_value = match operator {
                                    AssignmentOperator::AddAssign => {
                                        self.add_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::SubtractAssign => {
                                        self.subtract_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::MultiplyAssign => {
                                        self.multiply_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::DivideAssign => {
                                        self.divide_values(&current_value, &new_value)?
                                    }
                                    AssignmentOperator::ModuloAssign => {
                                        self.modulo_values(&current_value, &new_value)?
                                    }
                                    _ => unreachable!(),
                                };

                                match target_object {
                                    Value::List(ref mut list) => {
                                        if let Value::Int(idx) = index_value {
                                            let list_len = list.len() as i64;
                                            let normalized_idx =
                                                if idx < 0 { list_len + idx } else { idx };
                                            list[normalized_idx as usize] = final_value.clone();
                                            self.environment.set(var_name, target_object)?;
                                            Ok(final_value)
                                        } else {
                                            unreachable!() // Already handled above
                                        }
                                    }
                                    Value::Dict(ref mut dict) => {
                                        if let Value::String(key) = index_value {
                                            dict.insert(key, final_value.clone());
                                            self.environment.set(var_name, target_object)?;
                                            Ok(final_value)
                                        } else {
                                            unreachable!() // Already handled above
                                        }
                                    }
                                    _ => unreachable!(), // Already handled above
                                }
                            }
                            _ => Err(RuntimeError::new(
                                "Complex nested compound index assignment not yet supported"
                                    .to_string(),
                                ErrorType::RuntimeError,
                            )),
                        }
                    }
                }
            }
            _ => Err(RuntimeError::new(
                format!("Invalid assignment target: {:?}", target),
                ErrorType::RuntimeError,
            )),
        }
    }

    pub(crate) fn evaluate_program(&mut self, items: &[AstNode]) -> Result<Value, RuntimeError> {
        let mut result = Value::Unit;
        for item in items {
            result = self.evaluate(item)?;
        }
        Ok(result)
    }

    pub(crate) fn evaluate_function_declaration(
        &mut self,
        visibility: &Visibility,
        name: &str,
        parameters: &[Parameter],
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        // Capture current environment for closures (if not at global scope)
        let closure = if self.environment.is_nested_scope() {
            let closure_vars = self.environment.capture_closure();
            Some(crate::eval::scope::Scope::from_closure(closure_vars))
        } else {
            None
        };

        let function = Function {
            name: Some(name.to_string()),
            parameters: parameters.to_vec(),
            body: body.clone(),
            closure,
            is_builtin: false,
        };

        let value = Value::Function(function);
        self.environment
            .define_with_visibility(name.to_string(), value, visibility.clone());
        Ok(Value::Unit)
    }

    pub(crate) fn evaluate_anonymous_function(
        &mut self,
        parameters: &[Parameter],
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        // Capture current environment for closures
        let closure = if self.environment.is_nested_scope() {
            let closure_vars = self.environment.capture_closure();
            Some(crate::eval::scope::Scope::from_closure(closure_vars))
        } else {
            None
        };

        let function = Function {
            name: None,
            parameters: parameters.to_vec(),
            body: body.clone(),
            closure,
            is_builtin: false,
        };

        Ok(Value::Function(function))
    }

    pub(crate) fn evaluate_return_statement(
        &mut self,
        value: &Option<Box<AstNode>>,
    ) -> Result<Value, RuntimeError> {
        if let Some(value_node) = value {
            let result = self.evaluate(value_node)?;
            // Use a special error type to signal return
            Err(
                RuntimeError::new("RETURN_VALUE".to_string(), ErrorType::RuntimeError)
                    .with_return_value(result),
            )
        } else {
            Err(
                RuntimeError::new("RETURN_UNIT".to_string(), ErrorType::RuntimeError)
                    .with_return_value(Value::Unit),
            )
        }
    }

    pub(crate) fn evaluate_for_statement(
        &mut self,
        variable: &str,
        index_variable: &Option<String>,
        iterable: &AstNode,
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        let iterable_value = self.evaluate(iterable)?;

        // Create new scope for loop
        self.environment.push_scope();

        let mut result = Value::Unit;

        match iterable_value {
            Value::List(list) => {
                for (index, item) in list.iter().enumerate() {
                    // Bind loop variable
                    self.environment.define(variable.to_string(), item.clone());

                    // Bind index variable if present
                    if let Some(index_var) = index_variable {
                        self.environment
                            .define(index_var.clone(), Value::Int(index as i64));
                    }

                    // Execute body
                    match self.evaluate(body) {
                        Ok(value) => result = value,
                        Err(err) => {
                            if err.is_return() {
                                // Return from function, not just loop
                                self.environment.pop_scope();
                                return Err(err);
                            }
                            // For now, ignore other errors (break/continue would go here)
                        }
                    }
                }
            }
            Value::String(s) => {
                for (index, ch) in s.chars().enumerate() {
                    // Bind loop variable
                    self.environment
                        .define(variable.to_string(), Value::String(ch.to_string()));

                    // Bind index variable if present
                    if let Some(index_var) = index_variable {
                        self.environment
                            .define(index_var.clone(), Value::Int(index as i64));
                    }

                    // Execute body
                    match self.evaluate(body) {
                        Ok(value) => result = value,
                        Err(err) => {
                            if err.is_return() {
                                // Return from function, not just loop
                                self.environment.pop_scope();
                                return Err(err);
                            }
                            // For now, ignore other errors (break/continue would go here)
                        }
                    }
                }
            }
            Value::RustValue(ref rust_value) => {
                // Clone the RustValue to check if it's iterable and for iteration
                let mut iterable = rust_value.clone();
                if iterable.is_iterable() {
                    let mut index = 0i64;
                    while let Some(item) = iterable.iter_next() {
                        // Bind loop variable
                        self.environment.define(variable.to_string(), item);

                        // Bind index variable if present
                        if let Some(index_var) = index_variable {
                            self.environment
                                .define(index_var.clone(), Value::Int(index));
                        }

                        // Execute body
                        match self.evaluate(body) {
                            Ok(value) => result = value,
                            Err(err) => {
                                if err.is_return() {
                                    // Return from function, not just loop
                                    self.environment.pop_scope();
                                    return Err(err);
                                }
                                // For now, ignore other errors (break/continue would go here)
                            }
                        }

                        index += 1;
                    }
                } else {
                    self.environment.pop_scope();
                    return Err(RuntimeError::new(
                        format!("'{}' object is not iterable", iterable_value.type_name()),
                        ErrorType::TypeError,
                    ));
                }
            }
            _ => {
                self.environment.pop_scope();
                return Err(RuntimeError::new(
                    format!("'{}' object is not iterable", iterable_value.type_name()),
                    ErrorType::TypeError,
                ));
            }
        }

        self.environment.pop_scope();
        Ok(result)
    }

    pub(crate) fn evaluate_try_statement(
        &mut self,
        body: &AstNode,
        catch_clause: Option<&crate::parser::CatchClause>,
        finally_clause: Option<&AstNode>,
    ) -> Result<Value, RuntimeError> {
        let mut result = match self.evaluate(body) {
            Ok(value) => Ok(value),
            Err(error) => {
                if let Some(catch) = catch_clause {
                    // Enter new scope for catch block
                    self.environment.push_scope();

                    // Bind the error value to the catch variable
                    let error_value = if let Some(raised_value) = error.return_value {
                        raised_value
                    } else {
                        // Convert runtime error to string
                        Value::String(error.message)
                    };

                    self.environment.define(catch.variable.clone(), error_value);

                    // Execute catch block
                    let catch_result = self.evaluate(&catch.body);

                    // Clean up scope
                    self.environment.pop_scope();
                    catch_result
                } else {
                    // No catch clause, save the error for later
                    Err(error)
                }
            }
        };

        // Execute finally block if present (always runs)
        if let Some(finally_body) = finally_clause {
            match self.evaluate(finally_body) {
                Ok(_) => {
                    // Finally block succeeded, return original result
                }
                Err(finally_error) => {
                    // Finally block failed, this error takes precedence
                    result = Err(finally_error);
                }
            }
        }

        result
    }

    pub(crate) fn evaluate_class_expression(
        &mut self,
        members: &[crate::parser::ClassMember],
    ) -> Result<Value, RuntimeError> {
        let class = Class {
            name: None, // Anonymous class
            members: members.to_vec(),
        };
        Ok(Value::RustValue(Box::new(class)))
    }

    pub(crate) fn evaluate_class_declaration(
        &mut self,
        visibility: &Visibility,
        name: &str,
        members: &[crate::parser::ClassMember],
    ) -> Result<Value, RuntimeError> {
        // Create the class object
        let class = Class {
            name: Some(name.to_string()),
            members: members.to_vec(),
        };
        let class_value = Value::RustValue(Box::new(class));

        // Define the class in the current environment with visibility
        self.environment
            .define_with_visibility(name.to_string(), class_value, visibility.clone());

        // Class declarations evaluate to unit (like function declarations)
        Ok(Value::Unit)
    }

    pub(crate) fn evaluate_while_statement(
        &mut self,
        condition: &AstNode,
        body: &AstNode,
    ) -> Result<Value, RuntimeError> {
        // Create new scope for loop
        self.environment.push_scope();

        let mut result = Value::Unit;

        loop {
            // Evaluate condition
            let condition_value = self.evaluate(condition)?;

            // Check if condition is truthy
            if !condition_value.is_truthy()? {
                break;
            }

            // Execute body
            match self.evaluate(body) {
                Ok(value) => result = value,
                Err(err) => {
                    if err.is_return() {
                        // Return from function, not just loop
                        self.environment.pop_scope();
                        return Err(err);
                    }
                    // For now, ignore other errors (break/continue would go here)
                    // TODO: Handle break and continue statements
                }
            }
        }

        self.environment.pop_scope();
        Ok(result)
    }
}
