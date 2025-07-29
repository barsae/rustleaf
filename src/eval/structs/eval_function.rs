use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalFunction {
    pub data: super::eval_ref::FunctionData,
}

impl RustValue for EvalFunction {
    crate::impl_rust_value_any!(Self);
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        use crate::eval::{Params, RustLeafFunction};

        let params = Params::from_vec(self.data.params.clone());
        let function = RustLeafFunction::new(
            params,
            self.data.body.clone(),
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
                Some(Value::RustValue(self.data.body.clone()))
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
            self.data.body.str()
        )
    }
}
