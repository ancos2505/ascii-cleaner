use std::{
    convert::Infallible, io::Error as StdIoError, num::TryFromIntError, time::SystemTimeError,
};

pub type AsciiCleanerResult<T> = Result<T, AsciiCleanerError>;

#[derive(Debug)]
pub enum AsciiCleanerError {
    InvalidFilePath,
    StdIo(StdIoError),
    SystemTime(SystemTimeError),
    Infallibe(Infallible),
    TryFromIntError(TryFromIntError),
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

impl From<Infallible> for AsciiCleanerError {
    fn from(value: Infallible) -> Self {
        Self::Infallibe(value)
    }
}

impl From<TryFromIntError> for AsciiCleanerError {
    fn from(value: TryFromIntError) -> Self {
        Self::TryFromIntError(value)
    }
}
