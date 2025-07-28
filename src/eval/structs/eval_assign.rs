use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalAssign {
    pub name: String,
    pub expr: EvalRef,
}

impl RustValue for EvalAssign {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match self.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        match evaluator.current_env.set(&self.name, value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(err) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                err
            ))))),
        }
    }

    fn str(&self) -> String {
        format!("{} = {}", self.name, self.expr.str())
    }
}
