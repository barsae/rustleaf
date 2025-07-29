use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalSetAttr {
    pub obj_expr: RustValueRef,
    pub attr_name: String,
    pub value_expr: RustValueRef,
}

impl RustValue for EvalSetAttr {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let new_value = match self.value_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.set_attr(&self.attr_name, new_value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(err) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                err
            ))))),
        }
    }

    fn str(&self) -> String {
        format!(
            "{}.{} = {}",
            self.obj_expr.str(),
            self.attr_name,
            self.value_expr.str()
        )
    }
}
