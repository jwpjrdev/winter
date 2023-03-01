use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Repo {
        #[command(subcommand)]
        command: Option<RepoCommands>,
    },
}

#[derive(Debug, Subcommand)]
pub enum RepoCommands {
    Add {
        repo: String
    }
}