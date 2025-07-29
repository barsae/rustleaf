use anyhow::Result;
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
    pub fn rustleaf_magnitude(self_value: &Value, args: Args) -> Result<Value> {
        self_value.with_rust_value_no_args::<Vector2Ref, f64, _>(
            "magnitude",
            "Vector2",
            args,
            |vector_ref| Ok(vector_ref.borrow().magnitude()),
        )
    }

    pub fn rustleaf_dot(self_value: &Value, args: Args) -> Result<Value> {
        self_value.with_rust_value_same_type::<Vector2Ref, f64, _>(
            "dot",
            "Vector2",
            "other",
            args,
            |vector_ref, other_ref| Ok(vector_ref.borrow().dot(&*other_ref.borrow())),
        )
    }

    pub fn rustleaf_normalize(self_value: &Value, args: Args) -> Result<Value> {
        self_value.with_rust_value_no_args::<Vector2Ref, (), _>(
            "normalize",
            "Vector2",
            args,
            |vector_ref| {
                vector_ref.borrow_mut().normalize();
                Ok(())
            },
        )
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
