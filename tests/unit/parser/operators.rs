use super::common::{assert_debug_eq, parse_source};

/// Tests for operator parsing (binary, unary, precedence, parentheses)

#[test]
fn parser_simple_arithmetic() {
    let ast = parse_source("var result = 2 + 3;").expect("Should parse arithmetic");

    let actual = format!("{:#?}", ast);
    let expected = r#"Program {
    items: [
        VariableDeclaration {
            name: "result",
            value: Some(
                BinaryOp {
                    left: Literal(
                        Integer(
                            2,
                        ),
                    ),
                    operator: Add,
                    right: Literal(
                        Integer(
                            3,
                        ),
                    ),
                },
            ),
        },
    ],
}"#;

    assert_eq!(actual, expected);
}

#[test]
fn parser_binary_arithmetic_expressions() {
    // Addition
    let ast = parse_source("2 + 3;").expect("Should parse addition");

    let actual = format!("{:#?}", ast);
    let expected = r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Literal(
                    Integer(
                        2,
                    ),
                ),
                operator: Add,
                right: Literal(
                    Integer(
                        3,
                    ),
                ),
            },
        },
    ],
}"#;
    assert_eq!(actual, expected);

    // Subtraction
    let ast = parse_source("10 - 5;").expect("Should parse subtraction");
    let actual = format!("{:#?}", ast);
    assert!(actual.contains("Subtract"), "Expected subtraction operator");

    // Multiplication
    let ast = parse_source("4 * 5;").expect("Should parse multiplication");
    let actual = format!("{:#?}", ast);
    assert!(
        actual.contains("Multiply"),
        "Expected multiplication operator"
    );

    // Division
    let ast = parse_source("20 / 4;").expect("Should parse division");
    let actual = format!("{:#?}", ast);
    assert!(actual.contains("Divide"), "Expected division operator");

    // Modulo
    let ast = parse_source("10 % 3;").expect("Should parse modulo");
    let actual = format!("{:#?}", ast);
    assert!(actual.contains("Modulo"), "Expected modulo operator");

    // Power
    let ast = parse_source("2 ** 3;").expect("Should parse power");
    let actual = format!("{:#?}", ast);
    assert!(actual.contains("Power"), "Expected power operator");
}

#[test]
fn parser_binary_comparison_expressions() {
    // Equal
    let ast = parse_source("x == y;").expect("Should parse equality");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: Equal,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Not equal
    let ast = parse_source("x != y;").expect("Should parse inequality");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: NotEqual,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Less than
    let ast = parse_source("x < y;").expect("Should parse less than");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: Less,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Greater than
    let ast = parse_source("x > y;").expect("Should parse greater than");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: Greater,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Less than or equal
    let ast = parse_source("x <= y;").expect("Should parse less than or equal");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: LessEqual,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Greater than or equal
    let ast = parse_source("x >= y;").expect("Should parse greater than or equal");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: GreaterEqual,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_binary_logical_expressions() {
    // Logical AND
    let ast = parse_source("x and y;").expect("Should parse logical AND");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: And,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Logical OR
    let ast = parse_source("x or y;").expect("Should parse logical OR");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: Or,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Logical XOR
    let ast = parse_source("x xor y;").expect("Should parse logical XOR");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: Xor,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_binary_bitwise_expressions() {
    // Bitwise AND
    let ast = parse_source("x & y;").expect("Should parse bitwise AND");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: BitwiseAnd,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Bitwise OR
    let ast = parse_source("x | y;").expect("Should parse bitwise OR");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: BitwiseOr,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Bitwise XOR
    let ast = parse_source("x ^ y;").expect("Should parse bitwise XOR");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: BitwiseXor,
                right: Identifier(
                    "y",
                ),
            },
        },
    ],
}"#,
    );

    // Left shift
    let ast = parse_source("x << 2;").expect("Should parse left shift");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: LeftShift,
                right: Literal(
                    Integer(
                        2,
                    ),
                ),
            },
        },
    ],
}"#,
    );

    // Right shift
    let ast = parse_source("x >> 2;").expect("Should parse right shift");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Identifier(
                    "x",
                ),
                operator: RightShift,
                right: Literal(
                    Integer(
                        2,
                    ),
                ),
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_unary_expressions() {
    // Unary plus
    let ast = parse_source("+x;").expect("Should parse unary plus");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: UnaryOp {
                operator: Plus,
                operand: Identifier(
                    "x",
                ),
            },
        },
    ],
}"#,
    );

    // Unary minus
    let ast = parse_source("-x;").expect("Should parse unary minus");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: UnaryOp {
                operator: Minus,
                operand: Identifier(
                    "x",
                ),
            },
        },
    ],
}"#,
    );

    // Logical NOT
    let ast = parse_source("not x;").expect("Should parse logical NOT");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: UnaryOp {
                operator: Not,
                operand: Identifier(
                    "x",
                ),
            },
        },
    ],
}"#,
    );

    // Bitwise NOT
    let ast = parse_source("~x;").expect("Should parse bitwise NOT");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: UnaryOp {
                operator: BitwiseNot,
                operand: Identifier(
                    "x",
                ),
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_operator_precedence() {
    // Multiplication has higher precedence than addition
    let ast = parse_source("2 + 3 * 4;").expect("Should parse with correct precedence");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Literal(
                    Integer(
                        2,
                    ),
                ),
                operator: Add,
                right: BinaryOp {
                    left: Literal(
                        Integer(
                            3,
                        ),
                    ),
                    operator: Multiply,
                    right: Literal(
                        Integer(
                            4,
                        ),
                    ),
                },
            },
        },
    ],
}"#,
    );

    // Exponentiation has higher precedence than multiplication
    let ast = parse_source("2 * 3 ** 4;").expect("Should parse power precedence");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: Literal(
                    Integer(
                        2,
                    ),
                ),
                operator: Multiply,
                right: BinaryOp {
                    left: Literal(
                        Integer(
                            3,
                        ),
                    ),
                    operator: Power,
                    right: Literal(
                        Integer(
                            4,
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
fn parser_parenthesized_expressions() {
    let ast = parse_source("(2 + 3) * 4;").expect("Should parse parenthesized expression");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: BinaryOp {
                left: BinaryOp {
                    left: Literal(
                        Integer(
                            2,
                        ),
                    ),
                    operator: Add,
                    right: Literal(
                        Integer(
                            3,
                        ),
                    ),
                },
                operator: Multiply,
                right: Literal(
                    Integer(
                        4,
                    ),
                ),
            },
        },
    ],
}"#,
    );
}
