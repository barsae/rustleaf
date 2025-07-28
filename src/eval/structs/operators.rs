use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use super::literals::EvalRef;

// Logical operations

#[derive(Debug, Clone)]
pub struct EvalLogicalAnd {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalLogicalAnd {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        if !left_val.is_truthy() {
            Ok(Ok(left_val))
        } else {
            self.right.eval(evaluator)
        }
    }

    fn str(&self) -> String {
        format!("{} and {}", self.left.str(), self.right.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalLogicalOr {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalLogicalOr {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        if left_val.is_truthy() {
            Ok(Ok(left_val))
        } else {
            self.right.eval(evaluator)
        }
    }

    fn str(&self) -> String {
        format!("{} or {}", self.left.str(), self.right.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalLogicalNot {
    pub expr: EvalRef,
}

impl RustValue for EvalLogicalNot {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let val = match self.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        Ok(Ok(Value::Bool(!val.is_truthy())))
    }

    fn str(&self) -> String {
        format!("not {}", self.expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalIs {
    pub left: EvalRef,
    pub right: EvalRef,
}

impl RustValue for EvalIs {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let left_val = match self.left.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };
        let right_val = match self.right.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        if let Value::RustValue(rust_val_ref) = &right_val {
            let rust_val = rust_val_ref.borrow();
            if let Ok(result) = rust_val.op_is(&left_val) {
                return Ok(Ok(result));
            }
        }

        Ok(Ok(Value::Bool(left_val == right_val)))
    }

    fn str(&self) -> String {
        format!("{} is {}", self.left.str(), self.right.str())
    }
}
