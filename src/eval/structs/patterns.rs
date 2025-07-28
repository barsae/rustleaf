use crate::core::{RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

use super::literals::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalMatch {
    pub data: super::eval_ref::MatchData,
}

impl RustValue for EvalMatch {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Evaluate the match expression
        let match_value = match self.data.expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Try each case in order
        for case in &self.data.cases {
            // Check if pattern matches
            match evaluator.match_pattern_matches(&case.pattern, &match_value) {
                Ok(true) => {
                    // Check guard condition if present
                    if let Some(guard) = &case.guard {
                        let guard_result = match guard.eval(evaluator)? {
                            Ok(val) => val,
                            Err(e) => return Ok(Err(e)),
                        };
                        if !guard_result.is_truthy() {
                            continue;
                        }
                    }

                    // Pattern matches (and guard passes), execute body
                    // For Variable patterns, bind the variable
                    if let super::eval_ref::EvalMatchPattern::Variable(var_name) = &case.pattern {
                        evaluator.current_env.define(var_name.clone(), match_value);
                    }

                    return case.body.eval(evaluator);
                }
                Ok(false) => continue,
                Err(e) => return Ok(Err(e)),
            }
        }

        // No pattern matched
        Ok(Err(ControlFlow::Error(ErrorKind::SystemError(anyhow!(
            "No pattern matched in match expression"
        )))))
    }

    fn str(&self) -> String {
        let cases_str = self
            .data
            .cases
            .iter()
            .map(|case| format!("{:?} => {}", case.pattern, case.body.0.str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("match {} {{ {} }}", self.data.expr.0.str(), cases_str)
    }
}

#[derive(Debug, Clone)]
pub struct EvalDeclarePattern {
    pub pattern: super::eval_ref::EvalPattern,
    pub init_expr: EvalRef,
}

impl RustValue for EvalDeclarePattern {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Evaluate the initialization expression
        let init_value = match self.init_expr.eval(evaluator)? {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Bind the pattern to the value
        match evaluator.match_pattern(&self.pattern, &init_value) {
            Ok(_) => Ok(Ok(Value::Unit)),
            Err(e) => Ok(Err(e)),
        }
    }

    fn str(&self) -> String {
        format!("var {:?} = {}", self.pattern, self.init_expr.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalTry {
    pub body: EvalRef,
    pub catch_pattern: super::eval_ref::EvalPattern,
    pub catch_body: EvalRef,
}

impl RustValue for EvalTry {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Try to execute the main body
        match self.body.eval(evaluator)? {
            Ok(value) => Ok(Ok(value)),
            Err(ControlFlow::Error(ErrorKind::RaisedError(error_value))) => {
                // Bind the error to the catch pattern and execute catch body
                match evaluator.match_pattern(&self.catch_pattern, &error_value) {
                    Ok(_) => {
                        // Execute catch body
                        self.catch_body.eval(evaluator)
                    }
                    Err(e) => Ok(Err(e)),
                }
            }
            Err(other_control_flow) => {
                // Other control flow (return, break, continue) should propagate
                Ok(Err(other_control_flow))
            }
        }
    }

    fn str(&self) -> String {
        format!(
            "try {} catch {:?} {}",
            self.body.str(),
            self.catch_pattern,
            self.catch_body.str()
        )
    }
}
