use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalLogicalOr {
    pub left: Value,
    pub right: Value,
}

#[crate::rust_value_any]
impl RustValue for EvalLogicalOr {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        if left_val.is_truthy() {
            Ok(Ok(left_val))
        } else {
            self.right.eval(evaluator)
        }
    }

    fn str(&self) -> String {
        format!("{} or {}", self.left.str(), self.right.str())
    }
}
