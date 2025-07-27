/// Type constant for Eval variants used in macro system
use crate::core::{RustValue, Value};

#[derive(Debug, Clone, Default)]
pub struct EvalTypeConstant;

impl EvalTypeConstant {
    pub fn new() -> Self {
        Self
    }
}

impl RustValue for EvalTypeConstant {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "Function" => Some(Value::from_rust(EvalVariantConstant::new("Function"))),
            "Class" => Some(Value::from_rust(EvalVariantConstant::new("Class"))),
            "Variable" => Some(Value::from_rust(EvalVariantConstant::new("Variable"))),
            "Call" => Some(Value::from_rust(EvalVariantConstant::new("Call"))),
            "Block" => Some(Value::from_rust(EvalVariantConstant::new("Block"))),
            // Add other variants as needed
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EvalVariantConstant {
    variant_name: String,
}

impl EvalVariantConstant {
    pub fn new(variant_name: &str) -> Self {
        Self {
            variant_name: variant_name.to_string(),
        }
    }
}

impl RustValue for EvalVariantConstant {
    fn op_is(&self, other: &Value) -> anyhow::Result<Value> {
        // Check if other is an EvalNode by trying to get its "node_type" attribute
        if let Value::RustValue(rust_val_ref) = other {
            let rust_val = rust_val_ref.borrow();

            // Try to get the type from the EvalNode directly
            if let Some(Value::String(node_type)) = rust_val.get_attr("node_type") {
                return Ok(Value::Bool(node_type == self.variant_name));
            }
        }
        Ok(Value::Bool(false))
    }
}
