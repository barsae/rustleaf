use anyhow::Result;
use rustleaf::core::{Args, RustValue, Value};
use rustleaf::eval::Evaluator;
use rustleaf_macros::{rustleaf, RustLeafWrapper};

#[rustleaf]
#[derive(Debug, Clone)]
struct Point {
    pub x: f64,
    pub y: f64,
}

#[rustleaf]
impl Point {
    // Constructor (static method)
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Zero-argument instance method
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    // Single-argument instance method
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    // Mutating method
    fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
    }

    // Multiple arguments
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    // Mixed argument types
    fn set_if_positive(&mut self, x: f64, y: f64, should_set: bool) {
        if should_set && x >= 0.0 && y >= 0.0 {
            self.x = x;
            self.y = y;
        }
    }
}

// Helper struct to bridge Vec<Value> method signatures to Args interface
#[derive(Debug, Clone)]
struct RustMethodVec {
    name: &'static str,
    func: fn(Vec<Value>) -> anyhow::Result<Value>,
}

impl RustMethodVec {
    fn new(name: &'static str, func: fn(Vec<Value>) -> anyhow::Result<Value>) -> Self {
        Self { name, func }
    }
}

impl RustValue for RustMethodVec {
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
        Some("method")
    }

    fn str(&self) -> String {
        format!("<method {}>", self.name)
    }

    fn call(&self, mut args: Args) -> Result<Value> {
        // Convert Args to Vec<Value> by collecting all positional arguments
        let values = args.rest();
        (self.func)(values)
    }
}

// Helper function to register a generated method wrapper with the evaluator
fn register_method(
    evaluator: &mut Evaluator,
    name: &'static str,
    func: fn(Vec<Value>) -> anyhow::Result<Value>,
) {
    let rust_method = Value::from_rust(RustMethodVec::new(name, func));
    evaluator.globals.define(name, rust_method);
}

mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut evaluator = Evaluator::new();
        register_method(&mut evaluator, "Point", rustleaf_point_new);

        let result = evaluator.eval_str("Point(3.0, 4.0)").unwrap();
        // Should return a wrapped Point instance
        assert!(matches!(result, Value::RustValue(_)));
    }

    #[test]
    fn test_magnitude() {
        let mut evaluator = Evaluator::new();
        register_method(&mut evaluator, "Point", rustleaf_point_new);
        register_method(&mut evaluator, "magnitude", PointRef::rustleaf_magnitude);

        // Create point and call magnitude method
        evaluator.eval_str("var p = Point(3.0, 4.0);").unwrap();
        let result = evaluator.eval_str("magnitude(p)").unwrap();
        assert_eq!(result, Value::Float(5.0)); // 3-4-5 triangle
    }

    #[test]
    fn test_distance() {
        let mut evaluator = Evaluator::new();
        register_method(&mut evaluator, "Point", rustleaf_point_new);
        register_method(&mut evaluator, "distance", PointRef::rustleaf_distance);

        // Create two points and test distance
        evaluator.eval_str("var p1 = Point(0.0, 0.0);").unwrap();
        evaluator.eval_str("var p2 = Point(3.0, 4.0);").unwrap();
        let result = evaluator.eval_str("distance(p1, p2)").unwrap();
        assert_eq!(result, Value::Float(5.0));
    }

    #[test]
    fn test_scale_mutating() {
        let mut evaluator = Evaluator::new();
        register_method(&mut evaluator, "Point", rustleaf_point_new);
        register_method(&mut evaluator, "scale", PointRef::rustleaf_scale);

        // Create point and scale it
        evaluator.eval_str("var p = Point(2.0, 3.0);").unwrap();
        let result = evaluator.eval_str("scale(p, 2.0)").unwrap();
        assert_eq!(result, Value::Unit); // void method returns unit

        // The point should be modified (shared Rc<RefCell<>>)
        // Note: We can't easily test the modified values without accessing the internals
    }

    #[test]
    fn test_translate() {
        let mut evaluator = Evaluator::new();
        register_method(&mut evaluator, "Point", rustleaf_point_new);
        register_method(&mut evaluator, "translate", PointRef::rustleaf_translate);

        // Create point and translate it
        evaluator.eval_str("var p = Point(1.0, 1.0);").unwrap();
        let result = evaluator.eval_str("translate(p, 2.0, 3.0)").unwrap();
        assert_eq!(result, Value::Unit);
    }

    #[test]
    fn test_mixed_args() {
        let mut evaluator = Evaluator::new();
        register_method(&mut evaluator, "Point", rustleaf_point_new);
        register_method(
            &mut evaluator,
            "set_if_positive",
            PointRef::rustleaf_set_if_positive,
        );

        // Create point and call method with mixed argument types
        evaluator.eval_str("var p = Point(0.0, 0.0);").unwrap();
        let result = evaluator
            .eval_str("set_if_positive(p, 5.0, 6.0, true)")
            .unwrap();
        assert_eq!(result, Value::Unit);
    }
}
