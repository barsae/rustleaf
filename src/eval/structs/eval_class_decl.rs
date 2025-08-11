use crate::core::{RustValue, Value};
use crate::eval::{EvalResult, Evaluator};

#[derive(Debug, Clone)]
pub struct EvalClassDecl {
    pub data: super::eval_ref::ClassDeclData,
}

#[crate::rust_value_any]
impl RustValue for EvalClassDecl {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
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
