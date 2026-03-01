use std::fs::File;

use crate::AsciiCleaner;

pub struct Builder;

impl Builder {
    pub fn file(self, file: File) -> BuilderWithFile {
        BuilderWithFile { file }
    }
}

pub struct BuilderWithFile {
    file: File,
}
impl BuilderWithFile {
    pub fn verbose(self) -> BuilderToFinish {
        let Self { file } = self;
        BuilderToFinish {
            file,
            verbose: true,
        }
    }
    pub fn finish(self) -> AsciiCleaner {
        let Self { file } = self;
        AsciiCleaner {
            file,
            verbose: false,
            with_backup: true,
        }
    }
}

pub struct BuilderToFinish {
    file: File,
    verbose: bool,
}

impl BuilderToFinish {
    pub fn finish(self) -> AsciiCleaner {
        let Self { file, verbose } = self;
        AsciiCleaner {
            verbose,
            file,
            with_backup: true,
        }
    }
}
