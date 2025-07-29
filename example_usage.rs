// Example usage of RustLeaf as a library

use rustleaf;

fn main() -> anyhow::Result<()> {
    // Using eval_str for quick evaluation
    let result = rustleaf::eval_str("2 + 3 * 4")?;
    println!("Result: {:?}", result); // Should print: Result: Int(14)

    // Using eval_str with a more complex example
    let code = r#"
        fn factorial(n) {
            if n <= 1 {
                1
            } else {
                n * factorial(n - 1)
            }
        }
        factorial(5)
    "#;
    
    let result = rustleaf::eval_str(code)?;
    println!("Factorial(5): {:?}", result); // Should print: Factorial(5): Int(120)

    // Example of using eval_file (commented out since we don't have a test file)
    // let result = rustleaf::eval_file("examples/hello.rl")?;
    // println!("File result: {:?}", result);

    Ok(())
}