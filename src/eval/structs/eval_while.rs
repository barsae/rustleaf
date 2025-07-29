use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalWhile {
    pub condition: RustValueRef,
    pub body: RustValueRef,
}

#[crate::rust_value_any]
impl RustValue for EvalWhile {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        loop {
            let condition_val = match self.condition.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };

            if !condition_val.is_truthy() {
                return Ok(Ok(Value::Unit));
            }

            match self.body.eval(evaluator)? {
                Ok(_) => continue,
                Err(ControlFlow::Break(value)) => return Ok(Ok(value)),
                Err(ControlFlow::Continue) => continue,
                Err(other) => return Ok(Err(other)),
            }
        }
    }

    fn str(&self) -> String {
        format!("while {} {}", self.condition.str(), self.body.str())
    }
}
