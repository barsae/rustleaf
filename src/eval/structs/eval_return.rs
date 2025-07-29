use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalReturn {
    pub expr: Option<RustValueRef>,
}

#[crate::rust_value_any]
impl RustValue for EvalReturn {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.expr {
            Some(e) => match e.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            },
            None => Value::Unit,
        };
        Ok(Err(ControlFlow::Return(value)))
    }

    fn str(&self) -> String {
        if let Some(expr) = &self.expr {
            format!("return {}", expr.str())
        } else {
            "return".to_string()
        }
    }
}
