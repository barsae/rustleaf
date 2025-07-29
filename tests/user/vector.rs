use anyhow::{anyhow, Result};
use rustleaf::core::{Args, RustValue, Value};
use rustleaf::rust_value_any;
use std::fmt;

/// A Vector2 struct that demonstrates how a library user would extend RustLeaf
/// with their own custom types that integrate seamlessly with the language.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Core logic methods - pure Rust implementations
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    // RustLeaf wrapper methods - handle Args parsing and Value conversion
    pub fn rustleaf_magnitude(&self, mut args: Args) -> Result<Value> {
        args.set_function_name("magnitude");
        args.complete()?;
        Ok(Value::Float(self.magnitude()))
    }

    pub fn rustleaf_dot(&self, mut args: Args) -> Result<Value> {
        args.set_function_name("dot");
        let other_val = args.expect("other")?;
        args.complete()?;

        let result = if let Some(other_vec_ref) = other_val.downcast_rust_value::<Vector2>() {
            Ok(Value::Float(self.dot(&*other_vec_ref)))
        } else {
            Err(anyhow!(
                "dot() requires another Vector2, got {:?}",
                other_val
            ))
        };
        result
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

/// Implementation of RustValue trait allows Vector2 to be used directly in RustLeaf
#[rustleaf::rust_value_any]
impl RustValue for Vector2 {
    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "x" => Some(Value::Float(self.x)),
            "y" => Some(Value::Float(self.y)),
            "magnitude" => Some(Value::from_rust(
                VectorMethod::new(Vector2::rustleaf_magnitude).bind(self.clone()),
            )),
            "dot" => Some(Value::from_rust(
                VectorMethod::new(Vector2::rustleaf_dot).bind(self.clone()),
            )),
            _ => None,
        }
    }
}

/// Helper struct for Vector2 methods
#[derive(Debug, Clone)]
pub struct VectorMethod {
    instance: Vector2,
    func: fn(&Vector2, Args) -> Result<Value>,
}

impl VectorMethod {
    pub fn new(func: fn(&Vector2, Args) -> Result<Value>) -> Self {
        // This constructor is used to create an unbound method template
        // The actual bound method will be created with `bind`
        Self {
            instance: Vector2::new(0.0, 0.0), // Placeholder
            func,
        }
    }

    pub fn bind(&self, instance: Vector2) -> Self {
        Self {
            instance,
            func: self.func,
        }
    }
}

#[rust_value_any]
impl RustValue for VectorMethod {
    fn call(&self, args: Args) -> Result<Value> {
        (self.func)(&self.instance, args)
    }
}

/// Constructor function for Vector2 that can be registered as a builtin
pub fn vector2_constructor(mut args: Args) -> Result<Value> {
    args.set_function_name("Vector2");
    let x_val = args.expect("x")?;
    let y_val = args.expect("y")?;
    args.complete()?;

    let x = x_val
        .as_f64()
        .ok_or_else(|| anyhow!("Vector2 x coordinate must be a number, got {:?}", x_val))?;

    let y = y_val
        .as_f64()
        .ok_or_else(|| anyhow!("Vector2 y coordinate must be a number, got {:?}", y_val))?;

    Ok(Value::from_rust(Vector2::new(x, y)))
}

/// Function to register Vector2 type with a RustLeaf evaluator
pub fn register_vector2(evaluator: &mut rustleaf::eval::Evaluator) {
    evaluator.register_builtin_fn("Vector2", vector2_constructor);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustleaf::eval::Evaluator;

    #[test]
    fn test_vector2_user_experience() {
        let mut evaluator = Evaluator::new();
        register_vector2(&mut evaluator);

        // Test that we can create vectors
        let _code = r#"
            let v1 = Vector2(3.0, 4.0);
            let v2 = Vector2(1.0, 2.0);
        "#;

        // In a real implementation, we would:
        // 1. Parse and compile the code
        // 2. Evaluate it with our custom evaluator
        // 3. Test that vector operations work as expected
        //
        // For now, this demonstrates the API design

        // Test basic properties
        let v1 = Vector2::new(3.0, 4.0);
        assert_eq!(v1.x, 3.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.magnitude(), 5.0);

        // Test dot product
        let v2 = Vector2::new(1.0, 2.0);
        let dot_product = v1.dot(&v2);
        assert_eq!(dot_product, 11.0); // 3*1 + 4*2 = 11
    }
}
