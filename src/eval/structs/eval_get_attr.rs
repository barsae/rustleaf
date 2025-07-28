use crate::core::RustValue;
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalGetAttr {
    pub obj_expr: EvalRef,
    pub attr_name: String,
}

impl RustValue for EvalGetAttr {
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
