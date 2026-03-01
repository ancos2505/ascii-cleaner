use std::{io::Error as StdIoError, time::SystemTimeError};

pub type AsciiCleanerResult<T> = Result<T, AsciiCleanerError>;

#[derive(Debug)]
pub enum AsciiCleanerError {
    InvalidFilePath,
    StdIo(StdIoError),
    SystemTime(SystemTimeError),
}

impl From<StdIoError> for AsciiCleanerError {
    fn from(value: StdIoError) -> Self {
        Self::StdIo(value)
    }
}

impl From<SystemTimeError> for AsciiCleanerError {
    fn from(value: SystemTimeError) -> Self {
        Self::SystemTime(value)
    }
}
