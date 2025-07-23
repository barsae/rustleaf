/// Compiler from AST to simplified evaluation IR
use anyhow::Result;

use super::core::Eval;
use crate::core::*;

pub struct Compiler;

impl Compiler {
    /// Compile an AST program to evaluation IR
    pub fn compile(ast: Program) -> Result<Eval> {
        let mut compiler = Self;
        compiler.compile_program(ast)
    }

    fn compile_program(&mut self, program: Program) -> Result<Eval> {
        let statements = program.0;
        
        if statements.is_empty() {
            return Ok(Eval::Block(vec![], None));
        }
        
        let last_index = statements.len() - 1;
        let mut eval_statements = Vec::new();
        let mut final_expr = None;
        
        for (i, stmt) in statements.into_iter().enumerate() {
            let is_last = i == last_index;
            match stmt {
                Statement::Expression(expr) if is_last => {
                    // Last statement is an expression - make it the final expression
                    final_expr = Some(Box::new(self.compile_expression(expr)?));
                }
                _ => {
                    eval_statements.push(self.compile_statement(stmt)?);
                }
            }
        }
        
        Ok(Eval::Block(eval_statements, final_expr))
    }

    fn compile_statement(&mut self, stmt: Statement) -> Result<Eval> {
        match stmt {
            Statement::Expression(expr) => self.compile_expression(expr),
            Statement::VarDecl { pattern, value } => {
                // For now, only handle simple variable patterns
                match pattern {
                    Pattern::Variable(name) => {
                        let init = match value {
                            Some(expr) => Some(Box::new(self.compile_expression(expr)?)),
                            None => None,
                        };
                        Ok(Eval::Declare(name, init))
                    }
                    _ => Err(anyhow::anyhow!("Complex patterns not yet implemented")),
                }
            }
            Statement::Assignment { target, op, value } => {
                let compiled_value = self.compile_expression(value)?;
                match target {
                    LValue::Identifier(name) => {
                        // Handle compound assignment operators
                        let final_value = match op {
                            AssignOp::Assign => compiled_value,
                            AssignOp::AddAssign => Eval::BinaryOp(
                                super::core::BinaryOp::Add,
                                Box::new(Eval::Variable(name.clone())),
                                Box::new(compiled_value),
                            ),
                            AssignOp::SubAssign => Eval::BinaryOp(
                                super::core::BinaryOp::Sub,
                                Box::new(Eval::Variable(name.clone())),
                                Box::new(compiled_value),
                            ),
                            AssignOp::MulAssign => Eval::BinaryOp(
                                super::core::BinaryOp::Mul,
                                Box::new(Eval::Variable(name.clone())),
                                Box::new(compiled_value),
                            ),
                            AssignOp::DivAssign => Eval::BinaryOp(
                                super::core::BinaryOp::Div,
                                Box::new(Eval::Variable(name.clone())),
                                Box::new(compiled_value),
                            ),
                            AssignOp::ModAssign => Eval::BinaryOp(
                                super::core::BinaryOp::Mod,
                                Box::new(Eval::Variable(name.clone())),
                                Box::new(compiled_value),
                            ),
                        };
                        Ok(Eval::Assign(name, Box::new(final_value)))
                    }
                    LValue::GetAttr(obj, attr) => {
                        let compiled_obj = self.compile_expression(*obj)?;
                        Ok(Eval::SetAttr(
                            Box::new(compiled_obj),
                            attr,
                            Box::new(compiled_value),
                        ))
                    }
                    LValue::GetItem(obj, key) => {
                        let compiled_obj = self.compile_expression(*obj)?;
                        let compiled_key = self.compile_expression(*key)?;
                        Ok(Eval::SetItem(
                            Box::new(compiled_obj),
                            Box::new(compiled_key),
                            Box::new(compiled_value),
                        ))
                    }
                }
            }
            Statement::Return(expr) => {
                let compiled_expr = match expr {
                    Some(e) => Some(Box::new(self.compile_expression(e)?)),
                    None => None,
                };
                Ok(Eval::Return(compiled_expr))
            }
            Statement::Break(expr) => {
                let compiled_expr = match expr {
                    Some(e) => Some(Box::new(self.compile_expression(e)?)),
                    None => None,
                };
                Ok(Eval::Break(compiled_expr))
            }
            Statement::Continue => Ok(Eval::Continue),
            Statement::FnDecl { name, params, body, is_pub: _ } => {
                // Extract parameter names
                let param_names: Vec<String> = params.into_iter()
                    .map(|p| p.name)
                    .collect();
                
                // Compile the function body
                let compiled_body = self.compile_block_helper(body)?;
                
                Ok(Eval::Function(name, param_names, Box::new(compiled_body)))
            }
            _ => Err(anyhow::anyhow!("Statement not yet implemented: {:?}", stmt)),
        }
    }

