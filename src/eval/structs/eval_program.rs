use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalProgram {
    pub statements: Vec<Value>,
}

#[crate::rust_value_any]
impl RustValue for EvalProgram {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut last_result = Value::Unit;

        for (i, stmt) in self.statements.iter().enumerate() {
            match stmt.eval(evaluator)? {
                Ok(result) => {
                    // For the last statement, preserve its result
                    if i == self.statements.len() - 1 {
                        last_result = result;
                    }
                }
                Err(e) => return Ok(Err(e)),
            }
        }
        Ok(Ok(last_result))
    }

    fn str(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
