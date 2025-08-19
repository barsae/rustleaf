use rustleaf::Evaluator;
use rustleaf::Value;
use std::fs;

/// Test demonstrating the ergonomics of a Rust program that loads a user-specified
/// script and calls a function from it. This pattern shows how RustLeaf can be
/// embedded in larger applications where users provide scripts with specific
/// function contracts.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_loading_and_function_call() {
        // Simulate a Rust application loading the script and calling the function
        let mut evaluator = Evaluator::new();

        // Load the script file
        let script_path = "tests/user/polynomial.rustleaf";
        let script_source = fs::read_to_string(script_path).expect("Failed to read script");
        evaluator
            .eval_str(&script_source)
            .expect("Failed to evaluate script");

        // Call the polynomial function with different values
        let test_cases = [
            (0.0, 1.0),  // polynomial(0) = 1
            (1.0, 6.0),  // polynomial(1) = 2 + 3 + 1 = 6
            (2.0, 15.0), // polynomial(2) = 8 + 6 + 1 = 15
            (-1.0, 0.0), // polynomial(-1) = 2 - 3 + 1 = 0
        ];

        // Test the new ergonomic API with multiple test cases
        for (input, expected) in test_cases {
            let polynomial = evaluator
                .get("polynomial")
                .expect("Failed to get polynomial function");
            let result = evaluator
                .call(polynomial, &[Value::Float(input)])
                .expect("Failed to call polynomial");
            let y = result.as_f64().expect("Expected float result");

            assert!(
                (y - expected).abs() < 1e-10,
                "polynomial({input}) = {y}, expected {expected}"
            );
        }
    }
}