    fn compile_expression(&mut self, expr: Expression) -> Result<Eval> {
        match expr {
            Expression::Literal(lit) => {
                let value = self.compile_literal(lit);
                Ok(Eval::Literal(value))
            }
            Expression::Identifier(name) => Ok(Eval::Variable(name)),
            Expression::GetAttr(obj, attr) => {
                let compiled_obj = self.compile_expression(*obj)?;
                Ok(Eval::GetAttr(Box::new(compiled_obj), attr))
            }
            Expression::GetItem(obj, key) => {
                let compiled_obj = self.compile_expression(*obj)?;
                let compiled_key = self.compile_expression(*key)?;
                Ok(Eval::GetItem(Box::new(compiled_obj), Box::new(compiled_key)))
            }
            Expression::FunctionCall(func, args) => {
                let compiled_func = self.compile_expression(*func)?;
                let compiled_args: Result<Vec<Eval>> = args
                    .into_iter()
                    .map(|arg| self.compile_expression(arg))
                    .collect();
                Ok(Eval::Call(Box::new(compiled_func), compiled_args?))
            }
            Expression::MethodCall(obj, method, args) => {
                // Desugar method call to function call
                let compiled_obj = self.compile_expression(*obj)?;
                let method_expr = Eval::GetAttr(Box::new(compiled_obj), method);
                let compiled_args: Result<Vec<Eval>> = args
                    .into_iter()
                    .map(|arg| self.compile_expression(arg))
                    .collect();
                Ok(Eval::Call(Box::new(method_expr), compiled_args?))
            }
            Expression::List(elements) => {
                let compiled_elements: Result<Vec<Eval>> = elements
                    .into_iter()
                    .map(|elem| self.compile_expression(elem))
                    .collect();
                Ok(Eval::List(compiled_elements?))
            }
            Expression::Dict(pairs) => {
                let compiled_pairs: Result<Vec<(Eval, Eval)>> = pairs
                    .into_iter()
                    .map(|(k, v)| {
                        Ok((self.compile_expression(k)?, self.compile_expression(v)?))
                    })
                    .collect();
                Ok(Eval::Dict(compiled_pairs?))
            }
            Expression::Block(block) => {
                let mut eval_statements = Vec::new();
                
                // Compile all statements
                for stmt in block.statements {
                    eval_statements.push(self.compile_statement(stmt)?);
                }
                
                // Handle final expression
                match block.final_expr {
                    Some(expr) => {
                        let compiled_final_expr = self.compile_expression(*expr)?;
                        Ok(Eval::Block(eval_statements, Some(Box::new(compiled_final_expr))))
                    }
                    None => Ok(Eval::Block(eval_statements, None)),
                }
            }
            // Binary operators
            Expression::Add(left, right) => self.compile_binary_op(super::core::BinaryOp::Add, *left, *right),
            Expression::Sub(left, right) => self.compile_binary_op(super::core::BinaryOp::Sub, *left, *right),
            Expression::Mul(left, right) => self.compile_binary_op(super::core::BinaryOp::Mul, *left, *right),
            Expression::Div(left, right) => self.compile_binary_op(super::core::BinaryOp::Div, *left, *right),
            Expression::Mod(left, right) => self.compile_binary_op(super::core::BinaryOp::Mod, *left, *right),
            Expression::Pow(left, right) => self.compile_binary_op(super::core::BinaryOp::Pow, *left, *right),
            Expression::Eq(left, right) => self.compile_binary_op(super::core::BinaryOp::Eq, *left, *right),
            Expression::Ne(left, right) => self.compile_binary_op(super::core::BinaryOp::Ne, *left, *right),
            Expression::Lt(left, right) => self.compile_binary_op(super::core::BinaryOp::Lt, *left, *right),
            Expression::Le(left, right) => self.compile_binary_op(super::core::BinaryOp::Le, *left, *right),
            Expression::Gt(left, right) => self.compile_binary_op(super::core::BinaryOp::Gt, *left, *right),
            Expression::Ge(left, right) => self.compile_binary_op(super::core::BinaryOp::Ge, *left, *right),
            Expression::And(left, right) => self.compile_binary_op(super::core::BinaryOp::And, *left, *right),
            Expression::Or(left, right) => self.compile_binary_op(super::core::BinaryOp::Or, *left, *right),
            Expression::In(left, right) => self.compile_binary_op(super::core::BinaryOp::In, *left, *right),
            Expression::Is(left, right) => self.compile_binary_op(super::core::BinaryOp::Is, *left, *right),
            Expression::BitAnd(left, right) => self.compile_binary_op(super::core::BinaryOp::BitAnd, *left, *right),
            Expression::BitOr(left, right) => self.compile_binary_op(super::core::BinaryOp::BitOr, *left, *right),
            Expression::BitXor(left, right) => self.compile_binary_op(super::core::BinaryOp::BitXor, *left, *right),
            Expression::LeftShift(left, right) => self.compile_binary_op(super::core::BinaryOp::LeftShift, *left, *right),
            Expression::RightShift(left, right) => self.compile_binary_op(super::core::BinaryOp::RightShift, *left, *right),
            
            // Unary operators
            Expression::Neg(expr) => {
                self.compile_unary_op(super::core::UnaryOp::Neg, *expr)
            }
            Expression::Not(expr) => {
                self.compile_unary_op(super::core::UnaryOp::Not, *expr)
            }
            Expression::BitNot(expr) => {
                self.compile_unary_op(super::core::UnaryOp::BitNot, *expr)
            }
            Expression::If { condition, then_expr, else_expr } => {
                let compiled_condition = self.compile_expression(*condition)?;
                let compiled_then = self.compile_block_helper(then_expr)?;
                let compiled_else = match else_expr {
                    Some(block) => Some(Box::new(self.compile_block_helper(block)?)),
                    None => None,
                };
                Ok(Eval::If(Box::new(compiled_condition), Box::new(compiled_then), compiled_else))
            }
            Expression::Loop { body } => {
                let compiled_body = self.compile_block_helper(body)?;
                Ok(Eval::Loop(Box::new(compiled_body)))
            }
            
            _ => Err(anyhow::anyhow!("Expression not yet implemented: {:?}", expr)),
        }
    }

