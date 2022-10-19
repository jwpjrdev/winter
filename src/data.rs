use sudo::{check, RunningAs};
use std::{
    fs::{File, self},
    io::Write,
    path::Path,
};

use crate::error::Error;

pub fn ensure_status_file_exists() -> Result<(), Error> {
    if check() != RunningAs::Root {
        // not running as root
        return Err(Error::NotEnoughPrivileges);
    }

    let dir = Path::new("/var/lib/winter/");
    let status_file = dir.join("status");
    if !status_file.exists() {
        fs::create_dir_all(Path::new("/var/lib/winter/")).unwrap();

        let mut file = File::create(status_file).unwrap();
        // TODO: switch to YAML or TOML
        writeln!(file, "{{\"packages\": []}}").unwrap();
    }

    Ok(())
}