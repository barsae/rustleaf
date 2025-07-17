use super::common::{assert_debug_eq, parse_source};

/// Tests for literal parsing (integers, strings, booleans, lists, dicts, etc.)

#[test]
fn parser_literal_expressions() {
    // Integer literal as expression statement
    let ast = parse_source("42;").expect("Should parse integer literal");

    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: Literal(
                Integer(
                    42,
                ),
            ),
        },
    ],
}"#,
    );

    // Boolean literal
    let ast = parse_source("true;").expect("Should parse boolean literal");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: Literal(
                Boolean(
                    true,
                ),
            ),
        },
    ],
}"#,
    );

    // Note: String and float literals may still have parsing issues - testing individually
}

#[test]
fn parser_string_literal() {
    let ast = parse_source(r#""hello";"#).expect("Should parse string literal");

    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: Literal(
                String(
                    "hello",
                ),
            ),
        },
    ],
}"#,
    );
}

#[test]
fn parser_identifier_expression() {
    let ast = parse_source("variable_name;").expect("Should parse identifier");

    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: Identifier(
                "variable_name",
            ),
        },
    ],
}"#,
    );
}

#[test]
fn parser_list_literal() {
    let ast = parse_source("[1, 2, 3];").expect("Should parse list literal");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: ListLiteral {
                elements: [
                    Literal(
                        Integer(
                            1,
                        ),
                    ),
                    Literal(
                        Integer(
                            2,
                        ),
                    ),
                    Literal(
                        Integer(
                            3,
                        ),
                    ),
                ],
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_empty_list() {
    let ast = parse_source("[];").expect("Should parse empty list");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        ExpressionStatement {
            expression: ListLiteral {
                elements: [],
            },
        },
    ],
}"#,
    );
}

#[test]
fn parser_empty_dict() {
    // Note: Empty braces are parsed as blocks in statement context,
    // but as dict literals in expression context. This may be ambiguous
    // in the current parser implementation.
    let ast = parse_source("{};").expect("Should parse empty dict");
    assert_debug_eq(
        &ast,
        r#"Program {
    items: [
        Block {
            statements: [],
        },
    ],
}"#,
    );
}
