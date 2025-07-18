use crate::{Evaluator, Lexer, Parser as RustLeafParser, Value};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result as RustylineResult};
use std::env;

pub struct Repl {
    evaluator: Evaluator,
    editor: DefaultEditor,
}

impl Repl {
    pub fn new() -> RustylineResult<Self> {
        let mut editor = DefaultEditor::new()?;

        // Set up history file in user's home directory
        if let Ok(home) = env::var("HOME") {
            let history_path = format!("{}/.rustleaf_history", home);
            let _ = editor.load_history(&history_path);
        }

        // Create evaluator with current working directory
        let current_dir = env::current_dir().unwrap_or_else(|_| ".".into());
        let evaluator = Evaluator::new_with_base_dir(current_dir);

        Ok(Repl { evaluator, editor })
    }

    pub fn run(&mut self) -> RustylineResult<()> {
        let mut input_buffer = String::new();

        loop {
            let prompt = if input_buffer.is_empty() {
                "rustleaf> "
            } else {
                "     ...> "
            };

            match self.editor.readline(prompt) {
                Ok(line) => {
                    // Add line to input buffer
                    if !input_buffer.is_empty() {
                        input_buffer.push('\n');
                    }
                    input_buffer.push_str(&line);

                    // Try to parse the current input
                    match self.try_parse(&input_buffer) {
                        ParseResult::Complete(ast) => {
                            // Add complete input to history
                            self.editor.add_history_entry(&input_buffer)?;

                            // Evaluate the AST
                            match self.evaluator.evaluate(&ast) {
                                Ok(result) => {
                                    // Don't print unit values in REPL
                                    if !matches!(result, Value::Unit) {
                                        println!("{}", result);
                                    }
                                }
                                Err(error) => {
                                    eprintln!("Error: {}", error);
                                }
                            }

                            // Clear buffer for next input
                            input_buffer.clear();
                        }
                        ParseResult::Incomplete => {
                            // Continue reading more input
                            continue;
                        }
                        ParseResult::Error(errors) => {
                            // Add erroneous input to history anyway
                            self.editor.add_history_entry(&input_buffer)?;

                            // Display parse errors
                            for error in errors {
                                eprintln!("Parse error: {}", error);
                            }

                            // Clear buffer and start fresh
                            input_buffer.clear();
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // Ctrl+C - clear current input buffer
                    if !input_buffer.is_empty() {
                        input_buffer.clear();
                        println!();
                        continue;
                    } else {
                        break;
                    }
                }
                Err(ReadlineError::Eof) => {
                    // Ctrl+D
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }

        // Save history before exiting
        if let Ok(home) = env::var("HOME") {
            let history_path = format!("{}/.rustleaf_history", home);
            let _ = self.editor.save_history(&history_path);
        }

        Ok(())
    }

    fn try_parse(&self, input: &str) -> ParseResult {
        // First try parsing as-is
        let tokens = match Lexer::new(input) {
            Ok(tokens) => tokens,
            Err(lex_errors) => {
                return ParseResult::Error(vec![format!("Lexer error: {}", lex_errors)]);
            }
        };

        let mut parser = RustLeafParser::new(tokens);
        match parser.parse() {
            Ok(ast) => ParseResult::Complete(ast),
            Err(parse_errors) => {
                // Try adding a semicolon for REPL convenience first
                let input_with_semicolon = format!("{};", input.trim());
                if let Ok(tokens) = Lexer::new(&input_with_semicolon) {
                    let mut parser = RustLeafParser::new(tokens);
                    if let Ok(ast) = parser.parse() {
                        return ParseResult::Complete(ast);
                    }
                }

                // Check if the errors suggest incomplete input
                if self.looks_incomplete(&parse_errors) {
                    return ParseResult::Incomplete;
                }

                // Return original parse errors
                ParseResult::Error(parse_errors.into_iter().map(|e| e.to_string()).collect())
            }
        }
    }

    fn looks_incomplete(&self, errors: &[crate::parser::ast::ParseError]) -> bool {
        // Check if any error suggests the input is incomplete
        for error in errors {
            let error_msg = error.to_string().to_lowercase();

            // Common patterns that suggest incomplete input
            if error_msg.contains("unexpected end of file")
                || error_msg.contains("expected")
                || error_msg.contains("eof")
                || error_msg.contains("end of input")
            {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub enum ParseResult {
    Complete(crate::parser::ast::AstNode),
    Incomplete,
    Error(Vec<String>),
}

#[cfg(test)]
mod tests {
    use super::{ParseResult, Repl};

    #[test]
    fn test_parse_simple_expression() {
        let repl = Repl::new().unwrap();

        // Test simple arithmetic
        match repl.try_parse("2 + 3") {
            ParseResult::Complete(_) => {
                // Should succeed with automatic semicolon
            }
            other => panic!("Expected complete parse, got: {:?}", other),
        }
    }

    #[test]
    fn test_parse_incomplete_expression() {
        let repl = Repl::new().unwrap();

        // Test incomplete expression
        match repl.try_parse("if true {") {
            ParseResult::Incomplete => {
                // Should detect incomplete
            }
            other => panic!("Expected incomplete parse, got: {:?}", other),
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        let repl = Repl::new().unwrap();

        // Test variable declaration
        match repl.try_parse("var x = 42") {
            ParseResult::Complete(_) => {
                // Should succeed with automatic semicolon
            }
            other => panic!("Expected complete parse, got: {:?}", other),
        }
    }
}
