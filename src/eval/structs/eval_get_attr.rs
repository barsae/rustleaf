use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct EvalGetAttr {
    pub obj_expr: Value,
    pub attr_name: String,
}

#[crate::rust_value_any]
impl RustValue for EvalGetAttr {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.get_attr(&self.attr_name, evaluator) {
            Some(value) => Ok(Ok(value)),
            None => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No attribute '{}' on value {:?}",
                self.attr_name,
                obj_value
            ))))),
        }
    }

    fn str(&self) -> String {
        format!("{}.{}", self.obj_expr.str(), self.attr_name)
    }
}
