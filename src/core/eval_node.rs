/// RustValue wrapper for Eval nodes, used by the macro system
use crate::core::RustValue;
use crate::eval::{Eval, EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalNode {
    pub node: Eval,
}

impl EvalNode {
    pub fn new(node: Eval) -> Self {
        Self { node }
    }
}

impl RustValue for EvalNode {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(evaluator.eval(&self.node))
    }
}