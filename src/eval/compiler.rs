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
                            AssignOp::AddAssign => {
                                let get_method = Eval::GetAttr(Box::new(Eval::Variable(name.clone())), "op_add".to_string());
                                Eval::Call(Box::new(get_method), vec![compiled_value])
                            },
                            AssignOp::SubAssign => {
                                let get_method = Eval::GetAttr(Box::new(Eval::Variable(name.clone())), "op_sub".to_string());
                                Eval::Call(Box::new(get_method), vec![compiled_value])
                            },
                            AssignOp::MulAssign => {
                                let get_method = Eval::GetAttr(Box::new(Eval::Variable(name.clone())), "op_mul".to_string());
                                Eval::Call(Box::new(get_method), vec![compiled_value])
                            },
                            AssignOp::DivAssign => {
                                let get_method = Eval::GetAttr(Box::new(Eval::Variable(name.clone())), "op_div".to_string());
                                Eval::Call(Box::new(get_method), vec![compiled_value])
                            },
                            AssignOp::ModAssign => {
                                let get_method = Eval::GetAttr(Box::new(Eval::Variable(name.clone())), "op_mod".to_string());
                                Eval::Call(Box::new(get_method), vec![compiled_value])
                            },
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
            // Binary operators - most become method calls
            Expression::Add(left, right) => self.compile_method_call_op(*left, *right, "op_add"),
            Expression::Sub(left, right) => self.compile_method_call_op(*left, *right, "op_sub"),
            Expression::Mul(left, right) => self.compile_method_call_op(*left, *right, "op_mul"),
            Expression::Div(left, right) => self.compile_method_call_op(*left, *right, "op_div"),
            Expression::Mod(left, right) => self.compile_method_call_op(*left, *right, "op_mod"),
            Expression::Pow(left, right) => self.compile_method_call_op(*left, *right, "op_pow"),
            Expression::Eq(left, right) => self.compile_method_call_op(*left, *right, "op_eq"),
            Expression::Ne(left, right) => self.compile_method_call_op(*left, *right, "op_ne"),
            Expression::Lt(left, right) => self.compile_method_call_op(*left, *right, "op_lt"),
            Expression::Le(left, right) => self.compile_method_call_op(*left, *right, "op_le"),
            Expression::Gt(left, right) => self.compile_method_call_op(*left, *right, "op_gt"),
            Expression::Ge(left, right) => self.compile_method_call_op(*left, *right, "op_ge"),
            Expression::BitAnd(left, right) => self.compile_method_call_op(*left, *right, "op_bitwise_and"),
            Expression::BitOr(left, right) => self.compile_method_call_op(*left, *right, "op_bitwise_or"),
            Expression::BitXor(left, right) => self.compile_method_call_op(*left, *right, "op_bitwise_xor"),
            Expression::LeftShift(left, right) => self.compile_method_call_op(*left, *right, "op_lshift"),
            Expression::RightShift(left, right) => self.compile_method_call_op(*left, *right, "op_rshift"),
            
            // Special cases that remain built-in
            Expression::And(left, right) => {
                let compiled_left = self.compile_expression(*left)?;
                let compiled_right = self.compile_expression(*right)?;
                Ok(Eval::LogicalAnd(Box::new(compiled_left), Box::new(compiled_right)))
            }
            Expression::Or(left, right) => {
                let compiled_left = self.compile_expression(*left)?;
                let compiled_right = self.compile_expression(*right)?;
                Ok(Eval::LogicalOr(Box::new(compiled_left), Box::new(compiled_right)))
            }
            Expression::Is(left, right) => {
                let compiled_left = self.compile_expression(*left)?;
                let compiled_right = self.compile_expression(*right)?;
                Ok(Eval::Is(Box::new(compiled_left), Box::new(compiled_right)))
            }
            Expression::In(left, right) => self.compile_method_call_op_swapped(*left, *right, "op_contains"),
            Expression::NotIn(left, right) => self.compile_not_in(*left, *right),
            Expression::IsNot(left, right) => self.compile_is_not(*left, *right),
            
            // Unary operators - most become method calls
            Expression::Neg(expr) => self.compile_unary_method_call(*expr, "op_neg"),
            Expression::BitNot(expr) => self.compile_unary_method_call(*expr, "op_bitwise_not"),
            
            // Special case that remains built-in 
            Expression::Not(expr) => {
                let compiled_expr = self.compile_expression(*expr)?;
                Ok(Eval::LogicalNot(Box::new(compiled_expr)))
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
            Expression::While { condition, body } => {
                let compiled_condition = self.compile_expression(*condition)?;
                let compiled_body = self.compile_block_helper(body)?;
                Ok(Eval::While(Box::new(compiled_condition), Box::new(compiled_body)))
            }
            Expression::For { pattern, iter, body } => {
                // For now, only support simple variable patterns
                match pattern {
                    Pattern::Variable(var_name) => {
                        let compiled_iter = self.compile_expression(*iter)?;
                        let compiled_body = self.compile_block_helper(body)?;
                        Ok(Eval::For(var_name, Box::new(compiled_iter), Box::new(compiled_body)))
                    }
                    _ => Err(anyhow::anyhow!("Only variable patterns are supported in for loops for now: {:?}", pattern)),
                }
            }
            Expression::InterpolatedString(parts) => {
                self.compile_interpolated_string(parts)
            }
            
            // Range expressions
            Expression::RangeExclusive(start, end) => {
                let start_val = self.compile_expression(*start)?;
                let end_val = self.compile_expression(*end)?;
                self.compile_range(start_val, end_val, false)
            }
            Expression::RangeInclusive(start, end) => {
                let start_val = self.compile_expression(*start)?;
                let end_val = self.compile_expression(*end)?;
                self.compile_range(start_val, end_val, true)
            }
            
            // Lambda expressions
            Expression::Lambda { params, body } => {
                let compiled_body = match body {
                    LambdaBody::Expression(expr) => {
                        self.compile_expression(*expr)?
                    }
                    LambdaBody::Block(block) => {
                        self.compile_block_helper(block)?
                    }
                };
                Ok(Eval::Lambda(params, Box::new(compiled_body)))
            }
            
            _ => Err(anyhow::anyhow!("Expression not yet implemented: {:?}", expr)),
        }
    }

    // Helper to compile binary operations to method calls: a + b => a.op_get_attr("op_add").op_call(b)
    fn compile_method_call_op(&mut self, left: Expression, right: Expression, method_name: &str) -> Result<Eval> {
        let compiled_left = self.compile_expression(left)?;
        let compiled_right = self.compile_expression(right)?;
        
        let get_method = Eval::GetAttr(Box::new(compiled_left), method_name.to_string());
        let call_method = Eval::Call(Box::new(get_method), vec![compiled_right]);
        
        Ok(call_method)
    }
    
    // Helper for operators like 'in' where operands are swapped: "item in container" => container.op_contains(item)
    fn compile_method_call_op_swapped(&mut self, left: Expression, right: Expression, method_name: &str) -> Result<Eval> {
        let compiled_left = self.compile_expression(left)?;
        let compiled_right = self.compile_expression(right)?;
        
        let get_method = Eval::GetAttr(Box::new(compiled_right), method_name.to_string());
        let call_method = Eval::Call(Box::new(get_method), vec![compiled_left]);
        
        Ok(call_method)
    }
    
    // Helper to compile unary operations to method calls: -a => a.op_get_attr("op_neg").op_call()
    fn compile_unary_method_call(&mut self, expr: Expression, method_name: &str) -> Result<Eval> {
        let compiled_expr = self.compile_expression(expr)?;
        
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

    // Helper to compile "not in" as !(left in right)
    fn compile_not_in(&mut self, left: Expression, right: Expression) -> Result<Eval> {
        let in_expr = self.compile_method_call_op_swapped(left, right, "op_contains")?;
        Ok(Eval::LogicalNot(Box::new(in_expr)))
    }

    // Helper to compile "is not" as !(left is right)  
    fn compile_is_not(&mut self, left: Expression, right: Expression) -> Result<Eval> {
        let compiled_left = self.compile_expression(left)?;
        let compiled_right = self.compile_expression(right)?;
        let is_expr = Eval::Is(Box::new(compiled_left), Box::new(compiled_right));
        Ok(Eval::LogicalNot(Box::new(is_expr)))
    }

    // Helper to compile interpolated strings as string concatenation
    // "Hello ${name}" becomes "Hello " + name
    // "${a} and ${b}" becomes a + " and " + b
    fn compile_interpolated_string(&mut self, parts: Vec<InterpolationPart>) -> Result<Eval> {
        if parts.is_empty() {
            return Ok(Eval::Literal(Value::String(String::new())));
        }

        // Convert each part to an Eval expression
        let mut compiled_parts = Vec::new();
        for part in parts {
            match part {
                InterpolationPart::Text(text) => {
                    compiled_parts.push(Eval::Literal(Value::String(text)));
                }
                InterpolationPart::Expression(expr) => {
                    // Wrap non-string expressions in str() conversion
                    let compiled_expr = self.compile_expression(expr)?;
                    let str_call = Eval::Call(
                        Box::new(Eval::Variable("str".to_string())),
                        vec![compiled_expr]
                    );
                    compiled_parts.push(str_call);
                }
            }
        }

        // If there's only one part, return it directly
        if compiled_parts.len() == 1 {
            return Ok(compiled_parts.into_iter().next().unwrap());
        }

        // Build a chain of additions: a + b + c + ...
        let mut parts_iter = compiled_parts.into_iter();
        let mut result = parts_iter.next().unwrap();
        for part in parts_iter {
            // Use the same pattern as Expression::Add: left.op_add(right)
            let get_method = Eval::GetAttr(Box::new(result), "op_add".to_string());
            result = Eval::Call(Box::new(get_method), vec![part]);
        }

        Ok(result)
    }

    // Helper to compile range expressions - for now, only support integer literals
    fn compile_range(&mut self, start_eval: Eval, end_eval: Eval, inclusive: bool) -> Result<Eval> {
        // For now, require that both start and end are literal integers
        match (start_eval, end_eval) {
            (Eval::Literal(Value::Int(start)), Eval::Literal(Value::Int(end))) => {
                let range = crate::core::Range { start, end, inclusive };
                Ok(Eval::Literal(Value::Range(range)))
            }
            _ => Err(anyhow::anyhow!("Range expressions currently only support integer literals")),
        }
    }
}