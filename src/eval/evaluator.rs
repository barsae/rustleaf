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
}
