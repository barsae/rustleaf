/// Compiler from AST to simplified evaluation IR
use anyhow::Result;

use super::constructors::Eval;
use super::structs::ClassMethod;
use crate::core::*;

pub struct Compiler;

impl Compiler {
    /// Compile an AST program to evaluation IR
    pub fn compile(ast: Program) -> Result<RustValueRef> {
        let mut compiler = Self;
        compiler.compile_program(ast)
    }

    fn compile_program(&mut self, program: Program) -> Result<RustValueRef> {
        let statements = program.0;

        if statements.is_empty() {
            return Ok(Eval::program(vec![]));
        }

        let eval_statements: Result<Vec<_>> = statements
            .into_iter()
            .map(|stmt| self.compile_statement(stmt))
            .collect();

        Ok(Eval::program(eval_statements?))
    }

    fn compile_statement(&mut self, stmt: Statement) -> Result<RustValueRef> {
        match stmt {
            Statement::Macro {
                name,
                args: _,
                statement,
            } => {
                // Compile the wrapped statement normally first
                let target_eval = self.compile_statement(*statement)?;

                // Create an Eval::Macro node that will call the macro function
                Ok(Eval::macro_expr(super::structs::MacroData {
                    macro_fn: Eval::variable(name),
                    target: target_eval,
                    args: Vec::new(), // For now, ignore macro arguments
                }))
            }
            Statement::Expression(expr) => self.compile_expression(expr),
            Statement::VarDecl { pattern, value } => {
                match pattern {
                    Pattern::Variable(name) => {
                        // Simple variable declaration
                        let init = match value {
                            Some(expr) => Some(Box::new(self.compile_expression(expr)?)),
                            None => None,
                        };
                        Ok(Eval::declare(name, init.map(|boxed| *boxed)))
                    }
                    _ => {
                        // Pattern-based declaration
                        let eval_pattern = Self::compile_pattern(pattern)?;
                        let init_expr = match value {
                            Some(expr) => self.compile_expression(expr)?,
                            None => {
                                return Err(anyhow::anyhow!(
                                    "Pattern declarations require an initializer"
                                ))
                            }
                        };
                        Ok(Eval::declare_pattern(eval_pattern, init_expr))
                    }
                }
            }
            Statement::Assignment { target, op, value } => {
                let compiled_value = self.compile_expression(value)?;
                match target {
                    LValue::Identifier(name) => {
                        // Handle compound assignment operators
                        let target_eval = Eval::variable(name.clone());
                        let final_value =
                            self.compile_compound_assignment(target_eval, op, compiled_value)?;
                        Ok(Eval::assign(name, final_value))
                    }
                    LValue::GetAttr(obj, attr) => {
                        let compiled_obj = self.compile_expression(*obj)?;

                        // Handle compound assignment operators for attributes
                        let target_eval = Eval::get_attr(compiled_obj.clone(), attr.clone());
                        let final_value =
                            self.compile_compound_assignment(target_eval, op, compiled_value)?;

                        Ok(Eval::set_attr(compiled_obj, attr, final_value))
                    }
                    LValue::GetItem(obj, key) => {
                        let compiled_obj = self.compile_expression(*obj)?;
                        let compiled_key = self.compile_expression(*key)?;
                        Ok(Eval::set_item(compiled_obj, compiled_key, compiled_value))
                    }
                }
            }
            Statement::Return(expr) => {
                let compiled_expr = match expr {
                    Some(e) => Some(Box::new(self.compile_expression(e)?)),
                    None => None,
                };
                Ok(Eval::return_expr(compiled_expr.map(|boxed| *boxed)))
            }
            Statement::Break(expr) => {
                let compiled_expr = match expr {
                    Some(e) => Some(Box::new(self.compile_expression(e)?)),
                    None => None,
                };
                Ok(Eval::break_expr(compiled_expr.map(|boxed| *boxed)))
            }
            Statement::Continue => Ok(Eval::continue_expr()),
            Statement::FnDecl {
                name,
                params,
                body,
                is_pub: _,
            } => {
                // Extract parameter names, compile default values, and preserve kinds
                let param_data: Vec<(String, Option<Value>, ParameterKind)> = params
                    .into_iter()
                    .map(|p| {
                        let default_value = p.default.map(|lit| self.compile_literal(lit));
                        (p.name, default_value, p.kind)
                    })
                    .collect();

                // Compile the function body
                let compiled_body = self.compile_block_helper(body)?;

                Ok(Eval::function(super::structs::FunctionData {
                    name,
                    params: param_data,
                    body: compiled_body,
                }))
            }
            Statement::ClassDecl {
                name,
                members,
                is_pub: _,
            } => self.compile_class_decl(name, members),
            Statement::Import(import_spec) => Ok(Eval::import(super::structs::ImportData {
                module: import_spec.module,
                items: import_spec.items,
            })),
        }
    }

