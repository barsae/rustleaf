use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalLogicalNot {
    pub expr: Value,
}

#[crate::rust_value_any]
impl RustValue for EvalLogicalNot {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
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
