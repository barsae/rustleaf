use crate::value::value::{RustValue, Value};

// Example implementation of RustValue for demonstration
#[derive(Debug, Clone)]
pub struct ExampleRustValue {
    pub data: String,
}

impl RustValue for ExampleRustValue {
    fn type_name(&self) -> &'static str {
        "ExampleRustValue"
    }

    fn to_string(&self) -> String {
        format!("ExampleRustValue({})", self.data)
    }

    fn clone_box(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }
}

// This would be the main trait that Rust types implement to be usable in RustLeaf
pub trait IntoRustLeafValue {
    fn into_rustleaf_value(self) -> Value;
}

// Example implementations
impl IntoRustLeafValue for String {
    fn into_rustleaf_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoRustLeafValue for i32 {
    fn into_rustleaf_value(self) -> Value {
        Value::Int(self as i64)
    }
}

impl IntoRustLeafValue for i64 {
    fn into_rustleaf_value(self) -> Value {
        Value::Int(self)
    }
}

impl IntoRustLeafValue for f64 {
    fn into_rustleaf_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoRustLeafValue for bool {
    fn into_rustleaf_value(self) -> Value {
        Value::Bool(self)
    }
}