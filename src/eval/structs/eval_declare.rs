use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalDeclare {
    pub name: String,
    pub init_expr: Option<EvalRef>,
}

impl RustValue for EvalDeclare {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.init_expr {
            Some(expr) => match expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            },
            None => Value::Unit,
        };
        evaluator.current_env.define(&self.name, value);
        Ok(Ok(Value::Unit))
    }

    fn str(&self) -> String {
        if let Some(init_expr) = &self.init_expr {
            format!("var {} = {}", self.name, init_expr.str())
        } else {
            format!("var {}", self.name)
        }
    }
}
