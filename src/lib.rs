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
    file_path: PathBuf,
    file: File,
}
impl AsciiCleaner {
    pub fn new(path: PathBuf) -> AsciiCleanerResult<Self> {
        if path.is_file() {
            let file = File::open(&path)?;
            Ok(Self {
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

#[derive(Debug, PartialEq, Eq)]
pub enum WithBackup {
    BackupFile,
    NoBackupFile,
}

// impl From<WithBackup> for bool {
//     fn from(value: WithBackup) -> Self {
//         match value {
//             WithBackup::BackupFile => true,
//             WithBackup::NoBackupFile => false,
//         }
//     }
// }

#[derive(Debug)]
pub struct ReplaceChar(u8);
impl Default for ReplaceChar {
    fn default() -> Self {
        Self('?' as u8)
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
