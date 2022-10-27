use std::{
    fs::{
        File,
        create_dir_all,
        read_to_string,
    },
    io::Write,
    path::Path,
};

use crate::{
    backend::{
        data::PackagesFile,
        utils::ensure_running_as_root,
    },
    error::Error,
};

const DIR: &str = "/var/lib/winter/";
const FILE: &str = "packages";

pub fn load_packages() -> Result<String, Error> {
    create_if_not_exists().unwrap();

    let dir = Path::new(DIR);
    let packages_file = dir.join(FILE);

    let string = read_to_string(packages_file).unwrap();
    Ok(string)
}

pub fn write_packages(packages_data: String) -> Result<(), Error> {
    if let Err(error) = ensure_running_as_root() {
        return Err(error);
    }
    
    let dir = Path::new(DIR);
    let packages_file = dir.join(FILE);
    if !dir.exists() {
        create_dir_all(Path::new("/var/lib/winter/")).unwrap();
    }

    let mut file = File::create(packages_file).unwrap();
    writeln!(file, "{packages_data}").unwrap();

    Ok(())
}

fn create_if_not_exists() -> Result<(), Error> {
    let dir = Path::new(DIR);
    let packages_file = dir.join(FILE);
    if !packages_file.exists() {
        let default_data = serde_json::to_string(&PackagesFile::default())
            .expect("couldn't serialize default packages data");

        return write_packages(default_data);
    }

    Ok(())
}
