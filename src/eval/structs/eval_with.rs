use crate::core::{Args, RustValue};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use super::eval_ref::WithData;

#[derive(Debug, Clone)]
pub struct EvalWith {
    pub data: WithData,
}

impl RustValue for EvalWith {
    crate::impl_rust_value_any!(Self);
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
            .map(|(name, expr)| format!("{} = {}", name, expr.str()))
            .collect::<Vec<_>>()
            .join(", ");
        format!("with {} {}", resources_str, self.data.body.str())
    }
}
