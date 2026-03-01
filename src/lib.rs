mod builder;
mod detect;
mod remove;
mod replace;
mod report;
mod result;

use std::{fs::File, io::Read, num::NonZeroUsize, ops::Not};

pub use result::{AsciiCleanerError, AsciiCleanerResult};

use crate::{builder::Builder, report::AsciiCleanerReportItem};

use self::report::AsciiCleanerReport;

pub struct AsciiCleaner {
    verbose: bool,
    with_backup: bool,
    file: File,
}
impl AsciiCleaner {
    pub fn new(file: File) -> Self {
        Self {
            with_backup: true,
            verbose: false,
            file,
        }
    }
    pub fn builder() -> Builder {
        Builder
    }
}
