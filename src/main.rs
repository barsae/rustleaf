use clap::{Parser, Subcommand};
use rustleaf::{Lexer, Parser as RustLeafParser, Evaluator};
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
        Commands::Repl => {
            println!("RustLeaf REPL v{}", env!("CARGO_PKG_VERSION"));
            println!("REPL not yet implemented");
        }
        Commands::Parse { file } => {
            parse_file(&file)?;
        }
    }

    Ok(())
}

fn run_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    
    // Tokenize
    let mut lexer = Lexer::new(&source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    if !lex_errors.is_empty() {
        for error in &lex_errors {
            eprintln!("Lexer error: {}", error);
        }
        return Err("Lexing failed".into());
    }
    
    // Parse
    let mut parser = RustLeafParser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(parse_errors) => {
            for error in &parse_errors {
                eprintln!("Parse error: {}", error);
            }
            return Err("Parsing failed".into());
        }
    };
    
    // Evaluate
    let mut evaluator = Evaluator::new();
    let result = evaluator.evaluate(&ast)?;
    
    println!("Result: {}", result);
    Ok(())
}

fn parse_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(file_path)?;
    
    // Tokenize
    let mut lexer = Lexer::new(&source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    if !lex_errors.is_empty() {
        for error in &lex_errors {
            eprintln!("Lexer error: {}", error);
        }
        return Err("Lexing failed".into());
    }
    
    // Parse
    let mut parser = RustLeafParser::new(tokens);
    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(parse_errors) => {
            for error in &parse_errors {
                eprintln!("Parse error: {}", error);
            }
            return Err("Parsing failed".into());
        }
    };
    
    println!("AST: {:#?}", ast);
    Ok(())
}