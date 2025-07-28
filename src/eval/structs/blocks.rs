use crate::core::{Args, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use super::literals::EvalRef;

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

#[derive(Debug, Clone)]
pub struct EvalProgram {
    pub statements: Vec<EvalRef>,
}

impl RustValue for EvalProgram {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        for stmt in &self.statements {
            match stmt.eval(evaluator)? {
                Ok(_) => {}
                Err(e) => return Ok(Err(e)),
            }
        }
        Ok(Ok(Value::Unit))
    }

    fn str(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct EvalWith {
    pub data: crate::eval::core::WithData,
}

impl RustValue for EvalWith {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let mut resources = Vec::new();

        // Acquire all resources
        for (resource_name, resource_expr) in &self.data.resources {
            let resource_value = match resource_expr.eval(evaluator)? {
                Ok(val) => val,
                Err(e) => {
                    // Cleanup any already acquired resources
                    evaluator.cleanup_resources(&resources);
                    return Ok(Err(e));
                }
            };

            // Call op_open method if it exists
            let open_method = resource_value.get_attr("op_open", evaluator);
            if let Some(open_method) = open_method {
                let args = Args::positional(vec![]);
                if let Err(e) = open_method.call(args) {
                    evaluator.cleanup_resources(&resources);
                    return Ok(Err(ControlFlow::Error(ErrorKind::SystemError(e))));
                }
            }

            // Define the resource in current scope
            evaluator
                .current_env
                .define(resource_name.clone(), resource_value.clone());
            resources.push((resource_name.clone(), resource_value));
        }

        // Execute the body
        let body_result = self.data.body.eval(evaluator)?;

        // Always cleanup resources
        evaluator.cleanup_resources(&resources);

        Ok(body_result)
    }

    fn str(&self) -> String {
        let resources_str = self
            .data
            .resources
            .iter()
            .map(|(name, expr)| format!("{} = {}", name, expr.0.str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("with {} {}", resources_str, self.data.body.0.str())
    }
}