    fn compile_binary_op(
        &mut self,
        left: Expression,
        right: Expression,
        method_name: &str,
    ) -> Result<RustValueRef> {
        let left_eval = Box::new(self.compile_expression(left)?);
        let right_eval = Box::new(self.compile_expression(right)?);
        let get_method = Eval::get_attr(*left_eval, method_name.to_string());
        Ok(Eval::call(get_method, vec![*right_eval]))
    }

    fn compile_compound_assignment(
        &mut self,
        target_eval: RustValueRef,
        op: AssignOp,
        value_eval: RustValueRef,
    ) -> Result<RustValueRef> {
        match op {
            AssignOp::Assign => Ok(value_eval),
            _ => {
                let method_name = op.to_method_name();
                let get_method = Eval::get_attr(target_eval, method_name.to_string());
                Ok(Eval::call(get_method, vec![value_eval]))
            }
        }
    }

    fn compile_expression(&mut self, expr: Expression) -> Result<RustValueRef> {
        match expr {
            Expression::Literal(lit) => {
                let value = self.compile_literal(lit);
                Ok(Eval::literal(value))
            }
            Expression::Identifier(name) => Ok(Eval::variable(name)),
            Expression::GetAttr(obj, attr) => {
                let compiled_obj = self.compile_expression(*obj)?;
                Ok(Eval::get_attr(compiled_obj, attr))
            }
            Expression::GetItem(obj, key) => {
                let compiled_obj = self.compile_expression(*obj)?;
                let compiled_key = self.compile_expression(*key)?;
                Ok(Eval::get_item(compiled_obj, compiled_key))
            }
            Expression::FunctionCall(func, args) => {
                let compiled_func = self.compile_expression(*func)?;
                let compiled_args: Result<Vec<RustValueRef>> = args
                    .into_iter()
                    .map(|arg| self.compile_expression(arg))
                    .collect();
                Ok(Eval::call(compiled_func, compiled_args?))
            }
            Expression::MethodCall(obj, method, args) => {
                // Desugar method call to function call
                let compiled_obj = self.compile_expression(*obj)?;
                let method_expr = Eval::get_attr(compiled_obj, method);
                let compiled_args: Result<Vec<RustValueRef>> = args
                    .into_iter()
                    .map(|arg| self.compile_expression(arg))
                    .collect();
                Ok(Eval::call(method_expr, compiled_args?))
            }
            Expression::List(elements) => {
                let compiled_elements: Result<Vec<RustValueRef>> = elements
                    .into_iter()
                    .map(|elem| self.compile_expression(elem))
                    .collect();
                Ok(Eval::list(compiled_elements?))
            }
            Expression::Dict(pairs) => {
                let compiled_pairs: Result<Vec<(RustValueRef, RustValueRef)>> = pairs
                    .into_iter()
                    .map(|(k, v)| Ok((self.compile_expression(k)?, self.compile_expression(v)?)))
                    .collect();
                Ok(Eval::dict(compiled_pairs?))
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
                        Ok(Eval::block(eval_statements, Some(compiled_final_expr)))
                    }
                    None => Ok(Eval::block(eval_statements, None)),
                }
            }
            // Binary operators - most become method calls
            Expression::Add(left, right) => self.compile_binary_op(*left, *right, "op_add"),
            Expression::Sub(left, right) => self.compile_binary_op(*left, *right, "op_sub"),
            Expression::Mul(left, right) => self.compile_binary_op(*left, *right, "op_mul"),
            Expression::Div(left, right) => self.compile_binary_op(*left, *right, "op_div"),
            Expression::Mod(left, right) => self.compile_binary_op(*left, *right, "op_mod"),
            Expression::Pow(left, right) => self.compile_binary_op(*left, *right, "op_pow"),
            Expression::Eq(left, right) => self.compile_binary_op(*left, *right, "op_eq"),
            Expression::Ne(left, right) => self.compile_binary_op(*left, *right, "op_ne"),
            Expression::Lt(left, right) => self.compile_binary_op(*left, *right, "op_lt"),
            Expression::Le(left, right) => self.compile_binary_op(*left, *right, "op_le"),
            Expression::Gt(left, right) => self.compile_binary_op(*left, *right, "op_gt"),
            Expression::Ge(left, right) => self.compile_binary_op(*left, *right, "op_ge"),
            Expression::BitAnd(left, right) => {
                self.compile_binary_op(*left, *right, "op_bitwise_and")
            }
            Expression::BitOr(left, right) => {
                self.compile_binary_op(*left, *right, "op_bitwise_or")
            }
            Expression::BitXor(left, right) => {
                self.compile_binary_op(*left, *right, "op_bitwise_xor")
            }
            Expression::LeftShift(left, right) => {
                self.compile_binary_op(*left, *right, "op_lshift")
            }
            Expression::RightShift(left, right) => {
                self.compile_binary_op(*left, *right, "op_rshift")
            }

