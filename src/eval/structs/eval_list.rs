use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalList {
    pub elements: Vec<Value>,
}

#[crate::rust_value_any]
impl RustValue for EvalList {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut list_values = Vec::new();
        for element in &self.elements {
            match element.eval(evaluator)? {
                Ok(val) => list_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }
        Ok(Ok(Value::new_list_with_values(list_values)))
    }

    fn str(&self) -> String {
        let elements_str = self
            .elements
            .iter()
            .map(|elem| elem.str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("[{elements_str}]")
    }
}