    fn compile_binary_op(&mut self, op: super::core::BinaryOp, left: Expression, right: Expression) -> Result<Eval> {
        let compiled_left = self.compile_expression(left)?;
        let compiled_right = self.compile_expression(right)?;
        
        // Desugar binary operations to method calls: a + b => a.op_get_attr("op_add").op_call(b)
        let method_name = match op {
            super::core::BinaryOp::Add => "op_add",
            super::core::BinaryOp::Sub => "op_sub", 
            super::core::BinaryOp::Mul => "op_mul",
            super::core::BinaryOp::Div => "op_div",
            super::core::BinaryOp::Mod => "op_mod",
            super::core::BinaryOp::Pow => "op_pow",
            super::core::BinaryOp::Eq => "op_eq",
            super::core::BinaryOp::Ne => "op_ne",
            super::core::BinaryOp::Lt => "op_lt",
            super::core::BinaryOp::Le => "op_le",
            super::core::BinaryOp::Gt => "op_gt",
            super::core::BinaryOp::Ge => "op_ge",
            super::core::BinaryOp::BitAnd => "op_bitwise_and",
            super::core::BinaryOp::BitOr => "op_bitwise_or",
            super::core::BinaryOp::BitXor => "op_bitwise_xor",
            super::core::BinaryOp::LeftShift => "op_lshift",
            super::core::BinaryOp::RightShift => "op_rshift",
            super::core::BinaryOp::In => "op_contains", // Note: "item in container" => container.op_contains(item)
            super::core::BinaryOp::Is => return Ok(Eval::BinaryOp(op, Box::new(compiled_left), Box::new(compiled_right))), // Keep 'is' as built-in
            // Logical operations remain built-in for short-circuit evaluation
            super::core::BinaryOp::And | super::core::BinaryOp::Or => {
                return Ok(Eval::BinaryOp(op, Box::new(compiled_left), Box::new(compiled_right)));
            }
        };
        
        // Special handling for 'in' operator: "item in container" => container.op_contains(item)
        let (obj_expr, arg_expr) = if matches!(op, super::core::BinaryOp::In) {
            (compiled_right, compiled_left) // Swap operands for 'in'
        } else {
            (compiled_left, compiled_right)
        };
        
        // Create: obj.op_get_attr("method_name").op_call(arg)
        let get_method = Eval::GetAttr(Box::new(obj_expr), method_name.to_string());  
        let call_method = Eval::Call(Box::new(get_method), vec![arg_expr]);
        
        Ok(call_method)
    }

    fn compile_unary_op(&mut self, op: super::core::UnaryOp, expr: Expression) -> Result<Eval> {
        let compiled_expr = self.compile_expression(expr)?;
        
        // Desugar unary operations to method calls: -a => a.op_get_attr("op_neg").op_call()
        let method_name = match op {
            super::core::UnaryOp::Neg => "op_neg",
            super::core::UnaryOp::BitNot => "op_bitwise_not",
            // 'not' remains built-in for truthiness handling
            super::core::UnaryOp::Not => return Ok(Eval::UnaryOp(op, Box::new(compiled_expr))),
        };
        
        // Create: expr.op_get_attr("method_name").op_call()
        let get_method = Eval::GetAttr(Box::new(compiled_expr), method_name.to_string());
        let call_method = Eval::Call(Box::new(get_method), vec![]);
        
        Ok(call_method)
    }

    fn compile_literal(&self, lit: LiteralValue) -> Value {
        match lit {
            LiteralValue::Null => Value::Null,
            LiteralValue::Bool(b) => Value::Bool(b),
            LiteralValue::Int(i) => Value::Int(i),
            LiteralValue::Float(f) => Value::Float(f),
            LiteralValue::String(s) | LiteralValue::RawString(s) => Value::String(s),
        }
    }

    fn compile_block_helper(&mut self, block: crate::core::Block) -> Result<Eval> {
        let mut eval_statements = Vec::new();
        
        // Compile all statements
        for stmt in block.statements {
            eval_statements.push(self.compile_statement(stmt)?);
        }
        
        // Handle final expression
        match block.final_expr {
            Some(expr) => {
                let compiled_final_expr = self.compile_expression(*expr)?;
                Ok(Eval::Block(eval_statements, Some(Box::new(compiled_final_expr))))
            }
            None => Ok(Eval::Block(eval_statements, None)),
        }
    }
}