            // Special cases that remain built-in
            Expression::And(left, right) => {
                let compiled_left = self.compile_expression(*left)?;
                let compiled_right = self.compile_expression(*right)?;
                Ok(Eval::logical_and(compiled_left, compiled_right))
            }
            Expression::Or(left, right) => {
                let compiled_left = self.compile_expression(*left)?;
                let compiled_right = self.compile_expression(*right)?;
                Ok(Eval::logical_or(compiled_left, compiled_right))
            }
            Expression::Is(left, right) => {
                let compiled_left = self.compile_expression(*left)?;
                let compiled_right = self.compile_expression(*right)?;
                Ok(Eval::is(compiled_left, compiled_right))
            }
            Expression::In(left, right) => {
                // For 'in' operator, we need to swap arguments: item in container => container.op_contains(item)
                self.compile_binary_op(*right, *left, "op_contains")
            }
            Expression::NotIn(left, right) => self.compile_not_in(*left, *right),
            Expression::IsNot(left, right) => self.compile_is_not(*left, *right),

            // Unary operators - most become method calls
            Expression::Neg(expr) => self.compile_unary_method_call(*expr, "op_neg"),
            Expression::BitNot(expr) => self.compile_unary_method_call(*expr, "op_bitwise_not"),

            // Special case that remains built-in
            Expression::Not(expr) => {
                let compiled_expr = self.compile_expression(*expr)?;
                Ok(Eval::logical_not(compiled_expr))
            }
            Expression::If {
                condition,
                then_expr,
                else_expr,
            } => {
                let compiled_condition = self.compile_expression(*condition)?;
                let compiled_then = self.compile_block_helper(then_expr)?;
                let compiled_else = match else_expr {
                    Some(block) => Some(Box::new(self.compile_block_helper(block)?)),
                    None => None,
                };
                Ok(Eval::if_expr(
                    compiled_condition,
                    compiled_then,
                    compiled_else.map(|boxed| *boxed),
                ))
            }
            Expression::Loop { body } => {
                let compiled_body = self.compile_block_helper(body)?;
                Ok(Eval::loop_expr(compiled_body))
            }
            Expression::While { condition, body } => {
                let compiled_condition = self.compile_expression(*condition)?;
                let compiled_body = self.compile_block_helper(body)?;
                Ok(Eval::while_expr(compiled_condition, compiled_body))
            }
            Expression::For {
                pattern,
                iter,
                body,
            } => {
                // For now, only support simple variable patterns
                match pattern {
                    Pattern::Variable(var_name) => {
                        let compiled_iter = self.compile_expression(*iter)?;
                        let compiled_body = self.compile_block_helper(body)?;
                        Ok(Eval::for_expr(var_name, compiled_iter, compiled_body))
                    }
                    _ => Err(anyhow::anyhow!(
                        "Only variable patterns are supported in for loops for now: {:?}",
                        pattern
                    )),
                }
            }
            Expression::InterpolatedString(parts) => self.compile_interpolated_string(parts),

