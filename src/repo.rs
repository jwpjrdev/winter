use crate::error::Error as WinterError;
use anyhow::Result;
use directories::ProjectDirs;
use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty_indent, Value, json};
use std::{
    fs,
    fs::{File, OpenOptions},
    io::{BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct LocalRepo {
    local_repo: Repo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repo {
    pub id: String,
    pub packages: Vec<Package>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Package {
    // TODO: populate this field from the parent HashMap or just remove it
    pub id: String,
    pub dependencies: Vec<String>,
    pub install: String,
}

pub fn get_project_path() -> PathBuf {
    ProjectDirs::from("", "", "winter").unwrap().data_dir().to_path_buf()
}

pub fn get_remote_repos_path() -> PathBuf {
    get_project_path().join("remote_repos/")
}

pub fn get_local_repos_path() -> PathBuf {
    get_project_path().join("local_repos/")
}

pub fn remove_remote_repo(repo_id: String) -> Result<()> {
    let repos_path = get_remote_repos_path();
    let repo_path = repos_path.join(format!("{}.json", repo_id));
    std::fs::remove_file(repo_path)?;
    Ok(())
}

/// Returns the path of the remote repository file and its id
pub fn add_remote_repo(repo_url: String, new_id: Option<String>) -> Result<(PathBuf, String)> {
    let json = fetch_remote_repo(repo_url, new_id)?;

    let repos_path = get_remote_repos_path();

    fs::create_dir_all(&repos_path)?;

    let id = json["repo"]["id"]
        .as_str()
        .expect("invalid repository provided");
    let repo_path = repos_path.join(format!("{}.json", id));

    if repo_path.as_path().exists() {
        anyhow::bail!(WinterError::RepoExists);
    } else {
        File::create(&repo_path)?;
    }

    let json_string = to_string_pretty_indent(&json, b"    ")?;

    let file = OpenOptions::new().write(true).open(&repo_path)?;
    let mut file = BufWriter::new(file);
    file.write_all(json_string.as_bytes())?;

    Ok((repo_path, id.to_string()))
}

pub fn list_remote_repos() -> Result<Vec<(String, PathBuf)>> {
    let dir = get_remote_repos_path();
    // TODO: keep or pass the error upstream?
    if !dir.exists() {
        return Ok(vec![]);
    }

    let repo_paths = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    let mut list = Vec::new();
    for path in repo_paths {
        list.push((path.file_stem().unwrap().to_str().unwrap().to_string(), path));
    }

    Ok(list)
}

pub fn create_local_repo(id: String) -> Result<PathBuf> {
    let repos_path = get_local_repos_path();

    fs::create_dir_all(&repos_path)?;

    let repo_path = repos_path.join(format!("{}.json", id));

    if repo_path.as_path().exists() {
        anyhow::bail!(WinterError::RepoExists);
    } else {
        File::create(&repo_path)?;
    }

    let json_string = to_string_pretty_indent(&json!({
        "local_repo": {
            "id": id,
            "packages": [
                {
                    "id": "example_package",
                    "dependencies": ["echo"],
                    "install": "echo 'example'",
                }
            ],
        },
    }), b"    ")?;

    let file = OpenOptions::new().write(true).open(&repo_path)?;
    let mut file = BufWriter::new(file);
    file.write_all(json_string.as_bytes())?;

    Ok(repo_path)
}

// TODO: handle invalid repos (i.e. invalid JSON)
fn fetch_remote_repo(repo_url: String, new_id: Option<String>) -> Result<Value> {
    const URL: &str =
        "https://raw.githubusercontent.com/jwpjrdev/winter/master/examples/example_repo.json";
    let data = ureq::get(URL).call()?.into_string()?;

    let mut value: Value = serde_json::from_str(data.as_str())?;
    match new_id {
        Some(new_id) => value["id"] = Value::String(new_id),
        None => {}
    };
    let result = serde_json::json!({
        "url": repo_url,
        "repo": value
    });

    return Ok(result);
}
