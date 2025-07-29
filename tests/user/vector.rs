use anyhow::{anyhow, Result};
use rustleaf::core::{Args, RustValue, Value};
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

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2 {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vector2::new(0.0, 0.0)
        } else {
            Vector2::new(self.x / mag, self.y / mag)
        }
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn add(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    pub fn sub(&self, other: &Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }

    pub fn scale(&self, scalar: f64) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2({}, {})", self.x, self.y)
    }
}

/// Implementation of RustValue trait allows Vector2 to be used directly in RustLeaf
impl RustValue for Vector2 {
    rustleaf::impl_rust_value_any!(Vector2);

    fn get_attr(&self, name: &str) -> Option<Value> {
        match name {
            "x" => Some(Value::Float(self.x)),
            "y" => Some(Value::Float(self.y)),
            "magnitude" => Some(Value::from_rust(
                VectorMethod::new("magnitude", |vec, mut args| {
                    args.set_function_name("magnitude");
                    args.complete()?;
                    Ok(Value::Float(vec.magnitude()))
                })
                .bind(self.clone()),
            )),
            "normalize" => Some(Value::from_rust(
                VectorMethod::new("normalize", |vec, mut args| {
                    args.set_function_name("normalize");
                    args.complete()?;
                    Ok(Value::from_rust(vec.normalize()))
                })
                .bind(self.clone()),
            )),
            "dot" => Some(Value::from_rust(
                VectorMethod::new("dot", |vec, mut args| {
                    args.set_function_name("dot");
                    let other_val = args.expect("other")?;
                    args.complete()?;

                    let result =
                        if let Some(other_vec_ref) = other_val.downcast_rust_value::<Vector2>() {
                            let result = vec.dot(&*other_vec_ref);
                            Ok(Value::Float(result))
                        } else {
                            Err(anyhow!(
                                "dot() requires another Vector2, got {:?}",
                                other_val
                            ))
                        };
                    result
                })
                .bind(self.clone()),
            )),
            "op_add" => Some(Value::from_rust(
                VectorMethod::new("op_add", |vec, mut args| {
                    args.set_function_name("op_add");
                    let other_val = args.expect("other")?;
                    args.complete()?;

                    let result =
                        if let Some(other_vec_ref) = other_val.downcast_rust_value::<Vector2>() {
                            let result = vec.add(&*other_vec_ref);
                            Ok(Value::from_rust(result))
                        } else {
                            Err(anyhow!("Cannot add {:?} to Vector2", other_val))
                        };
                    result
                })
                .bind(self.clone()),
            )),
            "op_sub" => Some(Value::from_rust(
                VectorMethod::new("op_sub", |vec, mut args| {
                    args.set_function_name("op_sub");
                    let other_val = args.expect("other")?;
                    args.complete()?;

                    let result =
                        if let Some(other_vec_ref) = other_val.downcast_rust_value::<Vector2>() {
                            let result = vec.sub(&*other_vec_ref);
                            Ok(Value::from_rust(result))
                        } else {
                            Err(anyhow!("Cannot subtract {:?} from Vector2", other_val))
                        };
                    result
                })
                .bind(self.clone()),
            )),
            "op_mul" => Some(Value::from_rust(
                VectorMethod::new("op_mul", |vec, mut args| {
                    args.set_function_name("op_mul");
                    let other_val = args.expect("scalar")?;
                    args.complete()?;

                    if let Some(scalar) = other_val.as_f64() {
                        Ok(Value::from_rust(vec.scale(scalar)))
                    } else {
                        Err(anyhow!(
                            "Can only multiply Vector2 by numbers, got {:?}",
                            other_val
                        ))
                    }
                })
                .bind(self.clone()),
            )),
            "op_eq" => Some(Value::from_rust(
                VectorMethod::new("op_eq", |vec, mut args| {
                    args.set_function_name("op_eq");
                    let other_val = args.expect("other")?;
                    args.complete()?;

                    let result =
                        if let Some(other_vec_ref) = other_val.downcast_rust_value::<Vector2>() {
                            let result = vec == &*other_vec_ref;
                            Ok(Value::Bool(result))
                        } else {
                            Ok(Value::Bool(false)) // Different types are not equal
                        };
                    result
                })
                .bind(self.clone()),
            )),
            _ => None,
        }
    }
}

/// Helper struct for Vector2 methods  
#[derive(Debug, Clone)]
pub struct VectorMethod {
    instance: Vector2,
    name: &'static str,
    func: fn(&Vector2, Args) -> Result<Value>,
}

impl VectorMethod {
    pub fn new(name: &'static str, func: fn(&Vector2, Args) -> Result<Value>) -> Self {
        // This constructor is used to create an unbound method template
        // The actual bound method will be created with `bind`
        Self {
            instance: Vector2::new(0.0, 0.0), // Placeholder
            name,
            func,
        }
    }

    pub fn bind(&self, instance: Vector2) -> Self {
        Self {
            instance,
            name: self.name,
            func: self.func,
        }
    }
}

impl RustValue for VectorMethod {
    rustleaf::impl_rust_value_any!(VectorMethod);

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

        // Test operations
        let v2 = Vector2::new(1.0, 2.0);
        let sum = v1.add(&v2);
        assert_eq!(sum, Vector2::new(4.0, 6.0));

        let scaled = v1.scale(2.0);
        assert_eq!(scaled, Vector2::new(6.0, 8.0));

        let dot_product = v1.dot(&v2);
        assert_eq!(dot_product, 11.0); // 3*1 + 4*2 = 11
    }

    #[test]
    fn test_vector2_rustvalue_integration() {
        let v = Vector2::new(3.0, 4.0);

        // Test attribute access
        assert_eq!(v.get_attr("x"), Some(Value::Float(3.0)));
        assert_eq!(v.get_attr("y"), Some(Value::Float(4.0)));

        // Test that methods are available
        assert!(v.get_attr("magnitude").is_some());
        assert!(v.get_attr("normalize").is_some());
        assert!(v.get_attr("dot").is_some());
        assert!(v.get_attr("op_add").is_some());
        assert!(v.get_attr("op_sub").is_some());
        assert!(v.get_attr("op_mul").is_some());
        assert!(v.get_attr("op_eq").is_some());
    }
}
