use clap::Parser;

use crate::cli::{Cli, Commands, LocalCommands, RepoCommands};

pub mod cli;
pub mod error;
pub mod repo;

fn main() -> anyhow::Result<()> {
    let string = r#"{
        "local_repo": {
            "id": "example4",
            "packages": [
                {
                    "id": "example_package",
                    "dependencies": [
                        "echo"
                    ],
                    "install": "echo 'example'"
                }
            ]
        }
    }"#;
    let repo: crate::repo::LocalRepo = serde_json::from_str(string)?;
    println!("{repo:#?}");
    // let cli = Cli::parse();
    // println!("{cli:#?}");

    // match cli.command {
    //     Commands::Repo { command } => {
    //         match command {
    //             RepoCommands::Add {
    //                 repo_url,
    //                 new_id,
    //                 show_path,
    //             } => {
    //                 if !is_url::is_url(repo_url.as_str()) {
    //                     anyhow::bail!(crate::error::Error::InvalidURL);
    //                 }

    //                 let (repo_path, final_id) = crate::repo::add_remote_repo(repo_url, new_id)?;

    //                 println!("The repository `{final_id}` was added");
    //                 if show_path {
    //                     println!("{}", repo_path.display());
    //                 }
    //             }
    //             RepoCommands::Remove { repo_id } => {
    //                 println!("The repository `{repo_id}` was removed");
    //                 crate::repo::remove_remote_repo(repo_id)?;
    //             }
    //             RepoCommands::List { show_paths } => {
    //                 let remote_repos = crate::repo::list_remote_repos()?;
    //                 if remote_repos.is_empty() {
    //                     println!("No installed remote repositories");
    //                 } else {
    //                     if remote_repos.len() > 1 {
    //                         if show_paths {
    //                             let mut ids = Vec::new();
    //                             for (id, _) in remote_repos {
    //                                 ids.push(id);
    //                             }
    //                             println!(
    //                                 "{} installed remote repositories: \n{}",
    //                                 ids.len(),
    //                                 ids.join(", ")
    //                             );
    //                         } else {
    //                             println!("{} installed remote repositories:", remote_repos.len());
    //                             for (id, path) in remote_repos {
    //                                 println!("{id}: {}", path.display());
    //                             }
    //                         }
    //                     } else {
    //                         let first = remote_repos.first().unwrap();
    //                         println!(
    //                             "1 installed remote repository: \n{} ({})",
    //                             first.0,
    //                             first.1.display()
    //                         );
    //                     }
    //                 }
    //             }
    //             RepoCommands::Path => {
    //                 println!("{}", crate::repo::get_remote_repos_path().display());
    //             }
    //             RepoCommands::Local { command } => {
    //                 match command {
    //                     LocalCommands::Create { repo_id } => {
    //                         let path = crate::repo::create_local_repo(repo_id.clone())?;
    //                         println!(
    //                             "The local repository `{repo_id}` was created:\n{}",
    //                             path.display()
    //                         );
    //                     }
    //                     LocalCommands::Info => {
    //                         println!(
    //                             "Local repository directory: \n{}",
    //                             crate::repo::get_local_repos_path().display()
    //                         );
    //                     }
    //                 };
    //             }
    //         };
    //     }
    //     Commands::Path => {
    //         println!("{}", crate::repo::get_project_path().display());
    //     }
    //     Commands::Show { package_id } => {
    //         println!("{package_id}");
    //     }
    // };

    Ok(())
}
