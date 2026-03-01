use std::{io::Error as StdIoError, process::ExitCode};

use ascii_cleaner::AsciiCleanerError;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    NoArgs,
    UnknownAction(String),
    MissingFilePath,
    InvalidFilePath,
    // MissingInput,
    StdIo(StdIoError),
    AsciiCleaner(AsciiCleanerError),
}

impl From<StdIoError> for CliError {
    fn from(value: StdIoError) -> Self {
        Self::StdIo(value)
    }
}

impl From<AsciiCleanerError> for CliError {
    fn from(value: AsciiCleanerError) -> Self {
        Self::AsciiCleaner(value)
    }
}

impl From<CliError> for ExitCode {
    fn from(value: CliError) -> Self {
        let outcome = match value {
            CliError::NoArgs => 1,
            CliError::UnknownAction(_) => 2,
            CliError::MissingFilePath => 2,
            CliError::InvalidFilePath => 2,
            CliError::StdIo(_) => 3,
            CliError::AsciiCleaner(_) => 4,
        };
        outcome.into()
    }
}
