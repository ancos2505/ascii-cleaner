use std::io::Error as StdioError;

use ascii_cleaner::AsciiCleanerError;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    MissingVerb,
    UnknownVerb(String),
    MissingFilePath,
    InvalidFilePath,
    // MissingInput,
    Stdio(StdioError),
    AsciiCleaner(AsciiCleanerError),
}

impl From<StdioError> for CliError {
    fn from(value: StdioError) -> Self {
        Self::Stdio(value)
    }
}

impl From<AsciiCleanerError> for CliError {
    fn from(value: AsciiCleanerError) -> Self {
        Self::AsciiCleaner(value)
    }
}
