use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalIf {
    pub condition: Value,
    pub then_expr: Value,
    pub else_expr: Option<Value>,
}

#[crate::rust_value_any]
impl RustValue for EvalIf {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let condition_result = self.condition.eval(evaluator)?;
        let condition_val = match condition_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        if condition_val.is_truthy() {
            self.then_expr.eval(evaluator)
        } else {
            match &self.else_expr {
                Some(expr) => expr.eval(evaluator),
                None => Ok(Ok(Value::Unit)),
            }
        }
    }

    fn str(&self) -> String {
        let mut result = format!("if {} {}", self.condition.str(), self.then_expr.str());
        if let Some(else_expr) = &self.else_expr {
            result.push_str(&format!(" else {}", else_expr.str()));
        }
        result
    }
}
