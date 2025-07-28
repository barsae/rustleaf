use crate::core::{Args, RustValue, Value};
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};

use super::literals::EvalRef;

#[derive(Debug, Clone)]
pub struct EvalCall {
    pub func_expr: EvalRef,
    pub args: Vec<EvalRef>,
}

impl RustValue for EvalCall {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        // Get the function value
        let func_result = self.func_expr.eval(evaluator)?;
        let func_value = match func_result {
            Ok(val) => val,
            Err(e) => return Ok(Err(e)),
        };

        // Handle class constructor calls
        if let Value::Class(class_ref) = &func_value {
            let class = class_ref.borrow();

            // Evaluate arguments for constructor
            let mut arg_evals = Vec::new();
            for arg in &self.args {
                let arg_result = arg.eval(evaluator)?;
                match arg_result {
                    Ok(val) => {
                        // Convert back to old Eval for compatibility
                        // TODO: Remove this conversion once we fully migrate
                        arg_evals.push(crate::eval::Eval::literal(val));
                    }
                    Err(e) => return Ok(Err(e)),
                }
            }

            return Ok(evaluator.handle_class_constructor(&class, &arg_evals));
        }

        // Evaluate all arguments
        let mut arg_values = Vec::new();
        for arg in &self.args {
            let arg_result = arg.eval(evaluator)?;
            match arg_result {
                Ok(val) => arg_values.push(val),
                Err(e) => return Ok(Err(e)),
            }
        }

        let args_obj = Args::positional(arg_values);

        let result = func_value
            .call(args_obj)
            .map_err(|e| ControlFlow::Error(ErrorKind::SystemError(e)));

        match result {
            Ok(value) => {
                if let Value::Raised(error_value) = value {
                    return Ok(Err(ControlFlow::Error(ErrorKind::RaisedError(
                        *error_value,
                    ))));
                }
                Ok(Ok(value))
            }
            Err(control_flow) => Ok(Err(control_flow)),
        }
    }

    fn str(&self) -> String {
        let args_str = self
            .args
            .iter()
            .map(|arg| arg.str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}({})", self.func_expr.str(), args_str)
    }
}

#[derive(Debug, Clone)]
pub struct EvalFunction {
    pub data: super::eval_ref::FunctionData,
}

impl RustValue for EvalFunction {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::{Params, RustLeafFunction};

        let params = Params::from_vec(self.data.params.clone());
        let function = RustLeafFunction::new(
            params,
            self.data.body.as_ref().clone(),
            evaluator.current_env.clone(),
        );

        let function_value = Value::from_rust(function);
        evaluator
            .current_env
            .define(&self.data.name, function_value);
        Ok(Ok(Value::Unit))
    }

    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "name" => Some(Value::String(self.data.name.clone())),
            "params" => {
                // Return parameter names as a list of strings
                let param_names: Vec<Value> = self
                    .data
                    .params
                    .iter()
                    .map(|(name, _, _)| Value::String(name.clone()))
                    .collect();
                Some(Value::new_list_with_values(param_names))
            }
            "body" => {
                // Return the actual body Eval object
                Some(Value::RustValue(crate::core::RustValueRef::new(
                    self.data.body.0.as_rust_value(),
                )))
            }
            _ => None,
        }
    }

    fn str(&self) -> String {
        let params_str = self
            .data
            .params
            .iter()
            .map(|(name, default_value, _kind)| match default_value {
                Some(default) => format!("{} = {}", name, default.str()),
                None => name.clone(),
            })
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "fn {}({}) {}",
            self.data.name,
            params_str,
            self.data.body.0.str()
        )
    }
}

#[derive(Debug, Clone)]
pub struct EvalLambda {
    pub data: super::eval_ref::LambdaData,
}

impl RustValue for EvalLambda {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::{Params, RustLeafFunction};

        let params = Params::from_names(&self.data.params);
        let lambda_fn = RustLeafFunction::new(
            params,
            self.data.body.as_ref().clone(),
            evaluator.current_env.clone(),
        );

        Ok(Ok(Value::from_rust(lambda_fn)))
    }

    fn str(&self) -> String {
        let params_str = self.data.params.join(", ");
        format!("fn({}) {}", params_str, self.data.body.0.str())
    }
}

#[derive(Debug, Clone)]
pub struct EvalClassDecl {
    pub data: super::eval_ref::ClassDeclData,
}

impl RustValue for EvalClassDecl {
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::Class;

        let class = Class {
            name: self.data.name.clone(),
            field_names: self.data.field_names.clone(),
            field_defaults: self.data.field_defaults.clone(),
            methods: self.data.methods.clone(),
        };

        let class_value = Value::new_class(class);
        evaluator.current_env.define(&self.data.name, class_value);
        Ok(Ok(Value::Unit))
    }

    fn str(&self) -> String {
        let fields_str = self.data.field_names.join(", ");
        format!("class {}({})", self.data.name, fields_str)
    }
}
