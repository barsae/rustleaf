use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "rustleaf")]
#[command(about = "The RustLeaf programming language reference interpreter")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run a script file
    #[arg(value_name = "FILE")]
    file: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an interactive REPL
    Repl,

    /// Run a script file
    Run {
        /// The script file to run
        file: String,
    },

    /// Evaluate an expression
    Eval {
        /// The expression to evaluate
        expression: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Repl) => run_repl(),
        Some(Commands::Run { file }) => run_file(&file),
        Some(Commands::Eval { expression }) => run_expression(&expression),
        None => {
            // If a file was provided as positional argument, run it
            if let Some(file) = cli.file {
                run_file(&file)
            } else {
                // Otherwise start REPL
                run_repl()
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;

    match rustleaf::run_and_print(source) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_expression(expr: &str) -> Result<(), Box<dyn std::error::Error>> {
    match rustleaf::run_and_print(expr.to_string()) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn run_repl() -> Result<(), Box<dyn std::error::Error>> {
    println!("RustLeaf REPL v0.1.0");
    println!("Type 'exit' to quit\n");

    let mut line = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        line.clear();
        io::stdin().read_line(&mut line)?;

        let input = line.trim();
        if input == "exit" {
            break;
        }

        if input.is_empty() {
            continue;
        }

        match rustleaf::run_and_print(input.to_string()) {
            Ok(()) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
