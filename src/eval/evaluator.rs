use super::{scope::ScopeRef, EvalTypeConstant, Params, RustLeafFunction, TypeConstant};
use crate::{core::*, eval::Eval};
use anyhow::anyhow;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ErrorKind {
    SystemError(anyhow::Error),
    RaisedError(Value),
}

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    Break(Value),
    Continue,
    Error(ErrorKind),
}

pub type EvalResult = Result<Value, ControlFlow>;

pub struct Evaluator {
    pub globals: ScopeRef,
    pub current_env: ScopeRef,
    pub current_dir: PathBuf,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self::new_with_dir(std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    pub fn new_with_dir(current_dir: PathBuf) -> Self {
        let globals = ScopeRef::new();

        let mut evaluator = Evaluator {
            globals: globals.clone(),
            current_env: globals,
            current_dir,
        };

        evaluator.register_builtins();
        evaluator
    }

    fn register_builtins(&mut self) {
        self.register_builtin_fn("print", print);
        self.register_builtin_fn("assert", crate::core::assert);
        self.register_builtin_fn("is_unit", crate::core::is_unit);
        self.register_builtin_fn("str", crate::core::str);
        self.register_builtin_fn("raise", crate::core::raise);
        self.register_builtin_fn("parse", crate::core::parse_builtin);
        self.register_builtin_fn("macro", crate::core::macro_identity_builtin);
        self.register_builtin_fn("join", crate::core::join_builtin);

        // Register type constants for `is` operator
        self.register_type_constants();
    }

    fn register_type_constants(&mut self) {
        // Create type constants as special values
        self.globals
            .define("Null", Value::from_rust(TypeConstant::new("Null")));
        self.globals
            .define("Unit", Value::from_rust(TypeConstant::new("Unit")));
        self.globals
            .define("Bool", Value::from_rust(TypeConstant::new("Bool")));
        self.globals
            .define("Int", Value::from_rust(TypeConstant::new("Int")));
        self.globals
            .define("Float", Value::from_rust(TypeConstant::new("Float")));
        self.globals
            .define("String", Value::from_rust(TypeConstant::new("String")));
        self.globals
            .define("List", Value::from_rust(TypeConstant::new("List")));
        self.globals
            .define("Dict", Value::from_rust(TypeConstant::new("Dict")));
        self.globals
            .define("Range", Value::from_rust(TypeConstant::new("Range")));
        self.globals
            .define("Function", Value::from_rust(TypeConstant::new("Function")));

        // Add Eval type constant for macro system
        self.globals
            .define("Eval", Value::from_rust(EvalTypeConstant::new()));
    }

    fn register_builtin_fn(&mut self, name: &'static str, func: fn(Args) -> anyhow::Result<Value>) {
        let rust_fn = Value::from_rust(RustFunction::new(name, func));
        self.globals.define(name, rust_fn);
    }

    pub fn eval(&mut self, eval: &Eval) -> EvalResult {
        match eval {
            Eval::Program(_) => self.eval_program(eval),
            Eval::Block(_, _) => self.eval_block(eval),
            Eval::Literal(_) => self.eval_literal(eval),
            Eval::Variable(_) => self.eval_variable(eval),
            Eval::Call(_, _) => self.eval_call(eval),
            Eval::Declare(_, _) => self.eval_declare(eval),
            Eval::DeclarePattern(_, _) => self.eval_declare_pattern(eval),
            Eval::Function(_) => self.eval_function(eval),
            Eval::Lambda(_) => self.eval_lambda(eval),
            Eval::Assign(_, _) => self.eval_assign(eval),
            Eval::If(_, _, _) => self.eval_if(eval),
            Eval::Loop(_) => self.eval_loop(eval),
            Eval::While(_, _) => self.eval_while(eval),
            Eval::For(_, _, _) => self.eval_for(eval),
            Eval::Break(_) => self.eval_break(eval),
            Eval::Continue => self.eval_continue(eval),
            Eval::Return(_) => self.eval_return(eval),
            Eval::GetAttr(_, _) => self.eval_get_attr(eval),
            Eval::LogicalAnd(_, _) => self.eval_logical_and(eval),
            Eval::LogicalOr(_, _) => self.eval_logical_or(eval),
            Eval::LogicalNot(_) => self.eval_logical_not(eval),
            Eval::Is(_, _) => self.eval_is(eval),
            Eval::List(_) => self.eval_list(eval),
            Eval::Dict(_) => self.eval_dict(eval),
            Eval::GetItem(_, _) => self.eval_get_item(eval),
            Eval::SetAttr(_, _, _) => self.eval_set_attr(eval),
            Eval::SetItem(_, _, _) => self.eval_set_item(eval),
            Eval::Try(_, _, _) => self.eval_try(eval),
            Eval::With(_) => self.eval_with(eval),
            Eval::ClassDecl(_) => self.eval_class_decl(eval),
            Eval::Import(_) => self.eval_import(eval),
            Eval::Match(_) => self.eval_match(eval),
            Eval::Macro(_) => self.eval_macro(eval),
        }
    }

    /// Handle class constructor calls by evaluating default field expressions
    fn handle_class_constructor(
        &mut self,
        class: &crate::eval::Class,
        args: &[crate::eval::Eval],
    ) -> EvalResult {
        use std::collections::HashMap;

        // Constructor call - create new instance
        if !args.is_empty() {
            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow::anyhow!(
                "Class constructor takes no arguments"
            ))));
        }

        // Create new instance with properly evaluated default field values
        let mut fields = HashMap::new();
        for (i, field_name) in class.field_names.iter().enumerate() {
            let value = if let Some(default_expr) = &class.field_defaults[i] {
                // Evaluate the default expression
                self.eval(default_expr)?
            } else {
                Value::Null
            };
            fields.insert(field_name.clone(), value);
        }

        Ok(Value::new_class_instance(crate::eval::ClassInstance {
            class_name: class.name.clone(),
            fields,
            methods: class.methods.clone(),
        }))
    }

    /// Helper method to cleanup resources by calling op_close() in reverse order
    fn cleanup_resources(&mut self, resources: &[(String, Value)]) {
        // Cleanup in reverse order
        for (_name, resource_value) in resources.iter().rev() {
            let close_method = resource_value.get_attr("op_close", self);
            if let Some(close_method) = close_method {
                // Bound methods already have self bound, so call with no arguments
                let args = Args::positional(vec![]);
                // Ignore errors during cleanup - we don't want cleanup errors to mask the original error
                let _ = close_method.call(args);
            }
        }
    }

    /// Load and evaluate a module, returning its scope
    fn load_module(&self, module_name: &str) -> Result<ScopeRef, ControlFlow> {
        // Resolve module path
        let module_path = self.resolve_module_path(module_name)?;

        // Read module file
        let module_content = std::fs::read_to_string(&module_path).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to read module '{}' at path '{}': {}",
                module_name,
                module_path.display(),
                e
            )))
        })?;

        // Parse module content
        let tokens = crate::lexer::Lexer::tokenize(&module_content).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to tokenize module '{}': {}",
                module_name,
                e
            )))
        })?;

        let ast = crate::parser::Parser::parse(tokens).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to parse module '{}': {}",
                module_name,
                e
            )))
        })?;

        // Compile to eval IR
        let eval_ir = crate::eval::Compiler::compile(ast).map_err(|e| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Failed to compile module '{}': {}",
                module_name,
                e
            )))
        })?;

        // Create new evaluator for module with correct current_dir
        let module_dir = module_path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let mut module_evaluator = Evaluator::new_with_dir(module_dir);

        // Evaluate the module - handle module-level definitions specially
        match eval_ir {
            Eval::Program(statements) => {
                // For modules, evaluate statements directly in the module scope
                for stmt in statements.iter() {
                    module_evaluator.eval(stmt)?;
                }
            }
            Eval::Block(statements, final_expr) => {
                // For modules, evaluate statements directly in the module scope (not in a child scope)
                for stmt in statements.iter() {
                    module_evaluator.eval(stmt)?;
                }
                // Evaluate final expression if present
                if let Some(final_expr) = &final_expr {
                    module_evaluator.eval(final_expr)?;
                }
            }
            _ => {
                // For non-program/block evaluation, evaluate normally
                module_evaluator.eval(&eval_ir)?;
            }
        }

        // Return the module's current scope (which includes both built-ins and module items)
        // We'll filter out built-ins at import time instead
        Ok(module_evaluator.current_env)
    }

    /// Resolve module path based on current directory and module name
    fn resolve_module_path(&self, module_name: &str) -> Result<PathBuf, ControlFlow> {
        // For now, implement simple relative path resolution
        // Convert module::submodule to module/submodule.rustleaf
        let relative_path = module_name.replace("::", "/") + ".rustleaf";
        let full_path = self.current_dir.join(relative_path);

        if !full_path.exists() {
            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Module file not found: {}",
                full_path.display()
            ))));
        }

        Ok(full_path)
    }

    /// Pattern matching helper - binds variables from patterns
    fn match_pattern(
        &mut self,
        pattern: &crate::eval::core::EvalPattern,
        value: &Value,
    ) -> Result<(), ControlFlow> {
        use crate::eval::core::EvalPattern;

        match pattern {
            EvalPattern::Variable(name) => {
                // Simple variable binding
                self.current_env.define(name.clone(), value.clone());
                Ok(())
            }
            EvalPattern::List(patterns) => {
                // List destructuring
                match value {
                    Value::List(list_ref) => {
                        let list = list_ref.borrow();

                        if list.len() != patterns.len() {
                            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                "Cannot destructure list of length {} into pattern with {} elements",
                                list.len(),
                                patterns.len()
                            ))));
                        }

                        // Recursively match each pattern with corresponding list element
                        for (pattern, list_value) in patterns.iter().zip(list.iter()) {
                            self.match_pattern(pattern, list_value)?;
                        }

                        Ok(())
                    }
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot destructure non-list value {:?} with list pattern",
                        value
                    )))),
                }
            }
            EvalPattern::ListRest(patterns, rest_name) => {
                // List destructuring with rest capture: [first, *rest]
                match value {
                    Value::List(list_ref) => {
                        let list = list_ref.borrow();

                        // Must have at least as many elements as fixed patterns
                        if list.len() < patterns.len() {
                            return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                "Cannot destructure list of length {} with rest pattern expecting at least {} elements",
                                list.len(),
                                patterns.len()
                            ))));
                        }

                        // Match fixed patterns at the beginning
                        for (i, pattern) in patterns.iter().enumerate() {
                            let list_value = &list[i];
                            self.match_pattern(pattern, list_value)?;
                        }

                        // Capture the rest as a new list (if rest_name is provided)
                        if let Some(rest_var) = rest_name {
                            let rest_values: Vec<Value> = list[patterns.len()..].to_vec();
                            let rest_list = Value::new_list_with_values(rest_values);
                            self.current_env.define(rest_var.clone(), rest_list);
                        }

                        Ok(())
                    }
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot destructure non-list value {:?} with list rest pattern",
                        value
                    )))),
                }
            }
            EvalPattern::Dict(dict_patterns) => {
                // Dict destructuring
                match value {
                    Value::Dict(dict_ref) => {
                        let dict = dict_ref.borrow();

                        for dict_pattern in dict_patterns {
                            // Look up the value in the dict using string key directly
                            let dict_value = dict.get(&dict_pattern.key).ok_or_else(|| {
                                ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                    "Key '{}' not found in dict during destructuring",
                                    dict_pattern.key
                                )))
                            })?;

                            // Determine the variable name (alias if present, otherwise key)
                            let var_name = dict_pattern.alias.as_ref().unwrap_or(&dict_pattern.key);

                            // Bind the value to the variable
                            self.current_env
                                .define(var_name.clone(), dict_value.clone());
                        }
                        Ok(())
                    }
                    _ => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Cannot destructure non-dict value {:?} with dict pattern",
                        value
                    )))),
                }
            }
        }
    }

    fn match_pattern_matches(
        &self,
        pattern: &crate::eval::core::EvalMatchPattern,
        value: &Value,
    ) -> Result<bool, ControlFlow> {
        use crate::eval::core::EvalMatchPattern;

        match pattern {
            EvalMatchPattern::Literal(literal) => Ok(literal == value),
            EvalMatchPattern::Variable(_) => Ok(true),
            EvalMatchPattern::Wildcard => Ok(true),
        }
    }

    // Individual evaluation methods
    fn eval_program(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Program(statements) = eval {
            for stmt in statements.iter() {
                self.eval(stmt)?;
            }
            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_block(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Block(statements, final_expr) = eval {
            let block_scope = self.current_env.child();
            let previous_env = std::mem::replace(&mut self.current_env, block_scope);

            let mut result = Value::Unit;

            for stmt in statements.iter() {
                match self.eval(stmt) {
                    Ok(_) => {}
                    Err(e) => {
                        self.current_env = previous_env;
                        return Err(e);
                    }
                }
            }

            if let Some(final_expr) = final_expr {
                result = match self.eval(final_expr) {
                    Ok(val) => val,
                    Err(e) => {
                        self.current_env = previous_env;
                        return Err(e);
                    }
                };
            }

            self.current_env = previous_env;
            Ok(result)
        } else {
            unreachable!()
        }
    }

    fn eval_literal(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Literal(value) = eval {
            Ok(value.clone())
        } else {
            unreachable!()
        }
    }

    fn eval_variable(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Variable(name) = eval {
            self.current_env.get(name).ok_or_else(|| {
                ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                    "Undefined variable: {}",
                    name
                )))
            })
        } else {
            unreachable!()
        }
    }

    fn eval_call(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Call(func_expr, args) = eval {
            let func_value = self.eval(func_expr)?;

            if let Value::Class(class_ref) = &func_value {
                let class = class_ref.borrow();
                return self.handle_class_constructor(&class, args);
            }

            let arg_values: Result<Vec<Value>, ControlFlow> =
                args.iter().map(|arg| self.eval(arg)).collect();
            let arg_values = arg_values?;

            let args_obj = Args::positional(arg_values);

            let result = func_value
                .call(args_obj)
                .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))?;

            if let Value::Raised(error_value) = result {
                return Err(ControlFlow::Error(ErrorKind::RaisedError(*error_value)));
            }

            Ok(result)
        } else {
            unreachable!()
        }
    }

    fn eval_declare(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Declare(name, init_expr) = eval {
            let value = match init_expr {
                Some(expr) => self.eval(expr)?,
                None => Value::Unit,
            };
            self.current_env.define(name, value);
            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_declare_pattern(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::DeclarePattern(pattern, init_expr) = eval {
            let value = self.eval(init_expr)?;
            self.match_pattern(pattern, &value)?;
            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_function(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Function(data) = eval {
            let function = RustLeafFunction::new(
                Params::from_vec(data.params.clone()),
                (*data.body).clone(),
                self.current_env.clone(),
            );
            let function_value = function.into_value();
            self.current_env.define(&data.name, function_value);
            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_lambda(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Lambda(data) = eval {
            let param_data = Params::from_names(&data.params);
            let function =
                RustLeafFunction::new(param_data, (*data.body).clone(), self.current_env.clone());
            Ok(function.into_value())
        } else {
            unreachable!()
        }
    }

    fn eval_assign(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Assign(name, expr) = eval {
            let value = self.eval(expr)?;
            self.current_env
                .set(name, value)
                .map_err(|err| ControlFlow::Error(ErrorKind::SystemError(anyhow!(err))))?;
            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_if(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::If(condition, then_expr, else_expr) = eval {
            let condition_val = self.eval(condition)?;

            if condition_val.is_truthy() {
                self.eval(then_expr)
            } else {
                match else_expr {
                    Some(expr) => self.eval(expr),
                    None => Ok(Value::Unit),
                }
            }
        } else {
            unreachable!()
        }
    }

    fn eval_loop(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Loop(body) = eval {
            loop {
                match self.eval(body) {
                    Ok(_) => {
                        continue;
                    }
                    Err(ControlFlow::Break(value)) => {
                        return Ok(value);
                    }
                    Err(ControlFlow::Continue) => {
                        continue;
                    }
                    Err(other) => {
                        return Err(other);
                    }
                }
            }
        } else {
            unreachable!()
        }
    }

    fn eval_while(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::While(condition, body) = eval {
            loop {
                let condition_val = self.eval(condition)?;

                if !condition_val.is_truthy() {
                    return Ok(Value::Unit);
                }

                match self.eval(body) {
                    Ok(_) => {
                        continue;
                    }
                    Err(ControlFlow::Break(value)) => {
                        return Ok(value);
                    }
                    Err(ControlFlow::Continue) => {
                        continue;
                    }
                    Err(other) => {
                        return Err(other);
                    }
                }
            }
        } else {
            unreachable!()
        }
    }

    fn eval_for(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::For(var_name, iter_expr, body) = eval {
            let iter_value = self.eval(iter_expr)?;

            let loop_scope = self.current_env.child();
            let previous_env = std::mem::replace(&mut self.current_env, loop_scope);

            let mut result = Value::Unit;

            let mut iterator = match iter_value.op_iter() {
                Ok(iter) => iter,
                Err(e) => {
                    self.current_env = previous_env;
                    return Err(ControlFlow::Error(ErrorKind::SystemError(e)));
                }
            };

            loop {
                let next_item = match iterator.op_next() {
                    Ok(item) => item,
                    Err(e) => {
                        self.current_env = previous_env;
                        return Err(ControlFlow::Error(ErrorKind::SystemError(e)));
                    }
                };

                match next_item {
                    Some(item) => {
                        self.current_env.define(var_name, item);

                        match self.eval(body) {
                            Ok(_) => {}
                            Err(ControlFlow::Break(value)) => {
                                result = value;
                                break;
                            }
                            Err(ControlFlow::Continue) => {
                                continue;
                            }
                            Err(other) => {
                                self.current_env = previous_env;
                                return Err(other);
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            self.current_env = previous_env;
            Ok(result)
        } else {
            unreachable!()
        }
    }

    fn eval_break(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Break(expr) = eval {
            let value = match expr {
                Some(e) => self.eval(e)?,
                None => Value::Unit,
            };
            Err(ControlFlow::Break(value))
        } else {
            unreachable!()
        }
    }

    fn eval_continue(&mut self, _eval: &Eval) -> EvalResult {
        Err(ControlFlow::Continue)
    }

    fn eval_return(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Return(expr) = eval {
            let value = match expr {
                Some(e) => self.eval(e)?,
                None => Value::Unit,
            };
            Err(ControlFlow::Return(value))
        } else {
            unreachable!()
        }
    }

    fn eval_get_attr(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::GetAttr(obj_expr, attr_name) = eval {
            let obj_value = self.eval(obj_expr)?;

            match obj_value.get_attr(attr_name, self) {
                Some(value) => Ok(value),
                None => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                    "No attribute '{}' on value {:?}",
                    attr_name,
                    obj_value
                )))),
            }
        } else {
            unreachable!()
        }
    }

    fn eval_logical_and(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::LogicalAnd(left, right) = eval {
            let left_val = self.eval(left)?;
            if !left_val.is_truthy() {
                Ok(left_val)
            } else {
                self.eval(right)
            }
        } else {
            unreachable!()
        }
    }

    fn eval_logical_or(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::LogicalOr(left, right) = eval {
            let left_val = self.eval(left)?;
            if left_val.is_truthy() {
                Ok(left_val)
            } else {
                self.eval(right)
            }
        } else {
            unreachable!()
        }
    }

    fn eval_logical_not(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::LogicalNot(expr) = eval {
            let val = self.eval(expr)?;
            Ok(Value::Bool(!val.is_truthy()))
        } else {
            unreachable!()
        }
    }

    fn eval_is(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Is(left, right) = eval {
            let left_val = self.eval(left)?;
            let right_val = self.eval(right)?;

            if let Value::RustValue(rust_val_ref) = &right_val {
                let rust_val = rust_val_ref.borrow();
                match rust_val.op_is(&left_val) {
                    Ok(result) => return Ok(result),
                    Err(_) => {}
                }
            }

            Ok(Value::Bool(left_val == right_val))
        } else {
            unreachable!()
        }
    }

    fn eval_list(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::List(elements) = eval {
            let mut list_values = Vec::new();
            for element in elements.iter() {
                list_values.push(self.eval(element)?);
            }
            Ok(Value::new_list_with_values(list_values))
        } else {
            unreachable!()
        }
    }

    fn eval_dict(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Dict(pairs) = eval {
            let mut dict_map = indexmap::IndexMap::new();
            for (key_expr, value_expr) in pairs.iter() {
                let key_val = self.eval(key_expr)?;
                let value_val = self.eval(value_expr)?;

                let key_str = match key_val {
                    Value::String(s) => s,
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    _ => {
                        return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                            "Dictionary keys must be strings, numbers, or booleans, got {:?}",
                            key_val
                        ))))
                    }
                };

                dict_map.insert(key_str, value_val);
            }
            Ok(Value::new_dict_with_map(dict_map))
        } else {
            unreachable!()
        }
    }

    fn eval_get_item(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::GetItem(obj_expr, index_expr) = eval {
            let obj_value = self.eval(obj_expr)?;
            let index_value = self.eval(index_expr)?;

            match obj_value.get_attr("op_get_item", self) {
                Some(method) => {
                    let args = Args::positional(vec![index_value]);
                    method
                        .call(args)
                        .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))
                }
                None => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                    "No op_get_item method on value {:?}",
                    obj_value
                )))),
            }
        } else {
            unreachable!()
        }
    }

    fn eval_set_attr(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::SetAttr(obj_expr, attr_name, value_expr) = eval {
            let obj_value = self.eval(obj_expr)?;
            let new_value = self.eval(value_expr)?;

            match obj_value.set_attr(attr_name, new_value) {
                Ok(_) => Ok(Value::Unit),
                Err(err) => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(err)))),
            }
        } else {
            unreachable!()
        }
    }

    fn eval_set_item(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::SetItem(obj_expr, index_expr, value_expr) = eval {
            let obj_value = self.eval(obj_expr)?;
            let index_value = self.eval(index_expr)?;
            let new_value = self.eval(value_expr)?;

            match obj_value.get_attr("op_set_item", self) {
                Some(method) => {
                    let args = Args::positional(vec![index_value, new_value]);
                    method
                        .call(args)
                        .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))?;
                    Ok(Value::Unit)
                }
                None => Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                    "No op_set_item method on value {:?}",
                    obj_value
                )))),
            }
        } else {
            unreachable!()
        }
    }

    fn eval_try(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Try(body, catch_var, catch_body) = eval {
            match self.eval(body) {
                Ok(value) => Ok(value),
                Err(ControlFlow::Error(ErrorKind::RaisedError(error_value))) => {
                    let catch_scope = self.current_env.child();
                    let previous_env = std::mem::replace(&mut self.current_env, catch_scope);

                    self.current_env.define(catch_var.clone(), error_value);

                    let result = self.eval(catch_body);

                    self.current_env = previous_env;

                    result
                }
                Err(other_error) => Err(other_error),
            }
        } else {
            unreachable!()
        }
    }

    fn eval_with(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::With(data) = eval {
            let with_scope = self.current_env.child();
            let previous_env = std::mem::replace(&mut self.current_env, with_scope);

            let mut resource_values = Vec::new();

            for (name, resource_expr) in &data.resources {
                match self.eval(resource_expr) {
                    Ok(resource_value) => {
                        self.current_env
                            .define(name.clone(), resource_value.clone());

                        let open_method = resource_value.get_attr("op_open", self);
                        if let Some(open_method) = open_method {
                            let args = Args::positional(vec![]);
                            if let Err(e) = open_method.call(args) {
                                self.cleanup_resources(&resource_values);
                                self.current_env = previous_env;
                                return Err(ControlFlow::Error(ErrorKind::SystemError(e)));
                            }
                        }

                        resource_values.push((name.clone(), resource_value));
                    }
                    Err(e) => {
                        self.cleanup_resources(&resource_values);
                        self.current_env = previous_env;
                        return Err(e);
                    }
                }
            }

            let result = match self.eval(&data.body) {
                Ok(val) => val,
                Err(e) => {
                    self.cleanup_resources(&resource_values);
                    self.current_env = previous_env;
                    return Err(e);
                }
            };

            self.cleanup_resources(&resource_values);

            self.current_env = previous_env;
            Ok(result)
        } else {
            unreachable!()
        }
    }

    fn eval_class_decl(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::ClassDecl(data) = eval {
            use crate::eval::Class;

            let class = Class::new(
                data.name.clone(),
                data.field_names.clone(),
                data.field_defaults.clone(),
                data.methods.clone(),
            );

            let class_value = Value::new_class(class);
            self.current_env.define(data.name.clone(), class_value);

            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_import(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Import(data) = eval {
            let module_scope = self.load_module(&data.module)?;

            let builtin_names: std::collections::HashSet<&str> = [
                "print", "assert", "is_unit", "str", "raise", "Null", "Unit", "Bool", "Int",
                "Float", "String", "List", "Dict", "Range", "Function",
            ]
            .iter()
            .cloned()
            .collect();

            match &data.items {
                ImportItems::All => {
                    for (name, value) in module_scope.iter() {
                        if !builtin_names.contains(name.as_str()) {
                            self.current_env.define(name, value);
                        }
                    }
                }
                ImportItems::Specific(import_items) => {
                    for import_item in import_items {
                        let item_name = &import_item.name;
                        let alias = import_item.alias.as_ref().unwrap_or(item_name);

                        match module_scope.get(item_name) {
                            Some(value) => {
                                self.current_env.define(alias.clone(), value);
                            }
                            None => {
                                let available_items: Vec<_> = module_scope
                                    .iter()
                                    .into_iter()
                                    .filter(|(k, _)| !builtin_names.contains(k.as_str()))
                                    .map(|(k, _)| k)
                                    .collect();
                                return Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                                    "Module '{}' does not have item '{}'. Available items: {:?}",
                                    data.module,
                                    item_name,
                                    available_items
                                ))));
                            }
                        }
                    }
                }
            }

            Ok(Value::Unit)
        } else {
            unreachable!()
        }
    }

    fn eval_match(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Match(data) = eval {
            let match_value = self.eval(&data.expr)?;

            for case in &data.cases {
                if self.match_pattern_matches(&case.pattern, &match_value)? {
                    if let Some(guard) = &case.guard {
                        let guard_result = self.eval(guard)?;
                        if !guard_result.is_truthy() {
                            continue;
                        }
                    }

                    let match_scope = self.current_env.child();
                    let previous_env = std::mem::replace(&mut self.current_env, match_scope);

                    if let crate::eval::core::EvalMatchPattern::Variable(var_name) =
                        &case.pattern
                    {
                        self.current_env
                            .define(var_name.clone(), match_value.clone());
                    }

                    let result = self.eval(&case.body);

                    self.current_env = previous_env;

                    return result;
                }
            }

            Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No matching case found in match expression for value: {:?}",
                match_value
            ))))
        } else {
            unreachable!()
        }
    }

    fn eval_macro(&mut self, eval: &Eval) -> EvalResult {
        if let Eval::Macro(data) = eval {
            let macro_function = self.eval(&data.macro_fn)?;

            let mut macro_args = Vec::new();

            macro_args.push(Value::from_rust(EvalNode::new((*data.target).clone())));

            for arg in &data.args {
                macro_args.push(self.eval(arg)?);
            }

            let args_obj = Args::positional(macro_args);
            let transformed_node = macro_function
                .call(args_obj)
                .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)))?;

            if let Value::RustValue(rust_val_ref) = transformed_node {
                let rust_val = rust_val_ref.borrow();
                match rust_val.eval(self) {
                    Ok(result) => return result,
                    Err(e) => return Err(ControlFlow::Error(ErrorKind::SystemError(e))),
                }
            }

            Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Macro function must return an EvalNode, got {:?}",
                transformed_node
            ))))
        } else {
            unreachable!()
        }
    }
}
