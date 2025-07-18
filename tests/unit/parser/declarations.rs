use super::common::parse_source;
use rustleaf::{ImportClause, ModulePathRoot, Visibility};

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
                assert_eq!(path.root_type, ModulePathRoot::Absolute);
                assert_eq!(path.segments, vec!["std".to_string()]);
                assert!(
                    clause.is_none(),
                    "Expected no import clause for simple import"
                );
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_with_path_segments() {
    let ast = parse_source("use math::geometry::Point;").expect("Should parse path import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Absolute);
                assert_eq!(
                    path.segments,
                    vec!["math".to_string(), "geometry".to_string()]
                );
                assert_eq!(*clause, Some(ImportClause::Single("Point".to_string())));
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_super() {
    let ast = parse_source("use super::sibling;").expect("Should parse super import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Super);
                assert_eq!(path.segments, vec!["sibling".to_string()]);
                assert!(clause.is_none());
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_root() {
    let ast = parse_source("use root::utils::logger;").expect("Should parse root import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Root);
                assert_eq!(path.segments, vec!["utils".to_string()]);
                assert_eq!(*clause, Some(ImportClause::Single("logger".to_string())));
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_all() {
    let ast = parse_source("use math::constants::*;").expect("Should parse import all");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Absolute);
                assert_eq!(
                    path.segments,
                    vec!["math".to_string(), "constants".to_string()]
                );
                assert_eq!(*clause, Some(ImportClause::All));
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_named() {
    let ast = parse_source("use math::geometry::{Point, Line, Circle};")
        .expect("Should parse named import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Absolute);
                assert_eq!(
                    path.segments,
                    vec!["math".to_string(), "geometry".to_string()]
                );
                assert_eq!(
                    *clause,
                    Some(ImportClause::Named(vec![
                        "Point".to_string(),
                        "Line".to_string(),
                        "Circle".to_string()
                    ]))
                );
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_single() {
    let ast = parse_source("use graphics::renderer::Shader;").expect("Should parse single import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Absolute);
                assert_eq!(
                    path.segments,
                    vec!["graphics".to_string(), "renderer".to_string()]
                );
                assert_eq!(*clause, Some(ImportClause::Single("Shader".to_string())));
            }
            _ => panic!("Expected import statement"),
        }
    }
}

#[test]
fn parser_import_module_only() {
    let ast = parse_source("use math::geometry;").expect("Should parse module import");

    if let rustleaf::AstNode::Program { items, .. } = ast {
        match &items[0] {
            rustleaf::AstNode::ImportStatement { path, clause, .. } => {
                assert_eq!(path.root_type, ModulePathRoot::Absolute);
                assert_eq!(path.segments, vec!["math".to_string()]);
                assert_eq!(*clause, Some(ImportClause::Single("geometry".to_string())));
            }
            _ => panic!("Expected import statement"),
        }
    }
}
