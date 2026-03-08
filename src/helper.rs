use crate::{AsciiCleaner, AsciiCleanerResult};

pub(crate) fn now_in_unix_epoch() -> AsciiCleanerResult<u64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
impl AsciiCleaner {
    pub fn is_allowed_ascii(c: char) -> bool {
        c.is_ascii_alphanumeric()
            || c.is_ascii_graphic()
            || c.is_ascii_whitespace()
            || c == '\n'
            || c == '\t'
            || c == '\r'
    }
}
