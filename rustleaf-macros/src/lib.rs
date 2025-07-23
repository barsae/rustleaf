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

                let is_panic_test = matches!(test_type, TestType::Panic);
                let test_body = quote! {
                    // Read the markdown file and extract rustleaf code block
                    let md_content = include_str!(#include_path);
                    
                    let extract_rustleaf_code_block = |md_content: &str| -> Option<String> {
                        let lines: Vec<&str> = md_content.lines().collect();
                        let mut in_rustleaf_block = false;
                        let mut code_lines = Vec::new();
                        
                        for line in lines {
                            if line.trim() == "```rustleaf" {
                                in_rustleaf_block = true;
                                continue;
                            }
                            if line.trim() == "```" && in_rustleaf_block {
                                break;
                            }
                            if in_rustleaf_block {
                                code_lines.push(line);
                            }
                        }
                        
                        if code_lines.is_empty() {
                            None
                        } else {
                            Some(code_lines.join("\n"))
                        }
                    };
                    
                    let update_markdown_with_results = |_original_md: &str, source: &str, circle: &str, 
                        output_section: &str, execution_output: &str, lex_output: &str, 
                        parse_output: &str, eval_output: &str| -> String {
                        // Reconstruct the entire markdown from template
                        let output_display = if output_section == "None" {
                            format!("# Output\n{}", output_section)
                        } else {
                            format!("# Output\n```\n{}\n```", output_section)
                        };
                        
                        format!(
                            "# Program {}\n```rustleaf\n{}\n```\n\n{}\n\n# Result\n```rust\n{}\n```\n\n# Lex\n```rust\n{}\n```\n\n# Parse\n```rust\n{}\n```\n\n# Eval\n```rust\n{}\n```",
                            circle, source, output_display, execution_output, lex_output, parse_output, eval_output
                        )
                    };
                    
                    let source = extract_rustleaf_code_block(md_content)
                        .expect("Failed to find rustleaf code block in markdown file");
                    
                    // Check for #[fail_quietly] magic string
                    let fail_quietly = source.contains("#[fail_quietly]");
                    
                    // Try lexing
                    let tokens_result = rustleaf::lexer::Lexer::tokenize(&source);
                    let lex_output = format!("{:#?}", tokens_result);
                    
                    // Try parsing (only if lexing succeeded)
                    let parse_output = match &tokens_result {
                        Ok(_) => {
                            let parse_result = rustleaf::parser::Parser::parse_str(&source);
                            format!("{:#?}", parse_result)
                        }
                        Err(_) => "Skipped due to lex error".to_string(),
                    };
                    
                    // Try compiling to eval IR (only if parsing succeeded)
                    let eval_output = match rustleaf::parser::Parser::parse_str(&source) {
                        Ok(ast) => {
                            let eval_result = rustleaf::eval::Compiler::compile(ast);
                            format!("{:#?}", eval_result)
                        }
                        Err(_) => "Skipped due to parse error".to_string(),
                    };
                    
                    // Try evaluation (only if all previous stages succeeded)
                    let (output_section, execution_output, eval_success, final_result) = match rustleaf::parser::Parser::parse_str(&source) {
                        Ok(ast) => {
                            rustleaf::core::start_print_capture();
                            let result = rustleaf::eval::evaluate(ast);
                            let captured_output = rustleaf::core::get_captured_prints();
                            let execution_output = format!("{:#?}", result);
                            
                            let output_section = if captured_output.is_empty() {
                                "None".to_string()
                            } else {
                                captured_output.join("\n")
                            };
                            
                            let eval_success = result.is_ok();
                            (output_section, execution_output, eval_success, result)
                        }
                        Err(parse_error) => {
                            let error_result = Err(anyhow::Error::msg(format!("Parse error: {}", parse_error)));
                            ("None".to_string(), "Skipped due to parse error".to_string(), false, error_result)
                        }
                    };
                    
                    // For panic tests, success means it should fail (red circle = expected behavior)
                    // For normal tests, success means it should succeed (green circle = expected behavior)
                    let circle = if #is_panic_test {
                        if eval_success { "🔴" } else { "🟢" }
                    } else {
                        if eval_success { "🟢" } else { "🔴" }
                    };
                    
                    // Update the markdown file in-place with test results
                    let updated_md_content = update_markdown_with_results(
                        md_content, &source, &circle, &output_section, &execution_output, 
                        &lex_output, &parse_output, &eval_output
                    );
                    std::fs::write(#full_path, updated_md_content).unwrap();
                    
                    // Control test behavior based on fail_quietly flag
                    if !fail_quietly {
                        // Return actual result - let cargo handle expectations
                        final_result.unwrap();
                    }
                };

                // No need for separate panic test handling since we handle it in the main test body now
                let test_body = test_body;

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


    for entry in fs::read_dir(test_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "md" {
                    let file_path = path.to_string_lossy().to_string();

                    // Generate test function name: strip "./tests/" and convert to function name
                    let test_name = generate_test_name(&file_path);

                    // Determine test type based on filename suffix
                    let filename = path.file_name().unwrap().to_string_lossy();
                    let test_type = if filename.ends_with("_panic.md") {
                        TestType::Panic
                    } else if filename.ends_with("_ignore.md") {
                        TestType::Ignore
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

    // Remove .md extension and convert path separators to underscores
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
