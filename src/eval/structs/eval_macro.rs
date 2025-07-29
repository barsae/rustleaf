use crate::core::{Args, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct EvalMacro {
    pub data: super::eval_ref::MacroData,
}

#[crate::rust_value_any]
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
        let target_eval_value = Value::RustValue(self.data.target.clone());
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
            .map(|arg| arg.str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("macro({})", args_str)
    }
}
