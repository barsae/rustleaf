use crate::core::{Args, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalCall {
    pub func_expr: RustValueRef,
    pub args: Vec<RustValueRef>,
}

impl RustValue for EvalCall {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Get the function value
        let func_result = self.func_expr.eval(evaluator)?;
        let func_value = match func_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Handle class constructor calls
        if let Value::Class(class_ref) = &func_value {
            let class = class_ref.borrow();

            // Evaluate arguments for constructor
            let mut arg_evals = Vec::new();
            for arg in &self.args {
                let arg_result = arg.eval(evaluator)?;
                match arg_result {
                    Ok(val) => {
                        arg_evals.push(crate::eval::Eval::literal(val));
                    }
                    Err(e) => return Ok(Err(e)),
                }
            }

            return Ok(evaluator.handle_class_constructor(&class, &arg_evals));
        }

        // Evaluate all arguments
        let mut arg_values = Vec::new();
        for arg in &self.args {
            let arg_result = arg.eval(evaluator)?;
            match arg_result {
                Ok(val) => arg_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }

        let args_obj = Args::positional(arg_values);

        let result = func_value
            .call(args_obj)
            .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)));

        match result {
            Ok(value) => {
                if let Value::Raised(error_value) = value {
                    return Ok(Err(ControlFlow::Error(ErrorKind::RaisedError(
                        *error_value,
                    ))));
                }
                Ok(Ok(value))
            }
            Err(control_flow) => Ok(Err(control_flow)),
        }
    }

    fn str(&self) -> String {
        let args_str = self
            .args
            .iter()
            .map(|arg| arg.str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}({})", self.func_expr.str(), args_str)
    }
}
