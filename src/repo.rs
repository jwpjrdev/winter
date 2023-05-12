use crate::{error::{Error as WinterError, exit_with_error}, cli::OutputFormat};
use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    pub id: String,
    pub dependencies: Vec<String>,
    pub install: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum RepoType {
    Local,
    Remote,
}

// TODO: see how this is to write code around
pub trait Repo {
    fn id(&self) -> &String;
    fn packages(&self) -> &Vec<Package>;
    fn remote_url(&self) -> Option<&String>;
    fn path(&self) -> &PathBuf;
    // TODO: see if this is even needed
    fn repo_type(&self) -> RepoType;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RemoteRepo {
    id: String,
    packages: Vec<Package>,
    remote_url: String,
    path: PathBuf,
}

impl Repo for RemoteRepo {
    fn id(&self) -> &String {
        &self.id
    }

    fn packages(&self) -> &Vec<Package> {
        &self.packages
    }

    fn remote_url(&self) -> Option<&String> {
        Some(&self.remote_url)
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn repo_type(&self) -> RepoType {
        RepoType::Remote
    }
}

impl RemoteRepo {
    pub fn remove(path: PathBuf) -> Result<()> {
        std::fs::remove_file(path)?;
        Ok(())
    }

    pub fn remove_by_id(id: String) -> Result<()> {
        let repos_dir = get_remote_repos_path();
        let path = repos_dir.join(format!("{id}.json"));
        RemoteRepo::remove(path)?;
        Ok(())
    }

    pub fn add(repo_url: String, new_id: Option<String>, output_format: OutputFormat) -> Result<RemoteRepo> {
        // Fetch the RemoteRepo from the provided URL & set new id if provided
        let mut repo = RemoteRepo::fetch(repo_url)?;
        if let Some(new_id) = new_id {
            repo.id = new_id;
        }

        // Create remote repo directory if it doesn't exist
        let repos_dir = get_remote_repos_path();
        fs::create_dir_all(&repos_dir)?;

        // Set the remote repo's path & create it or error if it already exists
        repo.path = repos_dir.join(format!("{}.json", repo.id));
        if repo.path.as_path().exists() {
            exit_with_error(WinterError::RepoExists, output_format)?;
        } else {
            File::create(&repo.path)?;
        }

        // Serialise and write it to the file
        let json_string = toml::to_string(&repo)?;

        let file = OpenOptions::new().write(true).open(&repo.path)?;
        let mut file = BufWriter::new(file);
        file.write_all(json_string.as_bytes())?;

        // Ok((repo_path, remote_repo.id.to_string()))

        Ok(repo)
    }

    pub fn get(id: String) -> Result<RemoteRepo> {
        let path = get_remote_repos_path().join(format!("{id}.json"));
        let contents = std::fs::read_to_string(path)?;
        let repo: RemoteRepo = toml::from_str(&contents)?;

        Ok(repo)
    }

    pub fn get_by_path(path: PathBuf) -> Result<RemoteRepo> {
        let id = path.file_stem().unwrap().to_str().unwrap().to_string();
        RemoteRepo::get(id)
    }

    pub fn fetch(repo_url: String) -> Result<RemoteRepo> {
        // TODO: don't forget to remove temp URL and use repo_url
        const URL: &str =
            "https://raw.githubusercontent.com/jwpjrdev/winter/master/examples/example_repo.toml";

        let data = ureq::get(URL).call()?.into_string()?;
        let mut remote_repo: RemoteRepo = toml::from_str(&data)?;
        remote_repo.remote_url = repo_url;

        Ok(remote_repo)
    }

    pub fn list_all() -> Result<Vec<RemoteRepo>> {
        let dir = get_remote_repos_path();

        // TODO: error instead of returning an empty vector
        if !dir.exists() {
            return Ok(vec![]);
        }

        let repo_paths = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()?;

        let mut result = Vec::new();
        for path in repo_paths {
            result.push(RemoteRepo::get_by_path(path)?);
        }

        Ok(result)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalRepo {
    id: String,
    packages: Vec<Package>,
    path: PathBuf,
}

impl Repo for LocalRepo {
    fn id(&self) -> &String {
        &self.id
    }

    fn packages(&self) -> &Vec<Package> {
        &self.packages
    }

    fn remote_url(&self) -> Option<&String> {
        None
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }

    fn repo_type(&self) -> RepoType {
        RepoType::Local
    }
}

impl LocalRepo {
    pub fn new(id: String, packages: Vec<Package>, path: PathBuf) -> Self {
        LocalRepo { id: id, packages: packages, path: path }
    }

    pub fn create(id: String, output_format: OutputFormat) -> Result<Self> {
        // Create local repo directory if it doesn't exist
        let dir = get_local_repos_path();
        fs::create_dir_all(&dir)?;
        
        
        let repo_path = dir.join(format!("{}.json", id));
        if repo_path.as_path().exists() {
            exit_with_error(WinterError::RepoExists, output_format)?;
        } else {
            File::create(&repo_path)?;
        }

        let example_package = Package { id: "example_package".to_string(), dependencies: vec!["echo".to_string()], install: "echo 'example'".to_string() };
        let local_repo = LocalRepo::new(id, vec![example_package], repo_path);

        let json_string = toml::to_string_pretty(&local_repo)?;

        let file = OpenOptions::new().write(true).open(&local_repo.path)?;
        let mut file = BufWriter::new(file);
        file.write_all(json_string.as_bytes())?;

        Ok(local_repo)
    }
}

pub fn get_project_path() -> PathBuf {
    ProjectDirs::from("", "", "winter")
        .unwrap()
        .data_dir()
        .to_path_buf()
}

pub fn get_remote_repos_path() -> PathBuf {
    get_project_path().join("remote_repos/")
}

pub fn get_local_repos_path() -> PathBuf {
    get_project_path().join("local_repos/")
}
