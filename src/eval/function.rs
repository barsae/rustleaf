use super::{scope::ScopeRef, Params};
use crate::core::*;
use anyhow::anyhow;
use std::path::PathBuf;

use super::{ControlFlow, ErrorKind, Evaluator};

#[derive(Debug, Clone)]
pub struct RustLeafFunction {
    params: Params,
    body: RustValueRef,
    closure_env: ScopeRef,
}

impl RustLeafFunction {
    pub fn new(params: Params, body: RustValueRef, closure_env: ScopeRef) -> Self {
        Self {
            params,
            body,
            closure_env,
        }
    }

    pub fn into_value(self) -> Value {
        Value::from_rust(self)
    }
}

impl RustValue for RustLeafFunction {
    fn call(&self, mut args: Args) -> anyhow::Result<Value> {
        // Analyze parameter structure
        let regular_params: Vec<_> = self
            .params
            .iter()
            .filter(|(_, _, kind)| matches!(kind, ParameterKind::Regular))
            .collect();
        let rest_param = self
            .params
            .iter()
            .find(|(_, _, kind)| matches!(kind, ParameterKind::Rest));

        // Count required parameters (regular params without defaults)
        let required_params = regular_params
            .iter()
            .filter(|(_, default, _)| default.is_none())
            .count();
        let max_regular_params = regular_params.len();

        // Check argument count - with rest params, we accept unlimited args
        if rest_param.is_some() {
            // With rest param: need at least required regular params
            if args.len() < required_params {
                return Err(anyhow!(
                    "Function expects at least {} arguments, got {}",
                    required_params,
                    args.len()
                ));
            }
        } else {
            // Without rest param: standard validation
            if args.len() < required_params || args.len() > max_regular_params {
                return Err(anyhow!(
                    "Function expects {} to {} arguments, got {}",
                    required_params,
                    max_regular_params,
                    args.len()
                ));
            }
        }

        // Create new scope for function execution
        let function_scope = self.closure_env.child();

        args.set_function_name("user function");

        // Bind regular parameters to arguments using fluent API
        for (param_name, default_value, kind) in self.params.iter() {
            match kind {
                ParameterKind::Regular => {
                    let arg_value = if let Some(default) = default_value {
                        // Parameter has default - use optional with default
                        args.optional(param_name, default.clone())
                    } else {
                        // Required parameter
                        args.expect(param_name)?
                    };
                    function_scope.define(param_name, arg_value);
                }
                ParameterKind::Rest => {
                    // Collect remaining arguments into a list
                    let rest_args = args.rest();
                    let list_value = Value::new_list_with_values(rest_args);
                    function_scope.define(param_name, list_value);
                }
                ParameterKind::Keyword => {
                    // TODO: Implement keyword parameters later
                    return Err(anyhow!("Keyword parameters not yet implemented"));
                }
            }
        }

        // Validate all args consumed
        args.complete()?;

        // Create evaluator with function scope - use same globals as parent
        let mut evaluator = Evaluator {
            globals: self.closure_env.clone(), // Use closure environment as globals for now
            current_env: function_scope,
            current_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        };

        // Evaluate function body
        match evaluator.eval(&self.body) {
            Ok(value) => Ok(value),
            Err(ControlFlow::Return(value)) => Ok(value),
            Err(ControlFlow::Error(ErrorKind::SystemError(err))) => Err(err),
            Err(ControlFlow::Error(ErrorKind::RaisedError(value))) => {
                // Convert raised value to string for error display
                match value {
                    Value::String(s) => Err(anyhow::anyhow!("{}", s)),
                    _ => Err(anyhow::anyhow!("{:?}", value)),
                }
            }
            Err(other) => Err(anyhow!("Invalid control flow in function: {:?}", other)),
        }
    }
}
