mod builder;
mod detect;
mod helper;
mod remove;
mod replace;
mod report;
mod result;

use std::{
    fmt::{Debug, Display},
    fs::{File, OpenOptions},
    ops::Deref,
    path::PathBuf,
};

pub use result::{AsciiCleanerError, AsciiCleanerResult};

use crate::builder::Builder;

#[derive(Debug)]
pub struct AsciiCleaner {
    log_mode: LogMode,
    file_path: PathBuf,
    file: File,
}
impl AsciiCleaner {
    pub fn new(path: PathBuf) -> AsciiCleanerResult<Self> {
        if path.is_file() {
            Ok(Self {
                log_mode: LogMode::No,
                file: OpenOptions::new().read(true).write(true).open(&path)?,
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
#[derive(Debug, PartialEq, Eq)]
pub enum LogMode {
    PrintOnEachFinding,
    No,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WithBackup {
    BackupFile,
    NoBackupFile,
}

pub struct ReplaceChar(u8);
impl Default for ReplaceChar {
    fn default() -> Self {
        Self('?' as u8)
    }
}

impl Deref for ReplaceChar {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for ReplaceChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{}"#, self.0 as char)
    }
}

impl Debug for ReplaceChar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.0 as char)
    }
}

impl From<ReplaceChar> for u8 {
    fn from(value: ReplaceChar) -> Self {
        value.0
    }
}

impl From<u8> for ReplaceChar {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
