use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

use super::eval_ref::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalBlock {
    pub statements: Vec<EvalRef>,
    pub final_expr: Option<EvalRef>,
}

impl RustValue for EvalBlock {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let block_scope = evaluator.current_env.child();
        let previous_env = std::mem::replace(&mut evaluator.current_env, block_scope);

        let mut result = Value::Unit;

        // Execute all statements
        for stmt in &self.statements {
            match stmt.eval(evaluator)? {
                Ok(_) => {}
                Err(e) => {
                    evaluator.current_env = previous_env;
                    return Ok(Err(e));
                }
            }
        }

        // Evaluate final expression if present
        if let Some(final_expr) = &self.final_expr {
            result = match final_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => {
                    evaluator.current_env = previous_env;
                    return Ok(Err(e));
                }
            };
        }

        evaluator.current_env = previous_env;
        Ok(Ok(result))
    }

    fn str(&self) -> String {
        let mut result = String::from("{");
        for stmt in &self.statements {
            result.push_str(&format!("\n    {};", stmt.str()));
        }
        if let Some(final_expr) = &self.final_expr {
            result.push_str(&format!("\n    {}", final_expr.str()));
        }
        result.push_str("\n}");
        result
    }
}
