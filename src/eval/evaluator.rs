use super::{scope::ScopeRef, TypeConstant};
use crate::core::*;
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
        self.register_builtin_fn("int", crate::core::int);
        self.register_builtin_fn("float", crate::core::float);
        self.register_builtin_fn("raise", crate::core::raise);
        self.register_builtin_fn("parse", crate::core::parse_builtin);
        self.register_builtin_fn("macro", crate::core::macro_identity_builtin);
        self.register_builtin_fn("join", crate::core::join_builtin);
        self.register_builtin_fn("range", crate::core::range);
        self.register_builtin_fn("sum", crate::core::sum);
        self.register_builtin_fn("filter", crate::core::filter);
        self.register_builtin_fn("take_while", crate::core::take_while);

        // Register math functions
        crate::core::register_math(self);

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
    }

    pub fn register_builtin_fn(
        &mut self,
        name: &'static str,
        func: fn(Args) -> anyhow::Result<Value>,
    ) {
        let rust_fn = Value::from_rust(RustFunction::new(name, func));
        self.globals.define(name, rust_fn);
    }

    pub fn eval(&mut self, eval: &Value) -> EvalResult {
        match eval.eval(self) {
            Ok(result) => result,
            Err(e) => Err(ControlFlow::Error(ErrorKind::SystemError(e))),
        }
    }

    pub fn eval_str(&mut self, source: &str) -> anyhow::Result<Value> {
        // Parsing (includes lexical analysis)
        let ast = crate::parser::Parser::parse_str(source)?;

        // Compilation to evaluation IR
        let eval_ir = crate::eval::Compiler::compile(ast)?;

        // Evaluation
        self.eval(&eval_ir).map_err(|control_flow| {
            match control_flow {
                ControlFlow::Error(ErrorKind::SystemError(err)) => err,
                ControlFlow::Error(ErrorKind::RaisedError(value)) => {
                    // Convert raised value to string for error display
                    match value {
                        Value::String(s) => anyhow::anyhow!("{}", s),
                        _ => anyhow::anyhow!("{:?}", value),
                    }
                }
                ControlFlow::Return(val) => anyhow::anyhow!("Unexpected return: {:?}", val),
                ControlFlow::Break(val) => anyhow::anyhow!("Unexpected break: {:?}", val),
                ControlFlow::Continue => anyhow::anyhow!("Unexpected continue"),
            }
        })
    }

    /// Handle class constructor calls by evaluating default field expressions
    pub fn handle_class_constructor(
        &mut self,
        class: &crate::eval::Class,
        args: &[Value],
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
    pub fn cleanup_resources(&mut self, resources: &[(String, Value)]) {
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
    pub fn load_module(&self, module_name: &str) -> Result<ScopeRef, ControlFlow> {
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

        // Evaluate the module
        // TODO: Module-level evaluation may need special handling for different eval types
        // For now, evaluate everything normally through the trait dispatch
        module_evaluator.eval(&eval_ir)?;

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
    #[allow(dead_code)]
    pub fn match_pattern(
        &mut self,
        pattern: &crate::eval::EvalPattern,
        value: &Value,
    ) -> Result<(), ControlFlow> {
        use crate::eval::EvalPattern;

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

    pub fn match_pattern_matches(
        &mut self,
        pattern: &crate::eval::EvalMatchPattern,
        value: &Value,
    ) -> Result<bool, ControlFlow> {
        use crate::eval::EvalMatchPattern;

        match pattern {
            EvalMatchPattern::Literal(literal) => Ok(literal == value),
            EvalMatchPattern::Variable(_) => Ok(true),
            EvalMatchPattern::Wildcard => Ok(true),
        }
    }
}
