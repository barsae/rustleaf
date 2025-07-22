use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::Path;
use syn::{parse_macro_input, LitStr};

#[derive(Debug, Clone)]
enum TestType {
    Normal,
    Panic,
    Ignore,
    Lex,
    Parse,
}

// Test discovery implementation
#[proc_macro_attribute]
pub fn rustleaf_tests(args: TokenStream, _input: TokenStream) -> TokenStream {
    let test_dir = parse_macro_input!(args as LitStr);
    let test_dir_path = test_dir.value();

    // Read directory and find .rustleaf files
    let test_files = match discover_rustleaf_files(&test_dir_path) {
        Ok(files) => files,
        Err(e) => panic!("Failed to read test directory '{test_dir_path}': {e}"),
    };

    // Generate individual test functions that use include_str!
    let test_functions =
        test_files
            .iter()
            .map(|(test_name, include_path, test_type, full_path)| {
                let test_fn_name = syn::Ident::new(test_name, proc_macro2::Span::call_site());

                let test_body = match test_type {
                    TestType::Lex => {
                        let output_path = full_path.replace(".rustleaf", ".lex");
                        quote! {
                            let source = include_str!(#include_path);
                            let tokens = rustleaf::lexer::Lexer::tokenize(source).unwrap();
                            let output = format!("{:#?}", tokens);
                            std::fs::write(#output_path, output).unwrap();
                        }
                    }
                    TestType::Parse => {
                        let output_path = full_path.replace(".rustleaf", ".parse");
                        quote! {
                            let source = include_str!(#include_path);
                            let ast = rustleaf::parser::Parser::parse_str(source).unwrap();
                            let output = format!("{:#?}", ast);
                            std::fs::write(#output_path, output).unwrap();
                        }
                    }
                    TestType::Normal => {
                        let output_path = full_path.replace(".rustleaf", ".parse");
                        quote! {
                            let source = include_str!(#include_path);
                            let ast = rustleaf::parser::Parser::parse_str(source).unwrap();
                            let output = format!("{:#?}", ast);
                            std::fs::write(#output_path, output).unwrap();
                            let _result = rustleaf::eval::evaluate(ast).unwrap();
                        }
                    }
                    TestType::Panic => quote! {
                        let source = include_str!(#include_path);
                        let ast = rustleaf::parser::Parser::parse_str(source).unwrap();
                        let _result = rustleaf::eval::evaluate(ast).unwrap();
                    },
                    TestType::Ignore => quote! {
                        let source = include_str!(#include_path);
                        let ast = rustleaf::parser::Parser::parse_str(source).unwrap();
                        let _result = rustleaf::eval::evaluate(ast).unwrap();
                    },
                };

                match test_type {
                    TestType::Ignore => quote! {
                        #[test]
                        #[ignore]
                        fn #test_fn_name() {
                            #test_body
                        }
                    },
                    TestType::Panic => quote! {
                        #[test]
                        #[should_panic]
                        fn #test_fn_name() {
                            #test_body
                        }
                    },
                    _ => quote! {
                        #[test]
                        fn #test_fn_name() {
                            #test_body
                        }
                    },
                }
            });

    let expanded = quote! {
        #(#test_functions)*
    };

    TokenStream::from(expanded)
}

fn discover_rustleaf_files(
    test_dir: &str,
) -> Result<Vec<(String, String, TestType, String)>, std::io::Error> {
    let mut test_files = Vec::new();
    let test_path = Path::new(test_dir);

    if !test_path.exists() {
        // Create the directory if it doesn't exist
        fs::create_dir_all(test_path)?;
    }

    for entry in fs::read_dir(test_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "rustleaf" {
                    let file_path = path.to_string_lossy().to_string();

                    // Generate test function name: strip "./tests/" and convert to function name
                    let test_name = generate_test_name(&file_path);

                    // Determine test type based on filename suffix
                    let filename = path.file_name().unwrap().to_string_lossy();
                    let test_type = if filename.ends_with("_panic.rustleaf") {
                        TestType::Panic
                    } else if filename.ends_with("_ignore.rustleaf") {
                        TestType::Ignore
                    } else if filename.ends_with("_lex.rustleaf") {
                        TestType::Lex
                    } else if filename.ends_with("_parse.rustleaf") {
                        TestType::Parse
                    } else {
                        TestType::Normal
                    };

                    // For include_str!, construct path relative to where the macro is called
                    // Extract just the subdirectory name from test_dir and filename
                    let test_dir_name = Path::new(test_dir)
                        .file_name()
                        .unwrap_or_else(|| std::ffi::OsStr::new(""))
                        .to_string_lossy();
                    let include_path = format!("{test_dir_name}/{filename}");

                    test_files.push((test_name, include_path, test_type, file_path));
                }
            }
        }
    }

    test_files.sort_by(|a, b| a.0.cmp(&b.0)); // Sort by test name for consistent ordering
    Ok(test_files)
}

fn generate_test_name(file_path: &str) -> String {
    // Strip "./tests/" prefix if present
    let relative_path = if let Some(stripped) = file_path.strip_prefix("./tests/") {
        stripped // Remove "./tests/"
    } else if let Some(stripped) = file_path.strip_prefix("tests/") {
        stripped // Remove "tests/"
    } else {
        file_path
    };

    // Remove .rustleaf extension and convert path separators to underscores
    let without_extension = if let Some(stem) = Path::new(relative_path).file_stem() {
        let parent = Path::new(relative_path).parent().unwrap_or(Path::new(""));
        if parent.as_os_str().is_empty() {
            stem.to_string_lossy().to_string()
        } else {
            format!("{}/{}", parent.to_string_lossy(), stem.to_string_lossy())
        }
    } else {
        relative_path.to_string()
    };

    // Convert path separators to underscores
    without_extension.replace(['/', '\\'], "_")
}
