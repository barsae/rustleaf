use super::common::parse_source;
use rustleaf::Visibility;

/// Tests for declaration parsing (variables, functions, classes, imports)

#[test]
fn parser_simple_variable_declaration() {
    let ast = parse_source("var x = 42;").expect("Should parse variable declaration");

    // Using debug string comparison technique
    let actual = format!("{:#?}", ast);

    let expected = r#"Program {
    items: [
        VariableDeclaration {
            name: "x",
            value: Some(
                Literal(
                    Integer(
                        42,
                    ),
                ),
            ),
        },
    ],
}"#;

    assert_eq!(actual, expected);
}

#[test]
fn parser_simple_function_declaration() {
    let ast =
        parse_source("fn add(x, y) { return x + y; }").expect("Should parse function declaration");

    let actual = format!("{:#?}", ast);
    let expected = r#"Program {
    items: [
        FunctionDeclaration {
            visibility: Private,
            name: "add",
            parameters: [
                Parameter {
                    name: "x",
                    default_value: None,
                    variadic: false,
                    keyword_variadic: false,
                },
                Parameter {
                    name: "y",
                    default_value: None,
                    variadic: false,
                    keyword_variadic: false,
                },
            ],
            body: Block {
                statements: [
                    ReturnStatement {
                        value: Some(
                            BinaryOp {
                                left: Identifier(
                                    "x",
                                ),
                                operator: Add,
                                right: Identifier(
                                    "y",
                                ),
                            },
                        ),
                    },
                ],
            },
        },
    ],
}"#;
    assert_eq!(actual, expected);
}

#[test]
fn parser_public_function_declaration() {
    let ast = parse_source("pub fn hello() { }").expect("Should parse public function");

    let actual = format!("{:#?}", ast);
    let expected = r#"Program {
    items: [
        FunctionDeclaration {
            visibility: Public,
            name: "hello",
            parameters: [],
            body: Block {
                statements: [],
            },
        },
    ],
}"#;
    assert_eq!(actual, expected);
}

#[test]
fn parser_class_declaration() {
    let ast = parse_source("class Point { var x = 0; var y = 0; fn distance() { } }")
        .expect("Should parse class");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ClassDeclaration {
                visibility,
                name,
                members,
                ..
            } => {
                assert_eq!(*visibility, Visibility::Private);
                assert_eq!(name, "Point");
                assert_eq!(members.len(), 3);
            }
            _ => panic!("Expected class declaration"),
        }
    }
}

#[test]
fn parser_import_statement() {
    let ast = parse_source("use std;").expect("Should parse simple import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path, &vec!["std".to_string()]);
                assert!(
                    clause.is_none(),
                    "Expected no import clause for simple import"
                );
            }
            _ => panic!("Expected import statement"),
        }
    }
}
