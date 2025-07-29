use crate::core::{Args, RustValue, Value};
use anyhow::anyhow;

#[derive(Debug, Clone)]
pub struct TypeConstant {
    type_name: String,
}

impl TypeConstant {
    pub fn new(type_name: &str) -> Self {
        Self {
            type_name: type_name.to_string(),
        }
    }
}

#[crate::rust_value_any]
impl RustValue for TypeConstant {
    fn call(&self, _args: Args) -> anyhow::Result<Value> {
        Err(anyhow!("Type constants are not callable"))
    }

    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "name" => Some(Value::String(self.type_name.clone())),
            _ => None,
        }
    }

    fn op_is(&self, other: &Value) -> anyhow::Result<Value> {
        // Type checking: check if other value matches this type
        let matches = match self.type_name.as_str() {
            "Null" => matches!(other, Value::Null),
            "Unit" => matches!(other, Value::Unit),
            "Bool" => matches!(other, Value::Bool(_)),
            "Int" => matches!(other, Value::Int(_)),
            "Float" => matches!(other, Value::Float(_)),
            "String" => matches!(other, Value::String(_)),
            "List" => matches!(other, Value::List(_)),
            "Dict" => matches!(other, Value::Dict(_)),
            "Range" => matches!(other, Value::Range(_)),
            "Function" => matches!(other, Value::RustValue(_)), // Functions are RustValues
            _ => false,
        };
        Ok(Value::Bool(matches))
    }
}
