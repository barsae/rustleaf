use clap::{Parser, Subcommand};
use rustleaf::{Evaluator, Lexer, Parser as RustLeafParser, Repl};
use std::fs;

#[derive(Parser)]
#[command(name = "rustleaf")]
#[command(about = "The RustLeaf programming language interpreter")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a RustLeaf file
    Run {
        /// Path to the RustLeaf file to execute
        file: String,
    },
    /// Start interactive REPL mode
    Repl,
    /// Parse and display AST for a file (debug mode)
    Parse {
        /// Path to the RustLeaf file to parse
        file: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file } => {
            run_file(&file)?;
        }
        Commands::Repl => match Repl::new() {
            Ok(mut repl) => {
                if let Err(e) = repl.run() {
                    eprintln!("REPL error: {}", e);
                    return Err(e.into());
                }
            }
            Err(e) => {
                eprintln!("Failed to initialize REPL: {}", e);
                return Err(e.into());
            }
        },
        Commands::Parse { file } => {
            parse_file(&file)?;
        }
    }

    Ok(())
}

fn run_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;

    // Tokenize
    let tokens = match Lexer::new(&source) {
        Ok(tokens) => tokens,
        Err(lex_errors) => {
            eprintln!("Lexer errors: {lex_errors}");
            return Err("Lexing failed".into());
        }
    };

    // Parse
    let mut parser = RustLeafParser::new(tokens);
    let ast = parser.parse();

    // Evaluate
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&ast)?;

    println!("Result: {result}");
    Ok(())
}

fn parse_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;

    // Tokenize
    let tokens = match Lexer::new(&source) {
        Ok(tokens) => tokens,
        Err(lex_errors) => {
            eprintln!("Lexer errors: {lex_errors}");
            return Err("Lexing failed".into());
        }
    };

    // Parse
    let mut parser = RustLeafParser::new(tokens);
    let ast = parser.parse();

    println!("AST: {ast:#?}");
    Ok(())
}
