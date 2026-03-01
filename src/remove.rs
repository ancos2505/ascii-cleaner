use crate::{AsciiCleaner, AsciiCleanerResult, WithBackup, report::AsciiCleanerReport};

impl AsciiCleaner {
    pub fn remove(self, with_backup: WithBackup) -> AsciiCleanerResult<AsciiCleanerReport> {
        Self::replace(self, with_backup, None)
    }
}
