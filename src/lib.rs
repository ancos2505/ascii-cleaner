mod builder;
mod detect;
mod helper;
mod remove;
mod replace;
mod report;
mod result;

use std::{
    fmt::{Debug, Display},
    fs::File,
    ops::Deref,
    path::PathBuf,
};

pub use self::result::{AsciiCleanerError, AsciiCleanerResult};

use crate::{builder::Builder, helper::now_in_unix_epoch};

#[derive(Debug)]
pub struct AsciiCleaner {
    run_mode: RunningMode,
    action: Action,
    file_path: PathBuf,
    file: File,
}
impl AsciiCleaner {
    pub fn new(action: Action, file_path: PathBuf) -> AsciiCleanerResult<Self> {
        Ok(Self::builder().action(action)?.file(file_path)?.finish())
    }
    pub fn builder() -> Builder {
        Builder
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Detect,
    Remove(WithBackup),
    Replace(WithBackup, ReplaceChar),
}
impl Action {
    pub fn detect() -> Self {
        Self::Detect
    }
    pub fn remove(with_backup: WithBackup) -> Self {
        Self::Remove(with_backup)
    }
    pub fn replace(with_backup: WithBackup, replace_char: ReplaceChar) -> Self {
        Self::Replace(with_backup, replace_char)
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push('{');
        match self {
            Action::Detect => {
                output.push_str(format!(r#""name":"detect","#).as_str());
                output.push_str(format!(r#""args":null"#).as_str());
            }
            Action::Remove(with_backup) => {
                output.push_str(format!(r#""name":"remove","#).as_str());
                output.push_str(format!(r#""args":"#).as_str());
                output.push('[');
                output.push_str(format!(r#"{with_backup}"#).as_str());
                output.push(']');
            }
            Action::Replace(with_backup, replace_char) => {
                output.push_str(format!(r#""name":"replace","#).as_str());
                output.push_str(format!(r#""args":[{with_backup},{{"name":"replace_char","value":"{replace_char}"}}]"#).as_str());
            }
        }
        output.push('}');
        write!(f, "{output}")
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RunningMode {
    PrintOnEachFinding,
    ReportAlways,
    Quiet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFileSize(usize);
impl Deref for NewFileSize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<usize> for NewFileSize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileSize(usize);
impl FileSize {
    pub(crate) fn new(size: usize) -> Self {
        Self(size)
    }
}
impl Deref for FileSize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for FileSize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

// TODO: Refactor to Compile-time state machine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupFile {
    Defined(PathBuf),
    Finished(PathBuf, FileSize),
}
impl BackupFile {
    pub fn new(file_path: &PathBuf) -> AsciiCleanerResult<Self> {
        Ok(Self::Defined(Self::generate_bkp_file_path(file_path)?))
    }

    fn generate_bkp_file_path(file_path: &PathBuf) -> AsciiCleanerResult<PathBuf> {
        let new_file_str = format!("{}.bak.{}", file_path.display(), now_in_unix_epoch()?);
        let mut new_file_path = PathBuf::new();
        new_file_path.push(new_file_str);

        Ok(new_file_path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WithBackup {
    BackupFile(BackupFile),
    NoBackupFile,
}

impl Display for WithBackup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push('{');
        output.push_str(format!(r#""backup_file":"#).as_str());

        match self {
            WithBackup::BackupFile(backup_file) => {
                match backup_file {
                    BackupFile::Defined(_) => unreachable!(),
                    BackupFile::Finished(path_buf, file_size) => {
                        output.push('{');
                        output
                            .push_str(format!(r#""file_path":"{}","#, path_buf.display()).as_str());
                        output.push_str(format!(r#""file_size":{}"#, **file_size).as_str());
                        output.push('}');
                    }
                };
            }
            WithBackup::NoBackupFile => {
                output.push_str(format!(r#"null"#).as_str());
            }
        };
        output.push('}');

        write!(f, "{output}")
    }
}

#[derive(Clone)]
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
