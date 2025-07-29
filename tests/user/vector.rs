use anyhow::Result;
use rustleaf::core::{Args, RustValue, Value};
use rustleaf_macros::{rustleaf, RustLeafWrapper};

/// A Vector2 struct that demonstrates how a library user would extend RustLeaf
/// with their own custom types that integrate seamlessly with the language.
#[derive(Debug, Clone, PartialEq)]
#[rustleaf]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[rustleaf]
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
}

impl RustValue for Vector2Ref {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn str(&self) -> String {
        format!("Vector2({}, {})", self.borrow().x, self.borrow().y)
    }

    fn type_name(&self) -> Option<&str> {
        Some("Vector2")
    }

    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "x" => Some(Value::Float(self.borrow().x)),
            "y" => Some(Value::Float(self.borrow().y)),
            "magnitude" => Some(Value::from_rust(rustleaf::core::BoundMethod::new(
                &Value::rust_value(self.dyn_clone()),
                Self::rustleaf_magnitude,
            ))),
            "dot" => Some(Value::from_rust(rustleaf::core::BoundMethod::new(
                &Value::rust_value(self.dyn_clone()),
                Self::rustleaf_dot,
            ))),
            "normalize" => Some(Value::from_rust(rustleaf::core::BoundMethod::new(
                &Value::rust_value(self.dyn_clone()),
                Self::rustleaf_normalize,
            ))),
            _ => None,
        }
    }
}

/// Constructor function for Vector2 that can be registered as a builtin
pub fn vector2_constructor(mut args: Args) -> Result<Value> {
    let (x, y) = args.two_f64("Vector2", "x", "y")?;
    Ok(Value::from_rust(Vector2Ref::new(Vector2::new(x, y))))
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
            v3.normalize();
            assert(v3.magnitude() > 0.99);
            assert(v3.magnitude() < 1.01);
        "#;

        e.eval_str(code).unwrap();
    }
}