            // Range expressions
            Expression::RangeExclusive(start, end) => {
                self.compile_range_expression(*start, *end, false)
            }
            Expression::RangeInclusive(start, end) => {
                self.compile_range_expression(*start, *end, true)
            }

            // Lambda expressions
            Expression::Lambda { params, body } => {
                let compiled_body = match body {
                    LambdaBody::Expression(expr) => self.compile_expression(*expr)?,
                    LambdaBody::Block(block) => self.compile_block_helper(block)?,
                };
                Ok(Eval::lambda(super::structs::LambdaData {
                    params,
                    body: compiled_body,
                }))
            }

            Expression::Try { body, catch } => {
                let compiled_body = self.compile_block_helper(body)?;
                let compiled_catch_body = self.compile_block_helper(catch.body)?;
                let catch_pattern = Self::compile_pattern(catch.pattern)?;

                Ok(Eval::try_expr(
                    compiled_body,
                    catch_pattern,
                    compiled_catch_body,
                ))
            }

            Expression::With { resources, body } => {
                // Compile resources as (name, value) pairs
                let mut compiled_resources = Vec::new();
                for resource in resources {
                    let compiled_value = self.compile_expression(resource.value)?;
                    compiled_resources.push((resource.name, compiled_value));
                }

                let compiled_body = self.compile_block_helper(body)?;
                Ok(Eval::with_expr(super::structs::WithData {
                    resources: compiled_resources,
                    body: compiled_body,
                }))
            }

            // Pipe operator: expr1 : expr2
            // According to spec, should create partial application: |*args, **kwargs| expr2(expr1, *args, **kwargs)
            // For now, implement simpler version for the test case: 1 : test(2) => test(1, 2)
            Expression::Pipe(left, right) => {
                let compiled_left = self.compile_expression(*left)?;

                // Handle different right-hand side patterns
                match *right {
                    // Simple function call: left : func(args) => func(left, args)
                    Expression::FunctionCall(func, args) => {
                        let compiled_func = self.compile_expression(*func)?;
                        let mut compiled_args: Vec<RustValueRef> = vec![compiled_left]; // Insert left as first argument

                        // Add the existing arguments
                        for arg in args {
                            compiled_args.push(self.compile_expression(arg)?);
                        }

                        Ok(Eval::call(compiled_func, compiled_args))
                    }
                    // Method call: left : obj.method(args) => obj.method(left, args)
                    Expression::MethodCall(obj, method, args) => {
                        let compiled_obj = self.compile_expression(*obj)?;
                        let method_expr = Eval::get_attr(compiled_obj, method);

                        let mut compiled_args: Vec<RustValueRef> = vec![compiled_left]; // Insert left as first argument

                        // Add the existing arguments
                        for arg in args {
                            compiled_args.push(self.compile_expression(arg)?);
                        }

                        Ok(Eval::call(method_expr, compiled_args))
                    }
                    // Simple identifier: left : func => create lambda |args...| func(left, args...)
                    // For now, create a simple function call with just the left argument
                    Expression::Identifier(_) => {
                        let compiled_right = self.compile_expression(*right)?;
                        Ok(Eval::call(compiled_right, vec![compiled_left]))
                    }
                    _ => {
                        // For other cases, treat as function and call with left as argument
                        let compiled_right = self.compile_expression(*right)?;
                        Ok(Eval::call(compiled_right, vec![compiled_left]))
                    }
                }
            }

            Expression::Match { expr, cases } => {
                let compiled_expr = self.compile_expression(*expr)?;
                let compiled_cases: Result<Vec<super::structs::EvalMatchCase>> = cases
                    .into_iter()
                    .map(|case| self.compile_match_case(case))
                    .collect();
                Ok(Eval::match_expr(super::structs::MatchData {
                    expr: compiled_expr,
                    cases: compiled_cases?,
                }))
            }

