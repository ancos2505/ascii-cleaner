pub type AsciiCleanerResult<T> = Result<T, AsciiCleanerError>;

#[derive(Debug)]
pub enum AsciiCleanerError {
    MissingVerb,
}
impl AsciiCleanerError {
    pub fn msg<S: AsRef<str>>(s: S) -> Self {
        Self::msg(s.as_ref().to_string())
    }
}
