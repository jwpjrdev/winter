mod cli;

use cli::*;

use gumdrop::Options;
use sudo::{check, RunningAs};

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
                    // Command::List(opts) => {},
                    _ => {},
                };
            },
            None => {},
        };
    }
}