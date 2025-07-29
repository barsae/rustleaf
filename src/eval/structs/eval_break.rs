use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalBreak {
    pub expr: Option<Value>,
}

#[crate::rust_value_any]
impl RustValue for EvalBreak {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.expr {
            Some(e) => match e.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            },
            None => Value::Unit,
        };
        Ok(Err(ControlFlow::Break(value)))
    }

    fn str(&self) -> String {
        if let Some(expr) = &self.expr {
            format!("break {}", expr.str())
        } else {
            "break".to_string()
        }
    }
}
