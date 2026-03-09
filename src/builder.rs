use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

use crate::{
    Action, AsciiCleaner, AsciiCleanerError, AsciiCleanerResult, BackupFile, FileSize, RunningMode,
    WithBackup,
};

pub struct Builder;

impl Builder {
    pub fn action(self, action: Action) -> AsciiCleanerResult<BuilderWithAction> {
        Ok(BuilderWithAction { action })
    }
}
pub struct BuilderWithAction {
    action: Action,
}

impl BuilderWithAction {
    pub fn file(self, file_path: PathBuf) -> AsciiCleanerResult<BuilderWithFile> {
        let Self { action } = self;

        let action = match action {
            Action::Detect => Action::Detect,
            Action::Remove(with_backup) => match with_backup {
                WithBackup::BackupFile(BackupFile::Defined(new_file_path)) => {
                    let bytes_transferred = std::fs::copy(&file_path, &new_file_path)?;
                    let new_file_size = FileSize::new(bytes_transferred.try_into()?);
                    Action::Remove(WithBackup::BackupFile(BackupFile::Finished(
                        new_file_path,
                        new_file_size,
                    )))
                }
                WithBackup::BackupFile(BackupFile::Finished(_, _)) => unreachable!(),
                WithBackup::NoBackupFile => Action::Remove(WithBackup::NoBackupFile),
            },
            Action::Replace(with_backup, replace_char) => match with_backup {
                WithBackup::BackupFile(BackupFile::Defined(new_file_path)) => {
                    let bytes_transferred = std::fs::copy(&file_path, &new_file_path)?;
                    let new_file_size = FileSize::new(bytes_transferred.try_into()?);
                    Action::Replace(
                        WithBackup::BackupFile(BackupFile::Finished(new_file_path, new_file_size)),
                        replace_char,
                    )
                }
                WithBackup::BackupFile(BackupFile::Finished(_, _)) => unreachable!(),
                WithBackup::NoBackupFile => Action::Replace(WithBackup::NoBackupFile, replace_char),
            },
        };
        if file_path.is_file() {
            let file = OpenOptions::new().read(true).write(true).open(&file_path)?;
            Ok(BuilderWithFile {
                file,
                file_path,
                action,
            })
        } else {
            Err(AsciiCleanerError::InvalidFilePath)
        }
    }
}

pub struct BuilderWithFile {
    action: Action,
    file_path: PathBuf,
    file: File,
}
impl BuilderWithFile {
    pub fn print_each_finding(self) -> BuilderToFinish {
        let Self {
            file,
            file_path,
            action,
        } = self;
        BuilderToFinish {
            file,
            run_mode: RunningMode::PrintOnEachFinding,
            file_path,
            action,
        }
    }
    pub fn quiet_mode(self) -> BuilderToFinish {
        let Self {
            file,
            file_path,
            action,
        } = self;
        BuilderToFinish {
            file,
            run_mode: RunningMode::Quiet,
            file_path,
            action,
        }
    }
    pub fn finish(self) -> AsciiCleaner {
        let Self {
            file,
            file_path,
            action,
        } = self;
        AsciiCleaner {
            file,
            file_path,
            run_mode: RunningMode::ReportAlways,
            action,
        }
    }
}

pub struct BuilderToFinish {
    action: Action,
    file: File,
    file_path: PathBuf,
    run_mode: RunningMode,
}

impl BuilderToFinish {
    pub fn finish(self) -> AsciiCleaner {
        let Self {
            file,
            run_mode,
            file_path,
            action,
        } = self;
        AsciiCleaner {
            file_path,
            file,
            run_mode,
            action,
        }
    }
}
