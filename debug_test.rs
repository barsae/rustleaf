// Quick demo of debug string testing technique
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct TestStruct {
    name: String,
    value: i32,
    nested: Option<Box<TestStruct>>,
}

fn main() {
    let test_data = TestStruct {
        name: "test".to_string(),
        value: 42,
        nested: Some(Box::new(TestStruct {
            name: "nested".to_string(),
            value: 99,
            nested: None,
        })),
    };
    
    // Show what debug printing looks like
    println!("Debug format:");
    println!("{:#?}", test_data);
    
    // This is what we'd use in tests
    let actual = format!("{:#?}", test_data);
    let expected = r#"TestStruct {
    name: "test",
    value: 42,
    nested: Some(
        TestStruct {
            name: "nested",
            value: 99,
            nested: None,
        },
    ),
}"#;
    
    assert_eq!(actual, expected);
    println!("Debug string assertion passed!");
}