use clap::{Parser, Subcommand};

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
            println!("Would run file: {}", file);
        }
        Commands::Repl => {
            println!("RustLeaf REPL v{}", env!("CARGO_PKG_VERSION"));
            println!("REPL not yet implemented");
        }
        Commands::Parse { file } => {
            println!("Would parse file: {}", file);
        }
    }

    Ok(())
}