use crate::core::RustValue;
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalTry {
    pub body: RustValueRef,
    pub catch_pattern: super::eval_ref::EvalPattern,
    pub catch_body: RustValueRef,
}

#[crate::rust_value_any]
impl RustValue for EvalTry {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Try to execute the main body
        match self.body.eval(evaluator)? {
            Ok(value) => Ok(Ok(value)),
            Err(ControlFlow::Error(ErrorKind::RaisedError(error_value))) => {
                // Bind the error to the catch pattern and execute catch body
                match evaluator.match_pattern(&self.catch_pattern, &error_value) {
                    Ok(_) => {
                        // Execute catch body
                        self.catch_body.eval(evaluator)
                    }
                    Err(e) => Ok(Err(e)),
                }
            }
            Err(other_control_flow) => {
                // Other control flow (return, break, continue) should propagate
                Ok(Err(other_control_flow))
            }
        }
    }

    fn str(&self) -> String {
        format!(
            "try {} catch {:?} {}",
            self.body.str(),
            self.catch_pattern,
            self.catch_body.str()
        )
    }
}
