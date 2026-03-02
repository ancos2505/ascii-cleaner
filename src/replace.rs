use std::{
    fs::File,
    io::{Read as _, Write as _},
    num::NonZeroUsize,
};

use crate::{
    AsciiCleaner, AsciiCleanerResult, LogMode, ReplaceChar, WithBackup,
    helper::backup_file,
    report::{AsciiCleanerReport, AsciiCleanerReportItem},
};

impl AsciiCleaner {
    pub fn replace(
        self,
        with_backup: WithBackup,
        replace_char: Option<ReplaceChar>,
    ) -> AsciiCleanerResult<AsciiCleanerReport> {
        if with_backup == WithBackup::BackupFile {
            backup_file(&self)?;
        }
        let Self {
            log_mode,
            file_path,
            mut file,
        } = self;

        let mut buf_input: Vec<u8> = vec![];

        let bytes_read = file.read_to_end(&mut buf_input)?;
        drop(file);

        let mut new_file = File::create(file_path)?;

        let mut findings: Vec<AsciiCleanerReportItem> = vec![];
        let mut line = NonZeroUsize::new(1).unwrap();
        let mut column = NonZeroUsize::new(1).unwrap();
        let mut success = true;

        for (idx, c) in buf_input.iter().enumerate() {
            if Self::is_allowed_ascii(*c as char) {
                new_file.write(&[*c])?;
            } else {
                let found = AsciiCleanerReportItem {
                    offset: idx.into(),
                    line,
                    column,
                    byte: (*c).into(),
                };
                // TODO:
                if log_mode == LogMode::PrintOnEachFinding {
                    println!("{found}")
                }
                findings.push(found);
                if let Some(ref char_to_replace) = replace_char {
                    new_file.write(&[**char_to_replace])?;
                }
            }

            if *c == 10 {
                match line.checked_add(1) {
                    Some(v) => {
                        line = v;
                        column = NonZeroUsize::new(1).unwrap();
                    }
                    None => {
                        success = false;
                        // TODO: How to handle such error in the future
                        eprintln!("Graceful error: Line integer overflow");
                        break;
                    }
                }
            } else {
                match column.checked_add(1) {
                    Some(v) => column = v,
                    None => {
                        success = false;
                        // TODO: How to handle such error in the future
                        eprintln!("Graceful error: Column integer overflow");
                        break;
                    }
                }
            }
        }

        new_file.sync_all()?;
        let report = AsciiCleanerReport {
            success,
            bytes_read,
            findings,
        };
        Ok(report)
    }
}
