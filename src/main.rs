mod cli;
mod data;
mod error;
mod fetch;

use cli::*;

use gumdrop::Options;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let opts = Cli::parse_args_default_or_exit();

    if opts.version {
        println!("winter {VERSION}");
    } else {
        // todo: use verbose flag
        // println!("{:#?}", opts);
        match opts.command {
            Some(command) => {
                match command {
                    // Command::Install(opts) => {},
                    // Command::Uninstall(opts) => {},
                    // Command::Update(opts) => {},
                    // Command::Info(opts) => {},
                    Command::List(opts) => {
                        if opts.remote {
                            // lists remote
                            let package_list = fetch::fetch_package_list().unwrap();
                            let pretty_list = package_list.join(", ");
                            println!("all remote packages: {}", pretty_list);
                        } else {
                            // lists installed
                            // requires sudo to read /var/lib/winter/status
                            println!("{:?}", data::ensure_status_file_exists());
                        }
                    },
                    _ => {},
                };
            },
            None => {},
        };
    }
}