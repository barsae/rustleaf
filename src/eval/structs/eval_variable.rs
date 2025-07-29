use crate::core::RustValue;
use crate::eval::{ControlFlow, ErrorKind, EvalResult, Evaluator};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct EvalVariable {
    pub name: String,
}

#[crate::rust_value_any]
impl RustValue for EvalVariable {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
    fn eval(&self, evaluator: &mut Evaluator) -> anyhow::Result<EvalResult> {
        let result = evaluator.current_env.get(&self.name).ok_or_else(|| {
            ControlFlow::Error(ErrorKind::SystemError(anyhow!(
                "Undefined variable: {}",
                self.name
            )))
        });
        Ok(result)
    }

    fn str(&self) -> String {
        self.name.clone()
    }
}
