use anyhow::{anyhow, Result};
use rustleaf::core::{Args, BoundMethod, RustValue, Value};
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

    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }

    // RustLeaf wrapper methods - handle Args parsing and Value conversion
    pub fn rustleaf_magnitude(self_value: &Value, mut args: Args) -> Result<Value> {
        args.set_function_name("magnitude");
        args.complete()?;

        let vector = self_value
            .downcast_rust_value::<Vector2>()
            .ok_or_else(|| anyhow!("magnitude() called on non-Vector2 value"))?;

        Ok(Value::Float(vector.magnitude()))
    }

    pub fn rustleaf_dot(self_value: &Value, mut args: Args) -> Result<Value> {
        args.set_function_name("dot");
        let other_val = args.expect("other")?;
        args.complete()?;

        let vector = self_value
            .downcast_rust_value::<Vector2>()
            .ok_or_else(|| anyhow!("dot() called on non-Vector2 value"))?;

        let result = if let Some(other_vec_ref) = other_val.downcast_rust_value::<Vector2>() {
            Ok(Value::Float(vector.dot(&*other_vec_ref)))
        } else {
            Err(anyhow!(
                "dot() requires another Vector2, got {:?}",
                other_val
            ))
        };
        result
    }

    pub fn rustleaf_normalize(self_value: &Value, mut args: Args) -> Result<Value> {
        args.set_function_name("normalize");
        args.complete()?;

        let mut vector = self_value
            .downcast_rust_value_mut::<Vector2>()
            .ok_or_else(|| anyhow!("normalize() called on non-Vector2 value"))?;

        vector.normalize();
        Ok(Value::Unit)
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
            "magnitude" => Some(Value::from_rust(BoundMethod::new(
                &Value::from_rust(self.clone()),
                Vector2::rustleaf_magnitude,
            ))),
            "dot" => Some(Value::from_rust(BoundMethod::new(
                &Value::from_rust(self.clone()),
                Vector2::rustleaf_dot,
            ))),
            "normalize" => Some(Value::from_rust(BoundMethod::new(
                &Value::from_rust(self.clone()),
                Vector2::rustleaf_normalize,
            ))),
            _ => None,
        }
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
        let mut e = Evaluator::new();
        register_vector2(&mut e);

        // Test that we can create vectors and use their methods
        let code = r#"
            var v1 = Vector2(3.0, 4.0);
            var v2 = Vector2(1.0, 2.0);

            assert(v1.x == 3.0);
            assert(v1.y == 4.0);
            assert(v1.magnitude() == 5.0);
            assert(v1.dot(v2) == 11.0);
            var v3 = Vector2(3.0, 4.0);
            // v3.normalize();
            // assert(v3.magnitude() > 0.99);
            // assert(v3.magnitude() < 1.01);
        "#;

        e.eval_str(code).unwrap();
    }
}
