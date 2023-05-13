use anyhow::Result;
use is_terminal::IsTerminal;
use log::*;
use std::io::{BufRead, Write};

use crate::prelude::*;

pub struct WinterApp;

impl WinterApp {
    pub fn start(args: Cli) -> Result<()> {
        let output_format = match args.output_format {
            Some(format) => format,
            None => OutputFormat::Human,
        };
    
        WinterLogger::init(output_format, LevelFilter::max()).unwrap();

        // Trace > debug > info > warn > error
        trace!("trace");
        debug!("debug");
        info!("info");
        warn!("warn");
        error!("error");
    
        exit_with_error(Error::InvalidURL, output_format)?;
    
        match args.command {
            Commands::Test => {
                // cargo run -- test | cat
                if std::io::stdout().is_terminal() {
                    println!("it's a terminal");
                } else {
                    // piped
                    if let Some(format) = args.output_format {
                        match format {
                            OutputFormat::Json => {
                                std::io::stdout().write(b"{\"error\": \"404 page not found\"}\n")?;
                            }
                            OutputFormat::JsonLines => {
                                std::io::stdout().write(b"{\"error\": \"404 page not found\"}\n")?;
                            }
                            _ => {
                                std::io::stdout().write(b"this is a test")?;
                            }
                        };
                    } else {
                        println!("sucka");
                    }
                }
            }
            Commands::Test2 => {
                // echo test | cargo run -- test2
                let stdin = std::io::stdin();
                for line in stdin.lock().lines() {
                    println!("{} testing", line?);
                }
            }
            Commands::Repo { command } => {
                match command {
                    RepoCommands::Add {
                        repo_url,
                        new_id,
                        show_path,
                    } => {
                        if !is_url::is_url(repo_url.as_str()) {
                            exit_with_error(Error::InvalidURL, output_format)?;
                        }
    
                        let repo = RemoteRepo::add(repo_url, new_id, output_format)?;
    
                        println!("The repository `{}` was added", repo.id());
                        if show_path {
                            println!("{}", repo.path().display());
                        }
                    }
                    RepoCommands::Remove { repo_id } => {
                        println!("The repository `{repo_id}` was removed");
                        RemoteRepo::remove_by_id(repo_id)?;
                    }
                    RepoCommands::List { show_paths } => {
                        let remote_repos = RemoteRepo::list_all()?;
                        if remote_repos.is_empty() {
                            println!("No installed remote repositories");
                        } else {
                            if remote_repos.len() > 1 {
                                if show_paths {
                                    let mut ids = Vec::new();
                                    for repo in remote_repos {
                                        ids.push(repo.id().clone());
                                    }
                                    println!(
                                        "{} installed remote repositories: \n{}",
                                        ids.len(),
                                        ids.join(", ")
                                    );
                                } else {
                                    println!("{} installed remote repositories:", remote_repos.len());
                                    for repo in remote_repos {
                                        println!("{}: {}", repo.id(), repo.path().display());
                                    }
                                }
                            } else {
                                let first = remote_repos.first().unwrap();
                                println!(
                                    "1 installed remote repository: \n{} ({})",
                                    first.id(),
                                    first.path().display()
                                );
                            }
                        }
                    }
                    RepoCommands::Path => {
                        println!("{}", get_remote_repos_path().display());
                    }
                    RepoCommands::Local { command } => {
                        match command {
                            LocalCommands::Create { repo_id } => {
                                let repo = LocalRepo::create(repo_id.clone(), output_format)?;
                                println!(
                                    "The local repository `{repo_id}` was created:\n{}",
                                    repo.path().display()
                                );
                            }
                            LocalCommands::Info => {
                                println!(
                                    "Local repository directory: \n{}",
                                    get_local_repos_path().display()
                                );
                            }
                        };
                    }
                };
            }
            Commands::Path => {
                println!("{}", get_project_path().display());
            }
            Commands::Show { package_id } => {
                println!("{package_id}");
            }
        };
    
        Ok(())
    }
}