use super::common::{assert_debug_eq, parse_source};

/// Tests for expression parsing (property access, function calls, assignments, etc.)

#[test]
fn parser_assignment_expression() {
    let ast = parse_source("x = 42;").expect("Should parse assignment");

    let actual = format!("{:#?}", ast);
    let expected = r#"Program {
    items: [
        ExpressionStatement {
            expression: Assignment {
                target: Identifier(
                    "x",
                ),
                operator: Assign,
                value: Literal(
                    Integer(
                        42,
                    ),
                ),
            },
        },
    ],
}"#;
    assert_eq!(actual, expected);
}

#[test]
fn parser_property_access() {
    let ast = parse_source("object.property;").expect("Should parse property access");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: PropertyAccess {
                object: Identifier(
                    "object",
                ),
                property: "property",
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_chained_property_access() {
    let ast = parse_source("a.b.c;").expect("Should parse chained property access");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: PropertyAccess {
                object: PropertyAccess {
                    object: Identifier(
                        "a",
                    ),
                    property: "b",
                },
                property: "c",
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_index_access() {
    let ast = parse_source("array[0];").expect("Should parse index access");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: IndexAccess {
                object: Identifier(
                    "array",
                ),
                index: Literal(
                    Integer(
                        0,
                    ),
                ),
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_function_call() {
    let ast = parse_source("func();").expect("Should parse function call");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: FunctionCall {
                function: Identifier(
                    "func",
                ),
                arguments: [],
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_function_call_with_arguments() {
    let ast = parse_source("add(1, 2);").expect("Should parse function call with arguments");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: FunctionCall {
                function: Identifier(
                    "add",
                ),
                arguments: [
                    Argument {
                        value: Literal(
                            Integer(
                                1,
                            ),
                        ),
                        spread: false,
                        keyword_spread: false,
                    },
                    Argument {
                        value: Literal(
                            Integer(
                                2,
                            ),
                        ),
                        spread: false,
                        keyword_spread: false,
                    },
                ],
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_complex_nested_expression() {
    let ast = parse_source("func(a[0].prop, b + c * d);").expect("Should parse complex expression");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: FunctionCall {
                function: Identifier(
                    "func",
                ),
                arguments: [
                    Argument {
                        value: PropertyAccess {
                            object: IndexAccess {
                                object: Identifier(
                                    "a",
                                ),
                                index: Literal(
                                    Integer(
                                        0,
                                    ),
                                ),
                            },
                            property: "prop",
                        },
                        spread: false,
                        keyword_spread: false,
                    },
                    Argument {
                        value: BinaryOp {
                            left: Identifier(
                                "b",
                            ),
                            operator: Add,
                            right: BinaryOp {
                                left: Identifier(
                                    "c",
                                ),
                                operator: Multiply,
                                right: Identifier(
                                    "d",
                                ),
                            },
                        },
                        spread: false,
                        keyword_spread: false,
                    },
                ],
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_closure_basic() {
    let ast = parse_source("|x| x * 2;").expect("Should parse basic closure");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: AnonymousFunction {
                parameters: [
                    Parameter {
                        name: "x",
                        default_value: None,
                        variadic: false,
                        keyword_variadic: false,
                    },
                ],
                body: BinaryOp {
                    left: Identifier(
                        "x",
                    ),
                    operator: Multiply,
                    right: Literal(
                        Integer(
                            2,
                        ),
                    ),
                },
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_closure_empty_params() {
    let ast = parse_source("|| 42;").expect("Should parse empty closure");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: AnonymousFunction {
                parameters: [],
                body: Literal(
                    Integer(
                        42,
                    ),
                ),
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_closure_multiple_params() {
    let ast = parse_source("|a, b| a + b;").expect("Should parse closure with multiple params");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: AnonymousFunction {
                parameters: [
                    Parameter {
                        name: "a",
                        default_value: None,
                        variadic: false,
                        keyword_variadic: false,
                    },
                    Parameter {
                        name: "b",
                        default_value: None,
                        variadic: false,
                        keyword_variadic: false,
                    },
                ],
                body: BinaryOp {
                    left: Identifier(
                        "a",
                    ),
                    operator: Add,
                    right: Identifier(
                        "b",
                    ),
                },
            },
        },
    ],
}"#,
    );
}
