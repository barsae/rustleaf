use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalProgram {
    pub statements: Vec<RustValueRef>,
}

impl RustValue for EvalProgram {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        for stmt in &self.statements {
            match stmt.eval(evaluator)? {
                Ok(_) => {}
                Err(e) => return Ok(Err(e)),
            }
        }
        Ok(Ok(Value::Unit))
    }

    fn str(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
