#[derive(thiserror::Error, Debug)]
pub enum CliError {
    #[error("Unable to determine INI path")]
    MissingIniPath,
    #[error("Unable to determine INI section")]
    MissingIniSection,
    #[error("Unknown environment variable: {0}")]
    UnknownEnvironmentVariable(String),
    #[error("Parameter format invalid")]
    BadParam,
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Serde error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("{0}")]
    LibraryError(#[from] crate::errors::Error),
}

pub type CliResult = std::result::Result<(), CliError>;
