use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PackagesFile {
    #[serde(rename = "packages")]
    pub inner: BTreeMap<String, Package>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub status: String,
    pub maintainer: String,
    pub version: String,
}

// #[derive(Debug, Serialize, Deserialize)]
pub enum _InstallationStatus {
    Ok,
}
