use clap::Parser;

use crate::cli::Cli;


pub mod cli;
pub mod types;
pub mod config;


fn main(){
    let cli = Cli::parse();
    cli.e
}
