use clap::Parser;
use crate::cli::Cli;

pub mod cli;

fn main() {
    let cli = Cli::parse();
    println!("{cli:#?}");
}