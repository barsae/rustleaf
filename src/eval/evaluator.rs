use super::scope::ScopeRef;
use crate::{core::*, eval::Eval};
use anyhow::{anyhow, Result};

pub struct Evaluator {
    globals: ScopeRef,
    current_env: ScopeRef,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        let globals = ScopeRef::new();

        let mut evaluator = Evaluator {
            globals: globals.clone(),
            current_env: globals,
        };

        // Register built-in functions
        evaluator.register_builtins();
        evaluator
    }

    fn register_builtins(&mut self) {
        self.register_builtin_fn("print", print);
    }

    fn register_builtin_fn(&mut self, name: &'static str, func: fn(Args) -> Result<Value>) {
        let rust_fn = Value::rust_value(Box::new(
            RustFunction::new(name, func)
        ));
        self.globals.define(name.to_string(), rust_fn);
    }

    pub fn eval(&mut self, eval: &Eval) -> Result<Value> {
        match eval {
            Eval::Block(statements, final_expr) => {
                let mut result = Value::Unit;

                // Execute each statement in the block
                for stmt in statements {
                    self.eval(stmt)?;
                }

                // If there's a final expression, evaluate it and return its value
                if let Some(final_expr) = final_expr {
                    result = self.eval(final_expr)?;
                }

                Ok(result)
            }
            Eval::Literal(value) => Ok(value.clone()),
            Eval::Variable(name) => {
                self.current_env.get(name)
                    .ok_or_else(|| anyhow!("Undefined variable: {}", name))
            }
            Eval::Call(func_expr, args) => {
                // Evaluate the function expression
                let func_value = self.eval(func_expr)?;

                // Evaluate all arguments
                let arg_values: Result<Vec<Value>> = args.iter()
                    .map(|arg| self.eval(arg))
                    .collect();
                let arg_values = arg_values?;

                // Create Args object for function call
                let args_obj = Args::positional(arg_values);

                // Call the function
                func_value.call(args_obj)
            }
            x => Err(anyhow!("eval not implemented for: {:?}", eval))
        }
    }
}
