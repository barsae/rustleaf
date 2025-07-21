use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rustleaf")]
#[command(about = "The RustLeaf programming language reference interpreter")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
