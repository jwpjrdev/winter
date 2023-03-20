use clap::Parser;

use crate::cli::{Cli, Commands, RepoCommands};

pub mod cli;
pub mod error;
pub mod repo;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // println!("{cli:#?}");

    match cli.command {
        Commands::Repo { command } => {
            match command {
                RepoCommands::Add { repo, new_id } => {
                    if !is_url::is_url(repo.as_str()) {
                        anyhow::bail!(crate::error::Error::InvalidURL);
                    }
                    println!("add repo: {repo}");
                    println!("args id: {new_id:?}");
                    let path = crate::repo::add_remote_repo(repo, new_id)?;
                    println!("repo path: {}", path.display());
                }
                RepoCommands::Remove { repo_id } => {
                    println!("remove repo: {repo_id}");
                    crate::repo::remove_remote_repo(repo_id)?;
                }
                RepoCommands::List => {
                    println!("list repos");
                    let remote_repos = crate::repo::list_remote_repos()?;
                    // println!("remote repos: {}", remote_repos.join(", "));
                    println!("{remote_repos:?}");
                },
                RepoCommands::Path => {
                    println!("{}", crate::repo::get_repos_path().display());
                }
            };
        }
    };

    Ok(())
}
