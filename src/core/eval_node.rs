/// RustValue wrapper for Eval nodes, used by the macro system
use crate::core::RustValue;
use crate::eval::{Eval, EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalNode {
    pub node: Eval,
}

impl EvalNode {
    pub fn new(node: Eval) -> Self {
        Self { node }
    }

    pub fn pretty_print(&self, indent: usize) -> String {
        let indent_str = "    ".repeat(indent);

        match &self.node {
            Eval::Program(statements) => {
                if statements.is_empty() {
                    return "".to_string();
                }
                
                let parts: Vec<String> = statements
                    .iter()
                    .map(|stmt| format!("{}{};", indent_str, EvalNode::new(stmt.clone()).pretty_print(indent)))
                    .collect();
                
                parts.join("\n")
            }
            Eval::Block(statements, final_expr) => {
                let inner_indent_str = "    ".repeat(indent + 1);
                let mut parts = Vec::new();

                for stmt in statements.iter() {
                    parts.push(format!("{}{};", inner_indent_str, EvalNode::new(stmt.clone()).pretty_print(indent + 1)));
                }
                if let Some(expr) = final_expr {
                    parts.push(format!(
                        "{}{}",
                        inner_indent_str,
                        EvalNode::new(expr.as_ref().clone()).pretty_print(indent + 1)
                    ));
                }

                if parts.is_empty() {
                    "{}".to_string()
                } else {
                    format!("{{\n{}\n{}}}", parts.join("\n"), indent_str)
                }
            }
            Eval::Function(data) => {
                let param_list = data.params
                    .iter()
                    .map(|(name, _, _)| name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "fn {}({}) {}",
                    data.name,
                    param_list,
                    EvalNode::new(data.body.as_ref().clone()).pretty_print(indent)
                )
            }
            Eval::Variable(name) => name.clone(),
            Eval::ClassDecl(data) => {
                let fields = data.field_names
                    .iter()
                    .zip(data.field_defaults.iter())
                    .map(|(name, default)| match default {
                        Some(def) => {
                            format!("{} = {}", name, EvalNode::new(def.clone()).str())
                        }
                        None => name.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("class {}({})", data.name, fields)
            }
            Eval::Call(func, args) => {
                let func_str = EvalNode::new(func.as_ref().clone()).str();
                let args_str = args
                    .iter()
                    .map(|arg| EvalNode::new(arg.clone()).str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", func_str, args_str)
            }
            Eval::Literal(value) => {
                use crate::core::Value;
                match value {
                    Value::String(s) => format!("\"{}\"", s),
                    Value::Int(i) => i.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    Value::Unit => "unit".to_string(),
                    _ => format!("{:?}", value),
                }
            }
            Eval::If(condition, then_branch, else_branch) => {
                let cond_str = EvalNode::new(condition.as_ref().clone()).str();
                let then_str = EvalNode::new(then_branch.as_ref().clone()).str();
                match else_branch {
                    Some(else_br) => {
                        let else_str = EvalNode::new(else_br.as_ref().clone()).str();
                        format!("if {} {} else {}", cond_str, then_str, else_str)
                    }
                    None => format!("if {} {}", cond_str, then_str),
                }
            }
            Eval::Loop(body) => {
                let body_str = EvalNode::new(body.as_ref().clone()).str();
                format!("loop {}", body_str)
            }
            Eval::While(condition, body) => {
                let cond_str = EvalNode::new(condition.as_ref().clone()).str();
                let body_str = EvalNode::new(body.as_ref().clone()).str();
                format!("while {} {}", cond_str, body_str)
            }
            Eval::For(var, iterable, body) => {
                let iter_str = EvalNode::new(iterable.as_ref().clone()).str();
                let body_str = EvalNode::new(body.as_ref().clone()).str();
                format!("for {} in {} {}", var, iter_str, body_str)
            }
            Eval::Return(value) => match value {
                Some(val) => format!("return {}", EvalNode::new(val.as_ref().clone()).str()),
                None => "return".to_string(),
            },
            Eval::Break(value) => match value {
                Some(val) => format!("break {}", EvalNode::new(val.as_ref().clone()).str()),
                None => "break".to_string(),
            },
            Eval::Continue => "continue".to_string(),
            Eval::Try(try_expr, var, catch_expr) => {
                let try_str = EvalNode::new(try_expr.as_ref().clone()).str();
                let catch_str = EvalNode::new(catch_expr.as_ref().clone()).str();
                format!("try {} catch {} {}", try_str, var, catch_str)
            }
            Eval::With(data) => {
                let resources_str = data.resources
                    .iter()
                    .map(|(name, value)| {
                        format!("{} = {}", name, EvalNode::new(value.clone()).str())
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                let body_str = EvalNode::new(data.body.as_ref().clone()).str();
                format!("with {} {}", resources_str, body_str)
            }
            Eval::Import(data) => {
                use crate::core::ImportItems;
                let items_str = match &data.items {
                    ImportItems::All => "*".to_string(),
                    ImportItems::Specific(items) => items
                        .iter()
                        .map(|item| match &item.alias {
                            Some(alias) => format!("{} as {}", item.name, alias),
                            None => item.name.clone(),
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                };
                format!("import {{ {} }} from \"{}\"", items_str, data.module)
            }
            Eval::Match(data) => {
                let expr_str = EvalNode::new(data.expr.as_ref().clone()).str();
                let cases_str = data.cases
                    .iter()
                    .map(|case| {
                        let body_str = EvalNode::new(case.body.clone()).str();
                        format!("    {:?} => {}", case.pattern, body_str)
                    })
                    .collect::<Vec<_>>()
                    .join(",\n");
                format!("match {} {{\n{}\n}}", expr_str, cases_str)
            }
            Eval::Declare(name, value) => match value {
                Some(val) => format!(
                    "var {} = {}",
                    name,
                    EvalNode::new(val.as_ref().clone()).str()
                ),
                None => format!("var {}", name),
            },
            Eval::Assign(name, value) => {
                format!(
                    "{} = {}",
                    name,
                    EvalNode::new(value.as_ref().clone()).str()
                )
            }
            Eval::GetAttr(obj, attr) => {
                format!(
                    "{}.{}",
                    EvalNode::new(obj.as_ref().clone()).str(),
                    attr
                )
            }
            Eval::Macro(data) => {
                let macro_str = EvalNode::new(data.macro_fn.as_ref().clone()).str();
                let target_str = EvalNode::new(data.target.as_ref().clone()).str();
                let args_str = data.args
                    .iter()
                    .map(|arg| EvalNode::new(arg.clone()).str())
                    .collect::<Vec<_>>()
                    .join(", ");
                if data.args.is_empty() {
                    format!("#[{}] {}", macro_str, target_str)
                } else {
                    format!("#[{}({})] {}", macro_str, args_str, target_str)
                }
            }
            Eval::List(elements) => {
                let elements_str = elements
                    .iter()
                    .map(|elem| EvalNode::new(elem.clone()).str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", elements_str)
            }
            Eval::Dict(pairs) => {
                let pairs_str = pairs
                    .iter()
                    .map(|(key, value)| {
                        let key_str = EvalNode::new(key.clone()).str();
                        let value_str = EvalNode::new(value.clone()).str();
                        format!("{}: {}", key_str, value_str)
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", pairs_str)
            }
            Eval::LogicalAnd(left, right) => {
                let left_str = EvalNode::new(left.as_ref().clone()).str();
                let right_str = EvalNode::new(right.as_ref().clone()).str();
                format!("{} and {}", left_str, right_str)
            }
            Eval::LogicalOr(left, right) => {
                let left_str = EvalNode::new(left.as_ref().clone()).str();
                let right_str = EvalNode::new(right.as_ref().clone()).str();
                format!("{} or {}", left_str, right_str)
            }
            Eval::LogicalNot(expr) => {
                let expr_str = EvalNode::new(expr.as_ref().clone()).str();
                format!("not {}", expr_str)
            }
            Eval::Is(left, right) => {
                let left_str = EvalNode::new(left.as_ref().clone()).str();
                let right_str = EvalNode::new(right.as_ref().clone()).str();
                format!("{} is {}", left_str, right_str)
            }
            Eval::Lambda(data) => {
                let params_str = data.params.join(", ");
                let body_str = EvalNode::new(data.body.as_ref().clone()).str();
                format!("|{}| {}", params_str, body_str)
            }
            Eval::GetItem(obj, index) => {
                let obj_str = EvalNode::new(obj.as_ref().clone()).str();
                let index_str = EvalNode::new(index.as_ref().clone()).str();
                format!("{}[{}]", obj_str, index_str)
            }
            Eval::SetAttr(obj, attr, value) => {
                let obj_str = EvalNode::new(obj.as_ref().clone()).str();
                let value_str = EvalNode::new(value.as_ref().clone()).str();
                format!("{}.{} = {}", obj_str, attr, value_str)
            }
            Eval::SetItem(obj, index, value) => {
                let obj_str = EvalNode::new(obj.as_ref().clone()).str();
                let index_str = EvalNode::new(index.as_ref().clone()).str();
                let value_str = EvalNode::new(value.as_ref().clone()).str();
                format!("{}[{}] = {}", obj_str, index_str, value_str)
            }
            Eval::DeclarePattern(pattern, value) => {
                let value_str = EvalNode::new(value.as_ref().clone()).str();
                format!("var {:?} = {}", pattern, value_str)
            }
        }
    }
}

impl RustValue for EvalNode {
    fn get_attr(&self, name: &str) -> Option<crate::core::Value> {
        use crate::core::Value;

        // Special attribute to report the node type for type checking
        if name == "node_type" {
            let type_name = match &self.node {
                Eval::Function(_) => "Function",
                Eval::ClassDecl(_) => "Class",
                Eval::Variable(_) => "Variable",
                Eval::Call(_, _) => "Call",
                Eval::Program(_) => "Program",
                Eval::Block(_, _) => "Block",
                Eval::Literal(_) => "Literal",
                Eval::If(_, _, _) => "If",
                Eval::Loop(_) => "Loop",
                Eval::While(_, _) => "While",
                Eval::For(_, _, _) => "For",
                Eval::Return(_) => "Return",
                Eval::Break(_) => "Break",
                Eval::Continue => "Continue",
                Eval::Try(_, _, _) => "Try",
                Eval::With(_) => "With",
                Eval::Import(_) => "Import",
                Eval::Match(_) => "Match",
                _ => "Unknown",
            };
            return Some(Value::String(type_name.to_string()));
        }

        match &self.node {
            Eval::Function(data) => {
                match name {
                    "name" => Some(Value::String(data.name.clone())),
                    "params" => {
                        // Convert params Vec to RustLeaf list
                        let param_names: Vec<Value> = data.params
                            .iter()
                            .map(|(name, _, _)| Value::String(name.clone()))
                            .collect();
                        Some(Value::new_list_with_values(param_names))
                    }
                    "body" => Some(Value::from_rust(EvalNode::new(data.body.as_ref().clone()))),
                    _ => None,
                }
            }
            Eval::ClassDecl(data) => {
                match name {
                    "name" => Some(Value::String(data.name.clone())),
                    "field_names" => {
                        let names: Vec<Value> = data.field_names
                            .iter()
                            .map(|name| Value::String(name.clone()))
                            .collect();
                        Some(Value::new_list_with_values(names))
                    }
                    "field_defaults" => {
                        let defaults: Vec<Value> = data.field_defaults
                            .iter()
                            .map(|opt_eval| match opt_eval {
                                Some(eval) => Value::from_rust(EvalNode::new(eval.clone())),
                                None => Value::Null,
                            })
                            .collect();
                        Some(Value::new_list_with_values(defaults))
                    }
                    "methods" => {
                        // TODO: Convert methods to appropriate format
                        Some(Value::new_list())
                    }
                    _ => None,
                }
            }
            Eval::Variable(var_name) => match name {
                "name" => Some(Value::String(var_name.clone())),
                _ => None,
            },
            Eval::Call(func, args) => match name {
                "func" => Some(Value::from_rust(EvalNode::new(func.as_ref().clone()))),
                "args" => {
                    let arg_nodes: Vec<Value> = args
                        .iter()
                        .map(|arg| Value::from_rust(EvalNode::new(arg.clone())))
                        .collect();
                    Some(Value::new_list_with_values(arg_nodes))
                }
                _ => None,
            },
            Eval::Program(statements) => match name {
                "statements" => {
                    let stmt_nodes: Vec<Value> = statements
                        .iter()
                        .map(|stmt| Value::from_rust(EvalNode::new(stmt.clone())))
                        .collect();
                    Some(Value::new_list_with_values(stmt_nodes))
                }
                _ => None,
            },
            Eval::Block(statements, final_expr) => match name {
                "statements" => {
                    let stmt_nodes: Vec<Value> = statements
                        .iter()
                        .map(|stmt| Value::from_rust(EvalNode::new(stmt.clone())))
                        .collect();
                    Some(Value::new_list_with_values(stmt_nodes))
                }
                "final_expr" => match final_expr {
                    Some(expr) => Some(Value::from_rust(EvalNode::new(expr.as_ref().clone()))),
                    None => Some(Value::Null),
                },
                _ => None,
            },
            // Add other Eval variants as needed
            _ => None,
        }
    }

    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(evaluator.eval(&self.node))
    }

    fn str(&self) -> String {
        self.pretty_print(0)
    }
}
