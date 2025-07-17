use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::Path;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Comma, FatArrow},
    Ident, LitStr, Token,
};

// Binary ops implementation
struct BinaryOpLevel {
    method_name: Ident,
    next_method: Ident,
    operators: Punctuated<OpMapping, Comma>,
}

struct OpMapping {
    token_type: Ident,
    binary_op: Ident,
}

impl Parse for OpMapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let token_type = input.parse()?;
        if input.peek(FatArrow) {
            input.parse::<FatArrow>()?;
            let binary_op = input.parse()?;
            Ok(OpMapping {
                token_type,
                binary_op,
            })
        } else {
            let binary_op = token_type.clone();
            Ok(OpMapping {
                token_type,
                binary_op,
            })
        }
    }
}

impl Parse for BinaryOpLevel {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let method_name = input.parse()?;
        input.parse::<Token![->]>()?;
        let next_method = input.parse()?;
        input.parse::<Token![:]>()?;
        let content;
        syn::bracketed!(content in input);
        let operators = content.parse_terminated(OpMapping::parse, Comma)?;

        Ok(BinaryOpLevel {
            method_name,
            next_method,
            operators,
        })
    }
}

struct BinaryOpsInput {
    levels: Punctuated<BinaryOpLevel, Comma>,
}

impl Parse for BinaryOpsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let levels = input.parse_terminated(BinaryOpLevel::parse, Comma)?;
        Ok(BinaryOpsInput { levels })
    }
}

#[proc_macro_attribute]
pub fn binary_ops(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_ast = parse_macro_input!(input as syn::ItemImpl);
    let ops_input = parse_macro_input!(args as BinaryOpsInput);

    let mut methods = Vec::new();

    for level in ops_input.levels {
        let method_name = &level.method_name;
        let next_method = &level.next_method;

        let match_arms = level.operators.iter().map(|op| {
            let token_type = &op.token_type;
            let binary_op = &op.binary_op;
            quote! {
                TokenType::#token_type => {
                    self.advance();
                    BinaryOperator::#binary_op
                }
            }
        });

        let method = quote! {
            pub fn #method_name(&mut self) -> Option<AstNode> {
                let mut expr = self.#next_method()?;

                loop {
                    let op = match &self.peek().token_type {
                        #(#match_arms)*
                        _ => break,
                    };

                    let location = self.current_location();
                    let right = self.#next_method()?;
                    expr = AstNode::BinaryOp {
                        left: Box::new(expr),
                        operator: op,
                        right: Box::new(right),
                        location,
                    };
                }

                Some(expr)
            }
        };

        methods.push(method);
    }

    let self_ty = &input_ast.self_ty;
    let existing_items = &input_ast.items;

    let expanded = quote! {
        impl #self_ty {
            #(#existing_items)*
            #(#methods)*
        }
    };

    TokenStream::from(expanded)
}

// Test discovery implementation
#[proc_macro_attribute]
pub fn rustleaf_tests(args: TokenStream, _input: TokenStream) -> TokenStream {
    let test_dir = parse_macro_input!(args as LitStr);
    let test_dir_path = test_dir.value();

    // Read directory and find .rustleaf files
    let test_files = match discover_rustleaf_files(&test_dir_path) {
        Ok(files) => files,
        Err(e) => panic!("Failed to read test directory '{}': {}", test_dir_path, e),
    };

    // Generate individual test functions that use include_str!
    let test_functions = test_files.iter().map(|(test_name, file_path, should_panic)| {
        let test_fn_name = syn::Ident::new(test_name, proc_macro2::Span::call_site());
        
        if *should_panic {
            quote! {
                #[test]
                #[should_panic(expected = "Assertion failed")]
                fn #test_fn_name() {
                    let source = include_str!(#file_path);

                    let tokens = rustleaf::Lexer::new(source).unwrap();
                    let mut parser = rustleaf::Parser::new(tokens);
                    let ast = parser.parse().unwrap();

                    let mut evaluator = rustleaf::Evaluator::new();
                    evaluator.evaluate(&ast).unwrap();
                }
            }
        } else {
            quote! {
                #[test]
                fn #test_fn_name() {
                    let source = include_str!(#file_path);

                    let tokens = rustleaf::Lexer::new(source).unwrap();
                    let mut parser = rustleaf::Parser::new(tokens);
                    let ast = parser.parse().unwrap();

                    let mut evaluator = rustleaf::Evaluator::new();
                    evaluator.evaluate(&ast).unwrap();
                }
            }
        }
    });

    let expanded = quote! {
        #(#test_functions)*
    };

    TokenStream::from(expanded)
}

fn discover_rustleaf_files(test_dir: &str) -> Result<Vec<(String, String, bool)>, std::io::Error> {
    let mut test_files = Vec::new();
    let test_path = Path::new(test_dir);

    if !test_path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Test directory does not exist: {}", test_dir),
        ));
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

                    // Check if this is a panic test (ends with _panic.rustleaf)
                    let filename = path.file_name().unwrap().to_string_lossy();
                    let should_panic = filename.ends_with("_panic.rustleaf");

                    // For include_str!, construct path relative to where the macro is called
                    // The macro is called from tests/rustleaf.rs, so we need rustleaf/filename.rustleaf
                    let include_path = format!("rustleaf/{}", filename);

                    test_files.push((test_name, include_path, should_panic));
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
