use rustleaf::core::RustValue;
use rustleaf_macros::{rustleaf, RustLeafWrapper};

/// A Vector2 struct that demonstrates how a library user would extend RustLeaf
/// with their own custom types that integrate seamlessly with the language.
#[derive(Debug, Clone, PartialEq)]
// Generates the wrapper type needed to be passed in and out of the scripting langauge
#[rustleaf]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
// Generates the wrapper methods (and dispatch) needed to expose methods to the scripting language
#[rustleaf]
impl Vector2 {
    // The wrapping method gets named `rustleaf_new`, for registering with the evaluator
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    // Immutable, no args
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Immutable, "other"-arg
    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    // Mutable self-arg
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag > 0.0 {
            self.x /= mag;
            self.y /= mag;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustleaf::eval::Evaluator;

    #[test]
    fn test_vector2_user_experience() {
        let mut e = Evaluator::new();
        // Register our type with the evaluator, exposing it's properties, methods, etc.
        e.register_builtin_fn("Vector2", Vector2::rustleaf_new);

        // Test that we can create vectors and use their methods in the scripting language
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
