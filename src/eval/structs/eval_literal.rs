use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalLiteral {
    pub value: Value,
}

#[crate::rust_value_any]
impl RustValue for EvalLiteral {
    fn eval(&self, _evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(Ok(self.value.clone()))
    }

    fn str(&self) -> String {
        self.value.str()
    }
}
