use std::io::Error as StdIoError;

pub type AsciiCleanerResult<T> = Result<T, AsciiCleanerError>;

#[derive(Debug)]
pub enum AsciiCleanerError {
    MissingVerb,
    StdIo(StdIoError),
}

impl From<StdIoError> for AsciiCleanerError {
    fn from(value: StdIoError) -> Self {
        Self::StdIo(value)
    }
}
