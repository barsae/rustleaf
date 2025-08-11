use crate::core::{Args, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct EvalSetItem {
    pub obj_expr: Value,
    pub index_expr: Value,
    pub value_expr: Value,
}

#[crate::rust_value_any]
impl RustValue for EvalSetItem {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let obj_value = match self.obj_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let index_value = match self.index_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let new_value = match self.value_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        match obj_value.get_attr("op_set_item", evaluator) {
            Some(method) => {
                let args = Args::positional(vec![index_value, new_value]);
                match method.call(args) {
                    Ok(_) => Ok(Ok(Value::Unit)),
                    Err(e) => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e)))),
                }
            }
            None => Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "No op_set_item method on value {:?}",
                obj_value
            ))))),
        }
    }

    fn str(&self) -> String {
        format!(
            "{}[{}] = {}",
            self.obj_expr.str(),
            self.index_expr.str(),
            self.value_expr.str()
        )
    }
}
