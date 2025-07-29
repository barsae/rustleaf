use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalLiteral {
    pub value: Value,
}

impl RustValue for EvalLiteral {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, _evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(Ok(self.value.clone()))
    }

    fn str(&self) -> String {
        self.value.str()
    }
}
