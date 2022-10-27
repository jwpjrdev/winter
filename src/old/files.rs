use std::{
    fs::{File, self},
    io::Write,
    path::Path,
};

use crate::data::PackagesFile;
use crate::error::Error;

pub fn ensure_status_file_exists() -> Result<(), Error> {
    if let Err(error) = crate::backend::utils::ensure_running_as_root() {
        return Err(error);
    }

    let dir = Path::new("/var/lib/winter/");
    let status_file = dir.join("packages");
    if !status_file.exists() {
        fs::create_dir_all(Path::new("/var/lib/winter/")).unwrap();

        let mut file = File::create(status_file).unwrap();
        // TODO: switch to YAML or TOML
        let string = serde_json::to_string(&PackagesFile::default()).expect("couldn't serialize default status");
        writeln!(file, "{string}").unwrap();
    }

    Ok(())
}
// cheerful and allows

// pub fn fetch_packages_or_default() -> PackagesFile {

// }
