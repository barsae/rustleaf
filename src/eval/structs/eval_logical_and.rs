use crate::core::RustValue;
use crate::eval::{EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalLogicalAnd {
    pub left: RustValueRef,
    pub right: RustValueRef,
}

impl RustValue for EvalLogicalAnd {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        if !left_val.is_truthy() {
            Ok(Ok(left_val))
        } else {
            self.right.eval(evaluator)
        }
    }

    fn str(&self) -> String {
        format!("{} and {}", self.left.str(), self.right.str())
    }
}
