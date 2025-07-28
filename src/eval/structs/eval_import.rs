use crate::core::{ImportItems, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

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
