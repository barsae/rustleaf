use crate::core::RustValue;
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

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
            .map(|case| format!("{:?} => {}", case.pattern, case.body.str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("match {} {{ {} }}", self.data.expr.str(), cases_str)
    }
}
