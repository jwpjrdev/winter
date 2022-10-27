mod backend;
mod error;
mod frontend;

use gumdrop::Options;

use crate::backend::{
    data::{PackageInfo, Package},
    manager::PackageManager,
};
use crate::error::Error;
use crate::frontend::cli::*;


const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    let opts = Cli::parse_args_default_or_exit();

    if opts.version {
        println!("winter {VERSION}");
    } else {
        // todo: use verbose flag
        // println!("{:#?}", opts);
        match opts.command {
            Some(command) => {
                let mut manager = match PackageManager::from_file() {
                    Ok(manager) => manager,
                    Err(error) => return Err(error),
                };
                match command {
                    // Command::Install(opts) => {},
                    // Command::Uninstall(opts) => {},
                    // Command::Update(opts) => {},
                    // Command::Info(opts) => {},
                    Command::List(opts) => {
                        if opts.remote {
                            // lists remote
                            // let package_list = fetch::fetch_package_list().unwrap();
                            // let pretty_list = package_list.join(", ");
                            // println!("All remote packages: {}", pretty_list);
                        } else {
                            // lists installed
                            let packages = manager.list_packages();
                            if packages.is_empty() {
                                println!("There are no installed packages");
                            } else {
                                println!("All installed packages: {}", packages.join(", "));
                            }
                        
                            // manager.add_package(
                            //     "example",
                            //     Package {
                            //         package_info: PackageInfo {
                            //             name: "example".to_string(),
                            //             status: "Ok".to_string(),
                            //             maintainer: "Joshua Price <jwpjr567@gmail.com>".to_string(),
                            //             version: "1.0.1".to_string(),
                            //         }
                            //     }
                            // )?;
                            // manager.write_to_file()?;
                        }
                    },
                    _ => {},
                };
            },
            None => {},
        };
    }

    Ok(())
}