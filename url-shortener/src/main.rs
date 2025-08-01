use clap::{Parser, Subcommand, command};
use commands::{handle_delete_all, handle_delete_code, handle_list, handle_search, handle_shorten};
pub mod commands;
mod state;
mod store;

#[derive(Parser)]
#[command(name = "url-shortener")]
#[command(about = "A CLI tool to shorten URLs", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Shorten { url: String },
    Search { code: String },
    List,
    Delete { code: String },
    DeleteAll,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Shorten { url } => handle_shorten(url),
        Commands::Search { code } => handle_search(code),
        Commands::List => handle_list(),
        Commands::Delete { code } => handle_delete_code(code),
        Commands::DeleteAll => handle_delete_all(),
    }
}
