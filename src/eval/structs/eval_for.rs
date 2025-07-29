use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use crate::core::RustValueRef;

#[derive(Debug, Clone)]
pub struct EvalFor {
    pub var_name: String,
    pub iter_expr: RustValueRef,
    pub body: RustValueRef,
}

impl RustValue for EvalFor {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let iter_value = match self.iter_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        let loop_scope = evaluator.current_env.child();
        let previous_env = std::mem::replace(&mut evaluator.current_env, loop_scope);

        let mut result = Value::Unit;

        let mut iterator = match iter_value.op_iter() {
            Ok(iter) => iter,
            Err(e) => {
                evaluator.current_env = previous_env;
                return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e))));
            }
        };

        loop {
            let next_item = match iterator.op_next() {
                Ok(item) => item,
                Err(e) => {
                    evaluator.current_env = previous_env;
                    return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e))));
                }
            };

            match next_item {
                Some(item) => {
                    evaluator.current_env.define(&self.var_name, item);

                    match self.body.eval(evaluator)? {
                        Ok(_) => {}
                        Err(ControlFlow::Break(value)) => {
                            result = value;
                            break;
                        }
                        Err(ControlFlow::Continue) => continue,
                        Err(other) => {
                            evaluator.current_env = previous_env;
                            return Ok(Err(other));
                        }
                    }
                }
                None => break,
            }
        }

        evaluator.current_env = previous_env;
        Ok(Ok(result))
    }

    fn str(&self) -> String {
        format!(
            "for {} in {} {}",
            self.var_name,
            self.iter_expr.str(),
            self.body.str()
        )
    }
}
