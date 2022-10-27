use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PackagesFile {
    #[serde(rename = "packages")]
    pub inner: BTreeMap<String, Package>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    #[serde(rename = "package")]
    pub package_info: PackageInfo,
    // #[serde(rename = "file")]
    // pub file_info: PackageFileInfo<'a>,
}

// #[derive(Debug, Serialize, Deserialize)]
pub enum _InstallationStatus {
    Ok,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub status: String,
    pub maintainer: String,
    pub version: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PackageFileInfo<'a> {
//     pub filename: String, // example-x86_64-ubuntu-1.0.0.winter
//     pub name: String, // example
//     pub architecture: String, // x86_64
//     pub target: String, // ubuntu
//     pub version: String, // 1.0.0
// } // TODO: impl parser
