use super::scope::ScopeRef;
use crate::{core::*, eval::Eval};
use anyhow::anyhow;

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    Break(Value),
    Continue,
    Error(anyhow::Error),
}

pub type EvalResult = Result<Value, ControlFlow>;

#[derive(Debug, Clone)]
struct RustLeafFunction {
    params: Vec<String>,
    body: Eval,
    closure_env: ScopeRef, // Capture the environment where function was defined
}

impl RustLeafFunction {
    fn into_value(self) -> Value {
        Value::from_rust(self)
    }
}

impl RustValue for RustLeafFunction {
    fn call(&self, args: Args) -> anyhow::Result<Value> {
        // Check argument count first
        if args.len() != self.params.len() {
            return Err(anyhow!("Function expects {} arguments, got {}", self.params.len(), args.len()));
        }
        
        // Create new scope for function execution
        let function_scope = self.closure_env.child();
        
        // Set function name for better error messages
        args.set_function_name("user function");
        
        // Bind parameters to arguments using fluent API
        for param in &self.params {
            let arg_value = args.expect(param)?;
            function_scope.define(param.clone(), arg_value);
        }
        
        // Validate all args consumed
        args.complete()?;
        
        // Create evaluator with function scope - use same globals as parent
        let mut evaluator = Evaluator {
            globals: self.closure_env.clone(), // Use closure environment as globals for now
            current_env: function_scope,
        };
        
        // Evaluate function body
        match evaluator.eval(&self.body) {
            Ok(value) => Ok(value),
            Err(ControlFlow::Return(value)) => Ok(value),
            Err(ControlFlow::Error(err)) => Err(err),
            Err(other) => Err(anyhow!("Invalid control flow in function: {:?}", other)),
        }
    }
}

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
        self.register_builtin_fn("assert", crate::core::assert);
    }

    fn register_builtin_fn(&mut self, name: &'static str, func: fn(Args) -> anyhow::Result<Value>) {
        let rust_fn = Value::from_rust(RustFunction::new(name, func));
        self.globals.define(name.to_string(), rust_fn);
    }

    pub fn eval(&mut self, eval: &Eval) -> EvalResult {
        match eval {
            Eval::Block(statements, final_expr) => {
                // Create a new scope for the block
                let block_scope = self.current_env.child();
                let previous_env = std::mem::replace(&mut self.current_env, block_scope);
                
                let mut result = Value::Unit;

                // Execute each statement in the block
                for stmt in statements {
                    match self.eval(stmt) {
                        Ok(_) => {},
                        Err(e) => {
                            // Restore previous scope before propagating error
                            self.current_env = previous_env;
                            return Err(e);
                        }
                    }
                }

                // If there's a final expression, evaluate it and return its value
                if let Some(final_expr) = final_expr {
                    result = match self.eval(final_expr) {
                        Ok(val) => val,
                        Err(e) => {
                            // Restore previous scope before propagating error
                            self.current_env = previous_env;
                            return Err(e);
                        }
                    };
                }

                // Restore the previous scope
                self.current_env = previous_env;
                Ok(result)
            }
            Eval::Literal(value) => Ok(value.clone()),
            Eval::Variable(name) => {
                self.current_env.get(name)
                    .ok_or_else(|| ControlFlow::Error(anyhow!("Undefined variable: {}", name)))
            }
            Eval::Call(func_expr, args) => {
                // Evaluate the function expression
                let func_value = self.eval(func_expr)?;

                // Evaluate all arguments
                let arg_values: Result<Vec<Value>, ControlFlow> = args.iter()
                    .map(|arg| self.eval(arg))
                    .collect();
                let arg_values = arg_values?;

                // Create Args object for function call
                let args_obj = Args::positional(arg_values);

                // Call the function
                func_value.call(args_obj).map_err(|e| ControlFlow::Error(anyhow!(e)))
            }
            Eval::Declare(name, init_expr) => {
                let value = match init_expr {
                    Some(expr) => self.eval(expr)?,
                    None => Value::Unit,
                };
                self.current_env.define(name.clone(), value);
                Ok(Value::Unit)
            }
            Eval::Function(name, params, body) => {
                let function = RustLeafFunction {
                    params: params.clone(),
                    body: (**body).clone(),
                    closure_env: self.current_env.clone(),
                };
                let function_value = function.into_value();
                self.current_env.define(name.clone(), function_value);
                Ok(Value::Unit)
            }
            Eval::Assign(name, expr) => {
                let value = self.eval(expr)?;
                self.current_env.set(name, value)
                    .map_err(|err| ControlFlow::Error(anyhow!(err)))?;
                Ok(Value::Unit)
            }
            Eval::If(condition, then_expr, else_expr) => {
                let condition_val = self.eval(condition)?;
                
                // Check if condition is truthy
                let is_truthy = match condition_val {
                    Value::Bool(b) => b,
                    Value::Unit => false,
                    _ => true, // All other values are truthy (like Python/JavaScript)
                };
                
                if is_truthy {
                    self.eval(then_expr)
                } else {
                    match else_expr {
                        Some(expr) => self.eval(expr),
                        None => Ok(Value::Unit),
                    }
                }
            }
            Eval::Loop(body) => {
                loop {
                    match self.eval(body) {
                        Ok(_) => {
                            // Normal completion, continue looping
                            continue;
                        }
                        Err(ControlFlow::Break(value)) => {
                            // Break out of loop with value
                            return Ok(value);
                        }
                        Err(ControlFlow::Continue) => {
                            // Continue to next iteration
                            continue;
                        }
                        Err(other) => {
                            // Propagate other control flow (Return, Error)
                            return Err(other);
                        }
                    }
                }
            }
            Eval::Break(expr) => {
                let value = match expr {
                    Some(e) => self.eval(e)?,
                    None => Value::Unit,
                };
                Err(ControlFlow::Break(value))
            }
            Eval::Continue => {
                Err(ControlFlow::Continue)
            }
            Eval::GetAttr(obj_expr, attr_name) => {
                let obj_value = self.eval(obj_expr)?;
                match obj_value.get_attr(attr_name) {
                    Some(value) => Ok(value),
                    None => Err(ControlFlow::Error(anyhow!("No attribute '{}' on value {:?}", attr_name, obj_value))),
                }
            }
            
            // Built-in operations that don't use method dispatch
            Eval::LogicalAnd(left, right) => {
                let left_val = self.eval(left)?;
                // Short-circuit evaluation
                if !left_val.is_truthy().unwrap_or(true) {
                    Ok(left_val)
                } else {
                    self.eval(right)
                }
            }
            Eval::LogicalOr(left, right) => {
                let left_val = self.eval(left)?;
                // Short-circuit evaluation
                if left_val.is_truthy().unwrap_or(true) {
                    Ok(left_val)
                } else {
                    self.eval(right)
                }
            }
            Eval::LogicalNot(expr) => {
                let val = self.eval(expr)?;
                Ok(Value::Bool(!val.is_truthy().unwrap_or(true)))
            }
            Eval::Is(left, right) => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                // Identity comparison - same as == for now, could be object identity later
                Ok(Value::Bool(left_val == right_val))
            }
            
            x => Err(ControlFlow::Error(anyhow!("eval not implemented for: {:?}", x)))
        }
    }

}
