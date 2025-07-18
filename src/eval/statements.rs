use super::core::Evaluator;
use crate::lexer::SourceLocation;
use crate::parser::{AssignmentOperator, AstNode, ImportClause, ModulePath, Parameter, Visibility};
use crate::value::types::{ErrorType, Function, RuntimeError, Value};

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
            _ => Err(RuntimeError::new(
                "Complex assignment targets not implemented yet".to_string(),
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
}
