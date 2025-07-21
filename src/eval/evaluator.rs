/// Evaluator implementation for RustLeaf

use std::cell::RefCell;
use std::rc::Rc;

use anyhow::Result;
use crate::core::*;
use super::scope::Scope;

/// Evaluate a program and return the final value
pub fn evaluate(program: Program) -> Result<Value> {
    let mut evaluator = Evaluator::new();
    evaluator.eval_program(program)
}

pub struct Evaluator {
    globals: Rc<RefCell<Scope>>,
    current_env: Rc<RefCell<Scope>>,
}

impl Evaluator {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Scope::new()));
        
        // TODO: Register built-in functions in globals
        // print, type, len, etc.
        
        Evaluator {
            globals: globals.clone(),
            current_env: globals,
        }
    }
    
    pub fn eval_program(&mut self, program: Program) -> Result<Value> {
        let Program(statements) = program;
        let mut last_value = Value::Unit;
        
        for stmt in statements {
            last_value = self.eval_statement(&stmt)?;
        }
        
        Ok(last_value)
    }
    
    fn eval_statement(&mut self, stmt: &Statement) -> Result<Value> {
        match stmt {
            Statement::Expression(expr) => self.eval_expression(expr),
            
            Statement::VarDecl(name, init) => {
                let value = if let Some(expr) = init {
                    self.eval_expression(expr)?
                } else {
                    Value::Null
                };
                self.current_env.borrow_mut().define(name.clone(), value);
                Ok(Value::Unit)
            }
            
            _ => Err(anyhow::anyhow!(
                "Statement evaluation not yet implemented".to_string()
            )),
        }
    }
    
    fn eval_expression(&mut self, expr: &Expression) -> Result<Value> {
        match expr {
            Expression::Literal(lit) => Ok(self.eval_literal(lit)),
            
            Expression::Identifier(name) => {
                self.current_env.borrow().get(name)
                    .ok_or_else(|| anyhow::anyhow!(format!("Undefined variable: {}", name)))
            }
            
            Expression::GetAttr(obj_expr, attr_name) => {
                let obj = self.eval_expression(obj_expr)?;
                self.get_attribute(obj, attr_name)
            }
            
            Expression::SetAttr(obj_expr, attr_name, value_expr) => {
                let obj = self.eval_expression(obj_expr)?;
                let value = self.eval_expression(value_expr)?;
                self.set_attribute(obj, attr_name, value)
            }
            
            Expression::MethodCall(method_expr, args) => {
                let method = self.eval_expression(method_expr)?;
                let arg_values: Result<Vec<Value>> = args.iter()
                    .map(|arg| self.eval_expression(arg))
                    .collect();
                let arg_values = arg_values?;
                
                self.call_method(method, arg_values)
            }
            
            _ => Err(anyhow::anyhow!(
                "Expression evaluation not yet implemented".to_string()
            )),
        }
    }
    
    fn eval_literal(&self, lit: &LiteralValue) -> Value {
        match lit {
            LiteralValue::Null => Value::Null,
            LiteralValue::Bool(b) => Value::Bool(*b),
            LiteralValue::Int(n) => Value::Int(*n),
            LiteralValue::Float(f) => Value::Float(*f),
            LiteralValue::String(s) => Value::String(s.clone()),
        }
    }
    
    fn get_attribute(&mut self, obj: Value, attr_name: &str) -> Result<Value> {
        // This is the core of the attribute resolution system
        match &obj {
            Value::Int(n) => {
                n.get_attr(attr_name)
                    .ok_or_else(|| anyhow::anyhow!(
                        format!("'int' object has no attribute '{}'", attr_name)
                    ))
            }
            
            Value::Float(f) => {
                f.get_attr(attr_name)
                    .ok_or_else(|| anyhow::anyhow!(
                        format!("'float' object has no attribute '{}'", attr_name)
                    ))
            }
            
            Value::String(s) => {
                s.get_attr(attr_name)
                    .map(|method| {
                        // For methods, return a bound method
                        if let Value::BuiltinMethod(m) = method {
                            Value::BoundMethod(Box::new(obj.clone()), m)
                        } else {
                            method
                        }
                    })
                    .ok_or_else(|| anyhow::anyhow!(
                        format!("'string' object has no attribute '{}'", attr_name)
                    ))
            }
            
            _ => Err(anyhow::anyhow!(
                format!("Attribute access not yet implemented for {:?}", obj)
            )),
        }
    }
    
    fn set_attribute(&mut self, _obj: Value, _attr_name: &str, _value: Value) -> Result<Value> {
        // TODO: Implement attribute setting
        Err(anyhow::anyhow!(
            "Attribute setting not yet implemented"
        ))
    }
    
    fn call_method(&mut self, method: Value, args: Vec<Value>) -> Result<Value> {
        match method {
            Value::BuiltinMethod(builtin) => {
                self.call_builtin_method(builtin, None, args)
            }
            
            Value::BoundMethod(self_val, builtin) => {
                self.call_builtin_method(builtin, Some(*self_val), args)
            }
            
            _ => Err(anyhow::anyhow!(
                format!("'{}' object is not callable", crate::core::type_of(&method))
            )),
        }
    }
    
    fn call_builtin_method(
        &mut self, 
        method: BuiltinMethod, 
        self_val: Option<Value>, 
        args: Vec<Value>
    ) -> Result<Value> {
        // TODO: Implement all builtin methods
        match method {
            BuiltinMethod::IntAdd => {
                if args.len() != 1 {
                    return Err(anyhow::anyhow!(
                        format!("op_add() takes exactly 1 argument ({} given)", args.len())
                    ));
                }
                
                match (self_val, &args[0]) {
                    (Some(Value::Int(a)), Value::Int(b)) => Ok(Value::Int(a + b)),
                    _ => Err(anyhow::anyhow!(
                        "unsupported operand type(s) for +".to_string()
                    )),
                }
            }
            
            _ => Err(anyhow::anyhow!(
                format!("Builtin method {:?} not yet implemented", method)
            )),
        }
    }
}