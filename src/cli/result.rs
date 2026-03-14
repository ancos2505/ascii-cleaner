use std::process::ExitCode;

use ascii_cleaner::AsciiCleanerError;

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    NoArgs,
    UnknownAction(String),
    MissingFilePath,
    InvalidFilePath,
    InvalidReplaceCharArg(String),
    BackupFileExtension,
    AsciiCleaner(AsciiCleanerError),
}

impl From<AsciiCleanerError> for CliError {
    fn from(value: AsciiCleanerError) -> Self {
        Self::AsciiCleaner(value)
    }
}

impl From<CliError> for ExitCode {
    fn from(value: CliError) -> Self {
        let outcome = match value {
            CliError::NoArgs | CliError::UnknownAction(_) => 1,
            CliError::MissingFilePath | CliError::InvalidReplaceCharArg(_) => 2,
            CliError::InvalidFilePath | CliError::BackupFileExtension => 3,
            CliError::AsciiCleaner(lib_err) => match lib_err {
                AsciiCleanerError::InvalidFilePath => 3,
                AsciiCleanerError::Infallibe(_)
                | AsciiCleanerError::TryFromIntError(_)
                | AsciiCleanerError::StdIo(_)
                | AsciiCleanerError::SystemTime(_) => 4,
            },
        };
        outcome.into()
    }
}
