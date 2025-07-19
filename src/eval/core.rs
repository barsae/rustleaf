use super::environment::Environment;
use crate::lexer::SourceLocation;
use crate::lexer::{Lexer, LiteralValue};
use crate::modules::{ModuleEnvironment, ModuleLoader};
use crate::parser::{AstNode, ModulePath, ModulePathRoot, Parser};
use crate::value::types::{ErrorType, RuntimeError, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub path: PathBuf,
    pub environment: ModuleEnvironment,
    pub is_loading: bool,
}

pub struct Evaluator {
    pub(crate) environment: Environment,
    /// Base directory for module resolution
    base_dir: PathBuf,
    /// Currently loaded modules (path -> module info)
    modules: HashMap<PathBuf, ModuleInfo>,
    /// Import stack for circular dependency detection
    import_stack: Vec<PathBuf>,
    /// Current file being evaluated (for relative imports)
    pub(crate) current_file: Option<PathBuf>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            environment: Environment::new(),
            base_dir: PathBuf::from("."),
            modules: HashMap::new(),
            import_stack: Vec::new(),
            current_file: None,
        }
    }

    pub fn new_with_base_dir(base_dir: impl Into<PathBuf>) -> Self {
        Evaluator {
            environment: Environment::new(),
            base_dir: base_dir.into(),
            modules: HashMap::new(),
            import_stack: Vec::new(),
            current_file: None,
        }
    }

    pub fn set_current_file(&mut self, file_path: impl Into<PathBuf>) {
        self.current_file = Some(file_path.into());
    }

    /// Resolve a module path to a filesystem path
    fn resolve_module_path(
        &self,
        module_path: &ModulePath,
        current_file: Option<&Path>,
    ) -> Result<PathBuf, RuntimeError> {
        // Get current directory for module resolution
        let current_dir = if let Some(current) = current_file {
            current.parent().unwrap_or(&self.base_dir).to_path_buf()
        } else {
            self.base_dir.clone()
        };

        let loader = ModuleLoader::new(&current_dir);

        // Use ModuleLoader for path resolution
        loader
            .resolve_path(module_path)
            .map_err(|e| RuntimeError::new(e.to_string(), ErrorType::ImportError))
            .and_then(|resolved_path| {
                // Check if file exists (ModuleLoader doesn't validate existence in resolve_path)
                if !resolved_path.exists() {
                    return Err(RuntimeError::new(
                        format!(
                            "Module not found: '{}' (resolved to {})",
                            format_module_path(module_path),
                            resolved_path.display()
                        ),
                        ErrorType::ImportError,
                    ));
                }
                Ok(resolved_path)
            })
    }

    /// Load and evaluate a module, returning its path for later access
    pub(crate) fn load_module(
        &mut self,
        module_path: &ModulePath,
        _location: SourceLocation,
    ) -> Result<PathBuf, RuntimeError> {
        let resolved_path = self.resolve_module_path(module_path, self.current_file.as_deref())?;

        // Check for circular dependency
        if self.import_stack.contains(&resolved_path) {
            let cycle = self
                .import_stack
                .iter()
                .skip_while(|&p| p != &resolved_path)
                .map(|p| p.file_stem().unwrap().to_str().unwrap())
                .collect::<Vec<_>>();
            let current_module = resolved_path.file_stem().unwrap().to_str().unwrap();

            return Err(RuntimeError::new(
                format!(
                    "Circular dependency detected: {} → {}",
                    cycle.join(" → "),
                    current_module
                ),
                ErrorType::ImportError,
            ));
        }

        // Return existing module path if already loaded
        if let Some(module_info) = self.modules.get(&resolved_path) {
            if module_info.is_loading {
                // Create the same format as the import_stack check above
                let cycle = self
                    .import_stack
                    .iter()
                    .map(|p| p.file_stem().unwrap().to_str().unwrap())
                    .collect::<Vec<_>>();
                let current_module = resolved_path.file_stem().unwrap().to_str().unwrap();

                return Err(RuntimeError::new(
                    format!(
                        "Circular dependency detected: {} → {}",
                        cycle.join(" → "),
                        current_module
                    ),
                    ErrorType::ImportError,
                ));
            }
            return Ok(resolved_path);
        }

        // Mark as loading to prevent infinite recursion
        self.modules.insert(
            resolved_path.clone(),
            ModuleInfo {
                path: resolved_path.clone(),
                environment: ModuleEnvironment::new(),
                is_loading: true,
            },
        );

        // Add to import stack
        self.import_stack.push(resolved_path.clone());

        // Load and parse the module file
        let source = fs::read_to_string(&resolved_path).map_err(|e| {
            RuntimeError::new(
                format!("IO error loading {}: {}", resolved_path.display(), e),
                ErrorType::ImportError,
            )
        })?;

        let tokens = Lexer::new(&source).map_err(|e| {
            RuntimeError::new(
                format!("Lexer error in {}: {:?}", resolved_path.display(), e),
                ErrorType::ImportError,
            )
        })?;

        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        // Save current context
        let saved_current_file = self.current_file.clone();
        self.current_file = Some(resolved_path.clone());

        // Evaluate the module AST in an isolated environment
        self.environment.push_scope();

        // Evaluate the module - this will populate the current scope with bindings and their visibility
        let _result = self.evaluate(&ast)?;

        // Extract all bindings from the current scope with their visibility
        let all_bindings = self.environment.get_all_bindings_with_visibility();

        // Create module environment and populate it with all bindings (public and private)
        let mut module_environment = ModuleEnvironment::new();
        for (name, (value, visibility)) in all_bindings {
            module_environment.define(name, value, visibility);
        }

        self.environment.pop_scope();

        // Restore context
        self.current_file = saved_current_file;

        // Remove from import stack
        self.import_stack.pop();

        // Update module info with completed environment
        let module_info = self.modules.get_mut(&resolved_path).unwrap();
        module_info.environment = module_environment;
        module_info.is_loading = false;

        Ok(resolved_path)
    }

    /// Get a loaded module's environment
    pub(crate) fn get_module(&self, path: &Path) -> Option<&ModuleEnvironment> {
        self.modules.get(path).map(|info| &info.environment)
    }

    pub fn evaluate(&mut self, node: &AstNode) -> Result<Value, RuntimeError> {
        match node {
            // Literals
            AstNode::Literal(literal, _) => self.evaluate_literal(literal),

            // Interpolated strings
            AstNode::InterpolatedString { parts, .. } => self.evaluate_interpolated_string(parts),

            // Identifiers
            AstNode::Identifier(name, _) => self.environment.get(name),

            // Binary operations
            AstNode::BinaryOp {
                left,
                operator,
                right,
                ..
            } => self.evaluate_binary_op(left, operator, right),

            // Unary operations
            AstNode::UnaryOp {
                operator, operand, ..
            } => self.evaluate_unary_op(operator, operand),

            // Function calls
            AstNode::FunctionCall {
                function,
                arguments,
                ..
            } => self.evaluate_function_call(function, arguments),

            // Property access
            AstNode::PropertyAccess {
                object, property, ..
            } => self.evaluate_property_access(object, property),

            // Index access
            AstNode::IndexAccess { object, index, .. } => self.evaluate_index_access(object, index),

            // List literals
            AstNode::ListLiteral { elements, .. } => self.evaluate_list_literal(elements),

            // Dict literals
            AstNode::DictLiteral { entries, .. } => self.evaluate_dict_literal(entries),

            // If expressions
            AstNode::If {
                condition,
                then_branch,
                else_ifs,
                else_branch,
                ..
            } => self.evaluate_if_expression(condition, then_branch, else_ifs, else_branch),

            // Block expressions
            AstNode::Block { statements, .. } => self.evaluate_block(statements),

            // Variable declarations
            AstNode::VariableDeclaration {
                visibility,
                name,
                value,
                ..
            } => self.evaluate_variable_declaration(visibility, name, value),

            // Assignment
            AstNode::Assignment {
                target,
                operator,
                value,
                ..
            } => self.evaluate_assignment(target, operator, value),

            // Expression statements
            AstNode::ExpressionStatement { expression, .. } => self.evaluate(expression),

            // Program
            AstNode::Program { items, .. } => self.evaluate_program(items),

            // Expression forms
            AstNode::Match { .. } => {
                todo!("Match expressions not implemented yet")
            }
            AstNode::Try {
                body, catch_clause, ..
            } => self.evaluate_try_expression(body, catch_clause.as_ref().map(|c| c.as_ref())),
            AstNode::AnonymousFunction {
                parameters, body, ..
            } => self.evaluate_anonymous_function(parameters, body),
            AstNode::ClassExpression { members, .. } => self.evaluate_class_expression(members),

            // Statement forms
            AstNode::FunctionDeclaration {
                visibility,
                name,
                parameters,
                body,
                ..
            } => self.evaluate_function_declaration(visibility, name, parameters, body),
            AstNode::ClassDeclaration {
                visibility,
                name,
                members,
                ..
            } => self.evaluate_class_declaration(visibility, name, members),
            AstNode::ImportStatement {
                path,
                clause,
                location,
            } => self.evaluate_import_statement(path, clause, location.clone()),
            AstNode::WhileStatement {
                condition, body, ..
            } => self.evaluate_while_statement(condition, body),
            AstNode::ForStatement {
                variable,
                index_variable,
                iterable,
                body,
                ..
            } => self.evaluate_for_statement(variable, index_variable, iterable, body),
            AstNode::MatchStatement { .. } => {
                todo!("Match statements not implemented yet")
            }
            AstNode::TryStatement {
                body,
                catch_clause,
                finally_clause,
                ..
            } => self.evaluate_try_statement(
                body,
                catch_clause.as_ref(),
                finally_clause.as_ref().map(|f| f.as_ref()),
            ),
            AstNode::WithStatement { .. } => {
                todo!("With statements not implemented yet")
            }
            AstNode::BreakStatement { .. } => {
                todo!("Break statements not implemented yet")
            }
            AstNode::ContinueStatement { .. } => {
                todo!("Continue statements not implemented yet")
            }
            AstNode::ReturnStatement { value, .. } => self.evaluate_return_statement(value),
            AstNode::Empty { .. } => Ok(Value::Unit),
        }
    }

    fn evaluate_interpolated_string(
        &mut self,
        parts: &[crate::parser::ast::InterpolationPart],
    ) -> Result<Value, RuntimeError> {
        let mut result = String::new();

        for part in parts {
            match part {
                crate::parser::ast::InterpolationPart::Text(text) => {
                    result.push_str(text);
                }
                crate::parser::ast::InterpolationPart::Expression(expr) => {
                    let value = self.evaluate(expr)?;
                    // Convert the value to string for interpolation
                    result.push_str(&value.to_display_string());
                }
            }
        }

        Ok(Value::String(result))
    }

    fn evaluate_literal(&self, literal: &LiteralValue) -> Result<Value, RuntimeError> {
        match literal {
            LiteralValue::Null => Ok(Value::Null),
            LiteralValue::Boolean(b) => Ok(Value::Bool(*b)),
            LiteralValue::Integer(i) => Ok(Value::Int(*i)),
            LiteralValue::Float(f) => Ok(Value::Float(*f)),
            LiteralValue::String(s) => Ok(Value::String(s.clone())),
            LiteralValue::InterpolatedString(_) => {
                // This should never be reached since interpolated strings
                // are handled by AstNode::InterpolatedString
                Err(RuntimeError::new(
                    "Unexpected interpolated string literal".to_string(),
                    ErrorType::RuntimeError,
                ))
            }
        }
    }
}

/// Helper function to format module paths for error messages
fn format_module_path(module_path: &ModulePath) -> String {
    match module_path.root_type {
        ModulePathRoot::Super => format!("super::{}", module_path.segments.join("::")),
        ModulePathRoot::Relative => module_path.segments.join("::"),
    }
}
