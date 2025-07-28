use crate::core::RustValue;
use crate::eval::{ControlFlow, EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalLoop {
    pub body: RustValueRef,
}

impl RustValue for EvalLoop {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        loop {
            match self.body.eval(evaluator)? {
                Ok(_) => continue,
                Err(ControlFlow::Break(value)) => return Ok(Ok(value)),
                Err(ControlFlow::Continue) => continue,
                Err(other) => return Ok(Err(other)),
            }
        }
    }

    fn str(&self) -> String {
        format!("loop {}", self.body.str())
    }
}
