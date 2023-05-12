use std::fmt;
use anyhow::Result;
use crate::cli::OutputFormat;

#[derive(Debug)]
pub enum Error {
    RepoExists,
    InvalidURL,
}

impl std::error::Error for Error {}

// TODO: write a logger that can be switched into a machine output mode in place of `lsimplelog`

impl Error {
    pub fn format(&self, format: OutputFormat) -> String {
        let err = serde_json::json!({
            "error": "", // TODO
            "description": self.message()
        }).to_string();
        println!("{}", err);
        match format {
            OutputFormat::Human => self.message(),
            OutputFormat::Json => format!("{{\"error\": \"{}\"}}\n", self.message()),
            OutputFormat::JsonLines => format!("{{\"error\": \"{}\"}}\n", self.message()),
        }
    }

    fn message(&self) -> String {
        match self {
            Error::RepoExists => "A repository with the same id is already registered locally. Set a different id with the `--id=<value>` flag.".to_string(),
            Error::InvalidURL => "The provided repository must be a URL.".to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

pub fn exit_with_error(error: Error, format: OutputFormat) -> anyhow::Result<()> {
    let formatted_error = error.format(format);
    if format == OutputFormat::Human {
        log::error!("{}", formatted_error);
    } else {
        use std::io::Write;
        std::io::stdout().write(formatted_error.as_bytes())?;
    }
    std::process::exit(0);
    // anyhow::bail!(error)
}