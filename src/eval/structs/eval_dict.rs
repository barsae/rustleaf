use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalDict {
    pub pairs: Vec<(EvalRef, EvalRef)>,
}

impl RustValue for EvalDict {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut dict_map = indexmap::IndexMap::new();
        for (key_expr, value_expr) in &self.pairs {
            let key_val = match key_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };
            let value_val = match value_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };

            let key_str = match key_val {
                Value::String(s) => s,
                Value::Int(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => {
                    return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                        "Dictionary keys must be strings, numbers, or booleans, got {:?}",
                        key_val
                    )))))
                }
            };

            dict_map.insert(key_str, value_val);
        }
        Ok(Ok(Value::new_dict_with_map(dict_map)))
    }

    fn str(&self) -> String {
        let pairs_str = self
            .pairs
            .iter()
            .map(|(key, value)| format!("{}: {}", key.str(), value.str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("{{{}}}", pairs_str)
    }
}
