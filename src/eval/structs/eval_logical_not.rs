use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalLogicalNot {
    pub expr: RustValueRef,
}

impl RustValue for EvalLogicalNot {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let val = match self.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        Ok(Ok(Value::Bool(!val.is_truthy())))
    }

    fn str(&self) -> String {
        format!("not {}", self.expr.str())
    }
}
