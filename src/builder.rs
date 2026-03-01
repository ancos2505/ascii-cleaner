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
    pub fn verbose(self) -> BuilderToFinish {
        let Self { file, file_path } = self;
        BuilderToFinish {
            file,
            verbose: true,
            file_path,
        }
    }
    pub fn finish(self) -> AsciiCleaner {
        let Self { file, file_path } = self;
        AsciiCleaner {
            file,
            file_path,
            verbose: false,
            with_backup: true,
        }
    }
}

pub struct BuilderToFinish {
    file: File,
    file_path: PathBuf,
    verbose: bool,
}

impl BuilderToFinish {
    pub fn finish(self) -> AsciiCleaner {
        let Self {
            file,
            verbose,
            file_path,
        } = self;
        AsciiCleaner {
            file_path,
            file,
            verbose,
            with_backup: true,
        }
    }
}
