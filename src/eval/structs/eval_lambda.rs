use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

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
            self.data.body.clone(),
            evaluator.current_env.clone(),
        );

        Ok(Ok(Value::from_rust(lambda_fn)))
    }

    fn str(&self) -> String {
        let params_str = self.data.params.join(", ");
        format!("fn({}) {}", params_str, self.data.body.str())
    }
}
