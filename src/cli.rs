use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
}

/// Manage repositories
#[derive(Debug, Subcommand)]
pub enum RepoCommands {
    /// Add a remote repository
    Add {
        repo: String, // url
        #[arg(long = "id")]
        new_id: Option<String>,
    },
    /// Remove a remove repository
    Remove {
        repo_id: String, // id
    },
    /// List all remote and local repositories
    List,
    /// Show repository directory
    Path,
}
