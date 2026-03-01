mod builder;
mod detect;
mod helper;
mod remove;
mod replace;
mod report;
mod result;

use std::{fs::File, path::PathBuf};

pub use result::{AsciiCleanerError, AsciiCleanerResult};

use crate::builder::Builder;

pub struct AsciiCleaner {
    log_mode: bool,
    with_backup: bool,
    file_path: PathBuf,
    file: File,
}
impl AsciiCleaner {
    pub fn new(path: PathBuf) -> AsciiCleanerResult<Self> {
        if path.is_file() {
            let file = File::open(&path)?;
            Ok(Self {
                with_backup: true,
                log_mode: false,
                file,
                file_path: path,
            })
        } else {
            Err(AsciiCleanerError::InvalidFilePath)
        }
    }
    pub fn builder() -> Builder {
        Builder
    }
}
