use crate::core::{Args, RustValue};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalGetItem {
    pub obj_expr: RustValueRef,
    pub index_expr: RustValueRef,
}

impl RustValue for EvalGetItem {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let index_value = match self.index_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.get_attr("op_get_item", evaluator) {
            Some(method) => {
                let args = Args::positional(vec![index_value]);
                match method.call(args) {
                    Ok(result) => Ok(Ok(result)),
                    Err(e) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e)))),
                }
            }
            None => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No op_get_item method on value {:?}",
                obj_value
            ))))),
        }
    }

    fn str(&self) -> String {
        format!("{}[{}]", self.obj_expr.str(), self.index_expr.str())
    }
}
