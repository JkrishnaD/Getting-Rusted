use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kvdb")]
#[command(about = "A simple persistent KV database")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// commands which are used in the application
#[derive(Subcommand)]
pub enum Commands {
    Set { key: String, value: String },
    Get { key: String },
    Delete { key: String },
}

// function to parse the command line arguments
// and return a Cli struct
pub fn pasre_args() -> Cli {
    Cli::parse()
}
