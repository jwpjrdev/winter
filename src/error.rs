use std::fmt;

#[derive(Debug)]
pub enum Error {
    RepoExists,
    InvalidURL,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::RepoExists => write!(f, "A repository with the same id is already registered locally. Set a different id with the `--id=<value>` flag."),
            Error::InvalidURL => write!(f, "The provided repository must be a URL."),
        }
    }
}