            _ => Err(anyhow::anyhow!(
                "Expression not yet implemented: {:?}",
                expr
            )),
        }
    }

    // Helper to compile unary operations to method calls: -a => a.op_get_attr("op_neg").op_call()
    fn compile_unary_method_call(
        &mut self,
        expr: Expression,
        method_name: &str,
    ) -> Result<RustValueRef> {
        let compiled_expr = self.compile_expression(expr)?;

        let get_method = Eval::get_attr(compiled_expr, method_name.to_string());
        let call_method = Eval::call(get_method, vec![]);

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

    fn compile_block_helper(&mut self, block: crate::core::Block) -> Result<RustValueRef> {
        let mut eval_statements = Vec::new();

        // Compile all statements
        for stmt in block.statements {
            eval_statements.push(self.compile_statement(stmt)?);
        }

        // Handle final expression
        match block.final_expr {
            Some(expr) => {
                let compiled_final_expr = self.compile_expression(*expr)?;
                Ok(Eval::block(eval_statements, Some(compiled_final_expr)))
            }
            None => Ok(Eval::block(eval_statements, None)),
        }
    }

    // Helper to compile "not in" as !(left in right)
    fn compile_not_in(&mut self, left: Expression, right: Expression) -> Result<RustValueRef> {
        // For 'not in', swap arguments like 'in': item not in container => !(container.op_contains(item))
        let in_expr = self.compile_binary_op(right, left, "op_contains")?;
        Ok(Eval::logical_not(in_expr))
    }

    // Helper to compile "is not" as !(left is right)
    fn compile_is_not(&mut self, left: Expression, right: Expression) -> Result<RustValueRef> {
        let compiled_left = self.compile_expression(left)?;
        let compiled_right = self.compile_expression(right)?;
        let is_expr = Eval::is(compiled_left, compiled_right);
        Ok(Eval::logical_not(is_expr))
    }

    // Helper to compile interpolated strings as string concatenation
    // "Hello ${name}" becomes "Hello " + name
    // "${a} and ${b}" becomes a + " and " + b
    fn compile_interpolated_string(
        &mut self,
        parts: Vec<InterpolationPart>,
    ) -> Result<RustValueRef> {
        if parts.is_empty() {
            return Ok(Eval::literal(Value::String(String::new())));
        }

        // Convert each part to an Eval expression
        let mut compiled_parts = Vec::new();
        for part in parts {
            match part {
                InterpolationPart::Text(text) => {
                    compiled_parts.push(Eval::literal(Value::String(text)));
                }
                InterpolationPart::Expression(expr) => {
                    // Wrap non-string expressions in str() conversion
                    let compiled_expr = self.compile_expression(expr)?;
                    let str_call =
                        Eval::call(Eval::variable("str".to_string()), vec![compiled_expr]);
                    compiled_parts.push(str_call);
                }
            }
        }

        // If there's only one part, return it directly
        if compiled_parts.len() == 1 {
            return Ok(compiled_parts.into_iter().next().unwrap());
        }

        // Use a single op_add call with all parts as arguments: first.op_add(second, third, ...)
        let mut parts_iter = compiled_parts.into_iter();
        let first_part = parts_iter.next().unwrap();
        let remaining_parts: Vec<RustValueRef> = parts_iter.collect();

        if remaining_parts.is_empty() {
            Ok(first_part)
        } else {
            let get_method = Eval::get_attr(first_part, "op_add".to_string());
            Ok(Eval::call(get_method, remaining_parts))
        }
    }

    // Helper to compile range expressions - handle at Expression level to avoid downcasting
    fn compile_range_expression(
        &mut self,
        start_expr: Expression,
        end_expr: Expression,
        inclusive: bool,
    ) -> Result<RustValueRef> {
        // Extract literal integers directly from expressions
        let start_val = match start_expr {
            Expression::Literal(LiteralValue::Int(i)) => i,
            _ => return Err(anyhow::anyhow!("Range start must be an integer literal")),
        };

        let end_val = match end_expr {
            Expression::Literal(LiteralValue::Int(i)) => i,
            _ => return Err(anyhow::anyhow!("Range end must be an integer literal")),
        };

        let range = crate::core::Range {
            start: start_val,
            end: end_val,
            inclusive,
        };
        Ok(Eval::literal(Value::Range(range)))
    }

    fn compile_class_decl(
        &mut self,
        name: String,
        members: Vec<ClassMember>,
    ) -> Result<RustValueRef> {
        let mut field_names = Vec::new();
        let mut field_defaults = Vec::new();
        let mut methods = Vec::new();

        for member in members {
            match member.kind {
                ClassMemberKind::Field(default_expr) => {
                    field_names.push(member.name);
                    let default = match default_expr {
                        Some(expr) => Some(self.compile_expression(expr)?),
                        None => None,
                    };
                    field_defaults.push(default);
                }
                ClassMemberKind::Method { params, body } => {
                    let mut param_names: Vec<String> = params.into_iter().map(|p| p.name).collect();

                    // Desugar: prepend "self" parameter for instance methods
                    param_names.insert(0, "self".to_string());

                    let compiled_body = self.compile_block_helper(body)?;
                    methods.push(ClassMethod {
                        name: member.name,
                        params: param_names,
                        body: compiled_body,
                        is_static: false,
                    });
                }
                ClassMemberKind::StaticMethod { params, body } => {
                    let param_names: Vec<String> = params.into_iter().map(|p| p.name).collect();
                    let compiled_body = self.compile_block_helper(body)?;
                    methods.push(ClassMethod {
                        name: member.name,
                        params: param_names,
                        body: compiled_body,
                        is_static: true,
                    });
                }
            }
        }

        Ok(Eval::class_decl(super::structs::ClassDeclData {
            name,
            field_names,
            field_defaults,
            methods,
        }))
    }

    fn compile_pattern(pattern: Pattern) -> Result<super::structs::EvalPattern> {
        use super::structs::{EvalDictPattern, EvalPattern};

        match pattern {
            Pattern::Variable(name) => Ok(EvalPattern::Variable(name)),
            Pattern::List(patterns) => {
                let compiled_patterns: Result<Vec<EvalPattern>> =
                    patterns.into_iter().map(Self::compile_pattern).collect();
                Ok(EvalPattern::List(compiled_patterns?))
            }
            Pattern::ListRest(patterns, rest_name) => {
                let compiled_patterns: Result<Vec<EvalPattern>> =
                    patterns.into_iter().map(Self::compile_pattern).collect();
                Ok(EvalPattern::ListRest(compiled_patterns?, rest_name))
            }
            Pattern::Dict(dict_patterns) => {
                let compiled_patterns: Vec<EvalDictPattern> = dict_patterns
                    .into_iter()
                    .map(|dp| EvalDictPattern {
                        key: dp.key,
                        alias: dp.alias,
                    })
                    .collect();
                Ok(EvalPattern::Dict(compiled_patterns))
            }
            _ => Err(anyhow::anyhow!(
                "Pattern not yet implemented: {:?}",
                pattern
            )),
        }
    }

    fn compile_match_case(
        &mut self,
        case: crate::core::MatchCase,
    ) -> Result<super::structs::EvalMatchCase> {
        use super::structs::{EvalMatchCase, EvalMatchPattern};

        let compiled_pattern = match case.pattern {
            Pattern::Literal(lit) => EvalMatchPattern::Literal(self.compile_literal(lit)),
            Pattern::Variable(name) => EvalMatchPattern::Variable(name),
            Pattern::Wildcard => EvalMatchPattern::Wildcard,
            _ => {
                return Err(anyhow::anyhow!(
                    "Complex patterns in match not yet implemented: {:?}",
                    case.pattern
                ))
            }
        };

        let compiled_guard = match case.guard {
            Some(guard_expr) => Some(self.compile_expression(guard_expr)?),
            None => None,
        };

        let compiled_body = self.compile_block_helper(case.body)?;

        Ok(EvalMatchCase {
            pattern: compiled_pattern,
            guard: compiled_guard,
            body: compiled_body,
        })
    }
}
