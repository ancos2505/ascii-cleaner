use crate::{AsciiCleaner, AsciiCleanerResult, report::AsciiCleanerReport};

impl AsciiCleaner {
    pub fn remove(self) -> AsciiCleanerResult<AsciiCleanerReport> {
        Self::replace(self)
    }
}
