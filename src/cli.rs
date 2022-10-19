use gumdrop::Options;

#[derive(Debug, Options)]
pub struct Cli {
    #[options(help = "prints help message")]
    pub help: bool,
    #[options(help = "enables verbose output")]
    pub verbose: bool,
    #[options(help = "prints version message")]
    pub version: bool,
    
    #[options(command)]
    pub command: Option<Command>,
}

#[derive(Debug, Options)]
pub enum Command {
    // winter --help [command] - shows help info for a command
    // winter install [package] - installs a package - REQUIRES ROOT
    #[options(help = "installs the provided package")]
    Install(PackageOpts),
    // winter uninstall [package] - uninstalls a package - REQUIRES ROOT
    #[options(help = "uninstalls the provided package")]
    Uninstall(PackageOpts),
    // winter update [optional package] [--check] - updates a package - REQUIRES ROOT - TODO: just check? just preview? both?
    #[options(help = "updates the provided package")]
    Update(PackageOpts),
    // winter info [package] - shows info for a package
    #[options(help = "shows info for the provided package")]
    Info(InfoOpts),
    // winter list [--available] - lists all installed packages or all available packages - REQUIRES ROOT IF DISPLAYING INSTALLED
    #[options(help = "lists all installed or remote packages")]
    List(ListOpts),
}

#[derive(Debug, Options)]
pub struct ListOpts {
    #[options(help = "lists all remote packages")]
    pub remote: bool,
}

#[derive(Debug, Options)]
pub struct PackageOpts {
    #[options(help = "prints command without executing")]
    pub preview: bool,
    #[options(free, help = "target package")]
    pub package: Option<String>,
}

#[derive(Debug, Options)]
pub struct InfoOpts {
    #[options(help = "target package")]
    pub package: Option<String>,
}
