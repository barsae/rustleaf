use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalIs {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalIs {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let right_val = match self.right.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        if let Value::RustValue(rust_val_ref) = &right_val {
            let rust_val = rust_val_ref.borrow();
            if let Ok(result) = rust_val.op_is(&left_val) {
                return Ok(Ok(result));
            }
        }

        Ok(Ok(Value::Bool(left_val == right_val)))
    }

    fn str(&self) -> String {
        format!("{} is {}", self.left.str(), self.right.str())
    }
}
