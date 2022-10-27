use crate::error::Error;
use crate::backend::{
    data::{PackagesFile, Package},
    filesystem::{load_packages, write_packages},
};

#[derive(Debug)]
pub struct PackageManager {
    packages: PackagesFile,
}

impl PackageManager {
    pub fn from_file() -> Result<Self, Error> {
        let packages_data = load_packages();
        match packages_data {
            Ok(packages_data) => {
                let packages_file: PackagesFile = serde_json::from_str(&packages_data)
                    .expect("couldn't deserialize packages data");

                return Ok(PackageManager { packages: packages_file });
            },
            Err(error) => return Err(error),
        }
    }

    pub fn list_packages(&self) -> Vec<String> {
        Vec::from_iter(self.packages.inner.keys().cloned())
    }

    pub fn get_package<S: Into<String>>(&self, name: S) -> Option<&Package> {
        self.packages.inner.get(&name.into())
    }

    pub fn add_package<S: Into<String>>(&mut self, name: S, package_data: Package) -> Result<(), Error> {
        self.packages.inner.insert(name.into(), package_data);

        Ok(())
    }
    
    pub fn remove_package<S: Into<String>>(&mut self, name: S) -> Result<(), Error> {
        self.packages.inner.remove(&name.into());

        Ok(())
    }

    pub fn write_to_file(&self) -> Result<(), Error> {
        let packages_data = serde_json::to_string_pretty(&self.packages).expect("could not encode toml value");
        
        write_packages(packages_data)
    }
}
