use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use super::literals::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalIf {
    pub condition: EvalRef,
    pub then_expr: EvalRef,
    pub else_expr: Option<EvalRef>,
}

impl RustValue for EvalIf {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let condition_result = self.condition.eval(evaluator)?;
        let condition_val = match condition_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        if condition_val.is_truthy() {
            self.then_expr.eval(evaluator)
        } else {
            match &self.else_expr {
                Some(expr) => expr.eval(evaluator),
                None => Ok(Ok(Value::Unit)),
            }
        }
    }

    fn str(&self) -> String {
        let mut result = format!("if {} {}", self.condition.str(), self.then_expr.str());
        if let Some(else_expr) = &self.else_expr {
            result.push_str(&format!(" else {}", else_expr.str()));
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct EvalReturn {
    pub expr: Option<EvalRef>,
}

impl RustValue for EvalReturn {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.expr {
            Some(e) => match e.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            },
            None => Value::Unit,
        };
        Ok(Err(ControlFlow::Return(value)))
    }

    fn str(&self) -> String {
        if let Some(expr) = &self.expr {
            format!("return {}", expr.str())
        } else {
            "return".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalBreak {
    pub expr: Option<EvalRef>,
}

impl RustValue for EvalBreak {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let value = match &self.expr {
            Some(e) => match e.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            },
            None => Value::Unit,
        };
        Ok(Err(ControlFlow::Break(value)))
    }

    fn str(&self) -> String {
        if let Some(expr) = &self.expr {
            format!("break {}", expr.str())
        } else {
            "break".to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalContinue;

impl RustValue for EvalContinue {
    fn eval(&self, _evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        Ok(Err(ControlFlow::Continue))
    }

    fn str(&self) -> String {
        "continue".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct EvalLoop {
    pub body: EvalRef,
}

impl RustValue for EvalLoop {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        loop {
            match self.body.eval(evaluator)? {
                Ok(_) => continue,
                Err(ControlFlow::Break(value)) => return Ok(Ok(value)),
                Err(ControlFlow::Continue) => continue,
                Err(other) => return Ok(Err(other)),
            }
        }
    }

    fn str(&self) -> String {
        format!("loop {}", self.body.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalWhile {
    pub condition: EvalRef,
    pub body: EvalRef,
}

impl RustValue for EvalWhile {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        loop {
            let condition_val = match self.condition.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => return Ok(Err(e)),
            };

            if !condition_val.is_truthy() {
                return Ok(Ok(Value::Unit));
            }

            match self.body.eval(evaluator)? {
                Ok(_) => continue,
                Err(ControlFlow::Break(value)) => return Ok(Ok(value)),
                Err(ControlFlow::Continue) => continue,
                Err(other) => return Ok(Err(other)),
            }
        }
    }

    fn str(&self) -> String {
        format!("while {} {}", self.condition.str(), self.body.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalFor {
    pub var_name: String,
    pub iter_expr: EvalRef,
    pub body: EvalRef,
}

impl RustValue for EvalFor {
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
