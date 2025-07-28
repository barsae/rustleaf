use crate::core::{Args, ImportItems, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use super::literals::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalDeclare {
    pub name: String,
    pub init_expr: Option<EvalRef>,
}

impl RustValue for EvalDeclare {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.init_expr {
            Some(expr) => match expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            },
            None => Value::Unit,
        };
        evaluator.current_env.define(&self.name, value);
        Ok(Ok(Value::Unit))
    }

    fn str(&self) -> String {
        if let Some(init_expr) = &self.init_expr {
            format!("var {} = {}", self.name, init_expr.str())
        } else {
            format!("var {}", self.name)
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalAssign {
    pub name: String,
    pub expr: EvalRef,
}

impl RustValue for EvalAssign {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match self.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        match evaluator.current_env.set(&self.name, value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(err) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                err
            ))))),
        }
    }

    fn str(&self) -> String {
        format!("{} = {}", self.name, self.expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalImport {
    pub data: super::eval_ref::ImportData,
}

impl RustValue for EvalImport {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        match evaluator.load_module(&self.data.module) {
            Ok(module_scope) => {
                match &self.data.items {
                    ImportItems::All => {
                        // Import all public items from module
                        let module_vars = module_scope.iter();
                        for (name, value) in module_vars {
                            // Skip built-in functions (they start with certain patterns)
                            if !name.starts_with("__") {
                                evaluator.current_env.define(&name, value);
                            }
                        }
                    }
                    ImportItems::Specific(items) => {
                        // Import only selected items
                        for import_item in items {
                            let item_name = &import_item.name;
                            if let Some(value) = module_scope.get(item_name) {
                                let local_name = import_item.alias.as_ref().unwrap_or(item_name);
                                evaluator.current_env.define(local_name, value);
                            } else {
                                return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(
                                    anyhow!(
                                        "Item '{}' not found in module '{}'",
                                        item_name,
                                        self.data.module
                                    ),
                                ))));
                            }
                        }
                    }
                }
                Ok(Ok(Value::Unit))
            }
            Err(e) => Ok(Err(e)),
        }
    }

    fn str(&self) -> String {
        match &self.data.items {
            ImportItems::All => format!("import {}::*", self.data.module),
            ImportItems::Specific(items) => {
                let items_str = items
                    .iter()
                    .map(|item| match &item.alias {
                        Some(alias) => format!("{} as {}", item.name, alias),
                        None => item.name.clone(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("import {}::{{{}}}", self.data.module, items_str)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalMacro {
    pub data: super::eval_ref::MacroData,
}

impl RustValue for EvalMacro {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Get the macro function
        let macro_fn_result = self.data.macro_fn.eval(evaluator)?;
        let macro_fn = match macro_fn_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Evaluate macro arguments
        let mut arg_values = Vec::new();
        for arg in &self.data.args {
            let arg_result = arg.eval(evaluator)?;
            match arg_result {
                Ok(val) => arg_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }

        // Add the target Eval as a special argument
        let target_eval_value = Value::RustValue(crate::core::RustValueRef::new(
            self.data.target.0.as_rust_value(),
        ));
        arg_values.insert(0, target_eval_value);

        // Call the macro function
        let args_obj = Args::positional(arg_values);
        match macro_fn.call(args_obj) {
            Ok(result) => {
                // The macro should return an Eval node - extract it and execute it
                match result {
                    Value::RustValue(rust_val_ref) => {
                        let rust_val = rust_val_ref.borrow();
                        // Execute the generated Eval node
                        rust_val.eval(evaluator)
                    }
                    _ => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Macro must return an Eval node, got {:?}",
                        result
                    ))))),
                }
            }
            Err(e) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e)))),
        }
    }

    fn str(&self) -> String {
        let args_str = self
            .data
            .args
            .iter()
            .map(|arg| arg.0.str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("macro({})", args_str)
    }
}
