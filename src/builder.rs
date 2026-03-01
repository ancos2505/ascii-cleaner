use std::{fs::File, path::PathBuf};

use crate::{AsciiCleaner, AsciiCleanerError, AsciiCleanerResult};

pub struct Builder;

impl Builder {
    pub fn file(self, path: PathBuf) -> AsciiCleanerResult<BuilderWithFile> {
        if path.is_file() {
            let file = File::open(&path)?;
            Ok(BuilderWithFile {
                file,
                file_path: path,
            })
        } else {
            Err(AsciiCleanerError::InvalidFilePath)
        }
    }
}

pub struct BuilderWithFile {
    file_path: PathBuf,
    file: File,
}
impl BuilderWithFile {
    pub fn log_mode(self) -> BuilderToFinish {
        let Self { file, file_path } = self;
        BuilderToFinish {
            file,
            log_mode: true,
            file_path,
        }
    }
    pub fn finish(self) -> AsciiCleaner {
        let Self { file, file_path } = self;
        AsciiCleaner {
            file,
            file_path,
            log_mode: false,
            with_backup: true,
        }
    }
}

pub struct BuilderToFinish {
    file: File,
    file_path: PathBuf,
    log_mode: bool,
}

impl BuilderToFinish {
    pub fn finish(self) -> AsciiCleaner {
        let Self {
            file,
            log_mode,
            file_path,
        } = self;
        AsciiCleaner {
            file_path,
            file,
            log_mode,
            with_backup: true,
        }
    }
}
