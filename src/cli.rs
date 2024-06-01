use clap::{Parser, Subcommand, ValueEnum};
use crate::logger::OutputFormat;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long = "output-format", value_enum, global = true)]
    pub output_format: Option<OutputFormat>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage repositories
    Repo {
        #[command(subcommand)]
        command: RepoCommands,
    },
    /// Show winter directory
    Path,
    /// Show information for a package
    Show {
        /// Package id to lookup
        package_id: String,
    },
    /// Testing stuff
    Test,
    /// More testing
    Test2,
}

#[derive(Debug, Subcommand)]
pub enum RepoCommands {
    /// Add a remote repository
    Add {
        /// Remote repository URL
        repo_url: String,
        /// Optional new id for the repository
        #[arg(long = "id")]
        new_id: Option<String>,
        /// Toggle printing the repository's installation path
        #[arg(short = 'p', long = "path")]
        show_path: bool,
    },
    /// Remove a remove repository
    Remove {
        /// Repository id to remove
        repo_id: String,
    },
    /// List all remote and local repositories
    List {
        #[arg(short = 'p', long = "paths", action = clap::ArgAction::SetFalse)]
        show_paths: bool,
    },
    /// Show repository directory
    Path,
    /// Local repository commands
    Local {
        #[command(subcommand)]
        command: LocalCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum LocalCommands {
    /// Create a new local repository
    Create { repo_id: String },
    /// View local repository information
    Info,
}
