use anyhow::Result;
use clap::Parser;
use winter::{WinterApp, cli::Cli};

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    WinterApp::start(cli)
}
