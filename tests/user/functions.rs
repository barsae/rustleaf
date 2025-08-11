use anyhow::Result;
use indexmap::IndexMap;
use rustleaf::core::{Args, RustValue, Value};
use rustleaf::eval::Evaluator;
use rustleaf_macros::rustleaf;

#[rustleaf]
fn fn_f64(f: f64) -> f64 {
    f
}

#[rustleaf]
fn fn_add(a: f64, b: f64) -> f64 {
    a + b
}

#[rustleaf]
fn fn_no_args() -> f64 {
    42.0
}

#[rustleaf]
fn fn_borrow(v: &Vec<Value>) -> usize {
    v.len()
}

#[rustleaf]
fn fn_mut(v: &mut Vec<Value>) {
    v.push(Value::Int(8));
}

#[rustleaf]
fn fn_dict_len(d: &IndexMap<String, Value>) -> usize {
    d.len()
}

#[rustleaf]
fn fn_dict_mut(d: &mut IndexMap<String, Value>) {
    d.insert(
        "new_key".to_string(),
        Value::String("new_value".to_string()),
    );
}

// Helper struct to bridge Vec<Value> function signatures to Args interface
#[derive(Debug, Clone)]
struct RustFunctionVec {
    name: &'static str,
    func: fn(Vec<Value>) -> anyhow::Result<Value>,
}

impl RustFunctionVec {
    fn new(name: &'static str, func: fn(Vec<Value>) -> anyhow::Result<Value>) -> Self {
        Self { name, func }
    }
}

impl RustValue for RustFunctionVec {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn type_name(&self) -> Option<&str> {
        Some("function")
    }

    fn str(&self) -> String {
        format!("<function {}>", self.name)
    }

    fn call(&self, mut args: Args) -> Result<Value> {
        // Convert Args to Vec<Value> by collecting all positional arguments
        let values = args.rest();
        (self.func)(values)
    }
}

// Helper function to register a generated wrapper with the evaluator
fn register_function(
    evaluator: &mut Evaluator,
    name: &'static str,
    func: fn(Vec<Value>) -> anyhow::Result<Value>,
) {
    let rust_fn = Value::from_rust(RustFunctionVec::new(name, func));
    evaluator.globals.define(name, rust_fn);
}

mod tests {
    use super::*;

    #[test]
    fn test_fn_f64() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "fn_f64", rustleaf_fn_f64);

        let result = evaluator.eval_str("fn_f64(1.0)").unwrap();
        assert_eq!(result, Value::Float(1.0));
    }

    #[test]
    fn test_fn_add() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "add", rustleaf_fn_add);

        let result = evaluator.eval_str("add(2.0, 3.0)").unwrap();
        assert_eq!(result, Value::Float(5.0));
    }

    #[test]
    fn test_fn_zero() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "get_answer", rustleaf_fn_no_args);

        let result = evaluator.eval_str("get_answer()").unwrap();
        assert_eq!(result, Value::Float(42.0));
    }

    #[test]
    fn test_fn_borrow() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "list_len", rustleaf_fn_borrow);

        let result = evaluator.eval_str("list_len([1, 2, 3])").unwrap();
        assert_eq!(result, Value::Int(3)); // usize converts to i64
    }

    #[test]
    fn test_fn_mut() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "push_eight", rustleaf_fn_mut);

        // Create a list variable
        evaluator.eval_str("var my_list = [1, 2, 3];").unwrap();

        // Call the mutating function
        let result = evaluator.eval_str("push_eight(my_list)").unwrap();
        assert_eq!(result, Value::Unit); // void function returns unit

        // Check that the list was modified
        let modified_list = evaluator.get("my_list").unwrap();
        let borrowed = modified_list
            .as_ref_vec_value()
            .expect("Expected list to be accessible");
        assert_eq!(borrowed.len(), 4);
        assert_eq!(borrowed[3], Value::Int(8));
    }

    #[test]
    fn test_fn_dict_len() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "dict_len", rustleaf_fn_dict_len);

        let result = evaluator
            .eval_str("dict_len({\"key1\": 1, \"key2\": 2})")
            .unwrap();
        assert_eq!(result, Value::Int(2)); // usize converts to i64
    }

    #[test]
    fn test_fn_dict_mut() {
        let mut evaluator = Evaluator::new();
        register_function(&mut evaluator, "add_new_key", rustleaf_fn_dict_mut);

        // Create a dictionary variable
        evaluator
            .eval_str("var my_dict = {\"existing\": true};")
            .unwrap();

        // Call the mutating function
        let result = evaluator.eval_str("add_new_key(my_dict)").unwrap();
        assert_eq!(result, Value::Unit); // void function returns unit

        // Check that the dict was modified
        let modified_dict = evaluator.get("my_dict").unwrap();
        let borrowed = modified_dict
            .as_ref_indexmap_string_value()
            .expect("Expected dict to be accessible");
        assert_eq!(borrowed.len(), 2);
        assert_eq!(borrowed["new_key"], Value::String("new_value".to_string()));
    }
}
