use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalDeclarePattern {
    pub pattern: super::eval_ref::EvalPattern,
    pub init_expr: EvalRef,
}

impl RustValue for EvalDeclarePattern {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Evaluate the initialization expression
        let init_value = match self.init_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Bind the pattern to the value
        match evaluator.match_pattern(&self.pattern, &init_value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(e) => Ok(Err(e)),
        }
    }

    fn str(&self) -> String {
        format!("var {:?} = {}", self.pattern, self.init_expr.str())
    }
}
