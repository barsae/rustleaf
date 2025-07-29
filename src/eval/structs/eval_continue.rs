use crate::core::RustValue;
use crate::eval::{ControlFlow, EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalContinue;

#[crate::rust_value_any]
impl RustValue for EvalContinue {
    fn eval(&self, _evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(Err(ControlFlow::Continue))
    }

    fn str(&self) -> String {
        "continue".to_string()
    }
}
