use crate::core::RustValue;
use crate::eval::{EvalResult, Evaluator};

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalLogicalAnd {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalLogicalAnd {
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
