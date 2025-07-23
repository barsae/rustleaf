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
        Value::rust_value(Box::new(self))
    }
}

impl RustValue for RustLeafFunction {
    fn get_attr(&self, _name: &str) -> Option<Value> {
        None
    }
    
    fn set_attr(&mut self, _name: &str, _value: Value) -> Result<(), String> {
        Err("Cannot set attributes on function".to_string())
    }
    
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
    }

    fn register_builtin_fn(&mut self, name: &'static str, func: fn(Args) -> anyhow::Result<Value>) {
        let rust_fn = Value::rust_value(Box::new(
            RustFunction::new(name, func)
        ));
        self.globals.define(name.to_string(), rust_fn);
    }

    pub fn eval(&mut self, eval: &Eval) -> EvalResult {
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
            Eval::BinaryOp(op, left, right) => {
                let left_val = self.eval(left)?;
                let right_val = self.eval(right)?;
                self.eval_binary_op(op, left_val, right_val)
            }
            Eval::UnaryOp(op, expr) => {
                let val = self.eval(expr)?;
                self.eval_unary_op(op, val)
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
            x => Err(ControlFlow::Error(anyhow!("eval not implemented for: {:?}", eval)))
        }
    }

    fn eval_binary_op(&self, op: &super::core::BinaryOp, left: Value, right: Value) -> EvalResult {
        use super::core::BinaryOp;
        use crate::core::Value;

        match (op, &left, &right) {
            // Arithmetic operations
            (BinaryOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (BinaryOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (BinaryOp::Add, Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (BinaryOp::Add, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + *b as f64)),
            (BinaryOp::Add, Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),

            (BinaryOp::Sub, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (BinaryOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (BinaryOp::Sub, Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (BinaryOp::Sub, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - *b as f64)),

            (BinaryOp::Mul, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (BinaryOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (BinaryOp::Mul, Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (BinaryOp::Mul, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * *b as f64)),

            (BinaryOp::Div, Value::Int(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(ControlFlow::Error(anyhow!("Division by zero")))
                } else {
                    Ok(Value::Float(*a as f64 / *b as f64))
                }
            }
            (BinaryOp::Div, Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(ControlFlow::Error(anyhow!("Division by zero")))
                } else {
                    Ok(Value::Float(a / b))
                }
            }
            (BinaryOp::Div, Value::Int(a), Value::Float(b)) => {
                if *b == 0.0 {
                    Err(ControlFlow::Error(anyhow!("Division by zero")))
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            }
            (BinaryOp::Div, Value::Float(a), Value::Int(b)) => {
                if *b == 0 {
                    Err(ControlFlow::Error(anyhow!("Division by zero")))
                } else {
                    Ok(Value::Float(a / *b as f64))
                }
            }

            // Comparison operations
            (BinaryOp::Eq, a, b) => Ok(Value::Bool(a == b)),
            (BinaryOp::Ne, a, b) => Ok(Value::Bool(a != b)),

            (BinaryOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (BinaryOp::Lt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (BinaryOp::Lt, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) < *b)),
            (BinaryOp::Lt, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a < (*b as f64))),

            (BinaryOp::Le, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (BinaryOp::Le, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (BinaryOp::Le, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) <= *b)),
            (BinaryOp::Le, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a <= (*b as f64))),

            (BinaryOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (BinaryOp::Gt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (BinaryOp::Gt, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) > *b)),
            (BinaryOp::Gt, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a > (*b as f64))),

            (BinaryOp::Ge, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (BinaryOp::Ge, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (BinaryOp::Ge, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((*a as f64) >= *b)),
            (BinaryOp::Ge, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(*a >= (*b as f64))),

            // Logical operations  
            (BinaryOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a && *b)),
            (BinaryOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(*a || *b)),

            // Unsupported operation
            _ => Err(ControlFlow::Error(anyhow!("Unsupported binary operation: {:?} between {:?} and {:?}", op, left, right)))
        }
    }

    fn eval_unary_op(&self, op: &super::core::UnaryOp, val: Value) -> EvalResult {
        use super::core::UnaryOp;
        
        match (op, &val) {
            // Arithmetic negation
            (UnaryOp::Neg, Value::Int(n)) => Ok(Value::Int(-n)),
            (UnaryOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
            
            // Logical negation
            (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
            (UnaryOp::Not, Value::Unit) => Ok(Value::Bool(true)),
            (UnaryOp::Not, _) => Ok(Value::Bool(false)), // All other values are truthy
            
            // Bitwise negation (only for integers)
            (UnaryOp::BitNot, Value::Int(n)) => Ok(Value::Int(!n)),
            
            // Unsupported operation
            _ => Err(ControlFlow::Error(anyhow!("Unsupported unary operation: {:?} on {:?}", op, val)))
        }
    }
}
