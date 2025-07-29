use anyhow::{anyhow, Result};
use rustleaf::core::{Args, BoundMethod, RustValue, Value};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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

        let vector_ref = self_value
            .downcast_rust_value::<Vector2Ref>()
            .ok_or_else(|| anyhow!("magnitude() called on non-Vector2 value"))?;

        let magnitude = vector_ref.borrow().magnitude();
        Ok(Value::Float(magnitude))
    }

    pub fn rustleaf_dot(self_value: &Value, mut args: Args) -> Result<Value> {
        args.set_function_name("dot");
        let other_val = args.expect("other")?;
        args.complete()?;

        let vector_ref = self_value
            .downcast_rust_value::<Vector2Ref>()
            .ok_or_else(|| anyhow!("dot() called on non-Vector2 value"))?;

        let result = if let Some(other_ref) = other_val.downcast_rust_value::<Vector2Ref>() {
            let dot_result = vector_ref.borrow().dot(&*other_ref.borrow());
            Ok(Value::Float(dot_result))
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

        let vector_ref = self_value
            .downcast_rust_value::<Vector2Ref>()
            .ok_or_else(|| anyhow!("normalize() called on non-Vector2 value"))?;

        vector_ref.borrow_mut().normalize();
        Ok(Value::Unit)
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

/// Vector2Ref wraps Rc<RefCell<Vector2>> and implements RustValue
#[derive(Debug, Clone)]
pub struct Vector2Ref(Rc<RefCell<Vector2>>);

impl Vector2Ref {
    pub fn new(vector: Vector2) -> Self {
        Self(Rc::new(RefCell::new(vector)))
    }

    pub fn borrow(&self) -> std::cell::Ref<Vector2> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<Vector2> {
        self.0.borrow_mut()
    }
}

#[rustleaf::rust_value_any]
impl RustValue for Vector2Ref {
    fn dyn_clone(&self) -> Box<dyn RustValue> {
        Box::new(self.clone())
    }

    fn get_attr(&self, name: &str) -> Option<Value> {
        let borrowed = self.borrow();
        match name {
            "x" => Some(Value::Float(borrowed.x)),
            "y" => Some(Value::Float(borrowed.y)),
            "magnitude" => {
                drop(borrowed); // Release borrow before cloning
                Some(Value::from_rust(BoundMethod::new(
                    &Value::rust_value(self.dyn_clone()),
                    Vector2::rustleaf_magnitude,
                )))
            }
            "dot" => {
                drop(borrowed); // Release borrow before cloning
                Some(Value::from_rust(BoundMethod::new(
                    &Value::rust_value(self.dyn_clone()),
                    Vector2::rustleaf_dot,
                )))
            }
            "normalize" => {
                drop(borrowed); // Release borrow before cloning
                Some(Value::from_rust(BoundMethod::new(
                    &Value::rust_value(self.dyn_clone()),
                    Vector2::rustleaf_normalize,
                )))
            }
            _ => None,
        }
    }

    fn str(&self) -> String {
        format!("Vector2({}, {})", self.borrow().x, self.borrow().y)
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
