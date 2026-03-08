use std::{
    fs::File,
    io::{Read as _, Write as _},
    num::NonZeroUsize,
};

use crate::{
    AsciiCleaner, AsciiCleanerResult, LogMode,
    report::{AsciiCleanerReport, AsciiCleanerReportItem},
};

impl AsciiCleaner {
    pub fn replace(self) -> AsciiCleanerResult<AsciiCleanerReport> {
        let Self {
            log_mode,
            action,
            file_path,
            mut file,
        } = self;

        let mut buf_input: Vec<u8> = vec![];

        let bytes_read = file.read_to_end(&mut buf_input)?;
        drop(file);

        let mut new_file = File::create(&file_path)?;

        let mut findings: Vec<AsciiCleanerReportItem> = vec![];
        let mut line = NonZeroUsize::new(1).unwrap();
        let mut column = NonZeroUsize::new(1).unwrap();
        let mut success = true;
        let mut new_file_size = 0;

        for (idx, c) in buf_input.iter().enumerate() {
            if Self::is_allowed_ascii(*c as char) {
                match &action {
                    crate::Action::Detect => unreachable!(),
                    crate::Action::Remove(_) | crate::Action::Replace(_, _) => {
                        let bytes_wrote = new_file.write(&[*c])?;
                        new_file_size += bytes_wrote;
                    }
                };
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

                match &action {
                    crate::Action::Detect => unreachable!(),
                    crate::Action::Remove(_) => (),
                    crate::Action::Replace(_, replace_char) => {
                        let bytes_wrote = new_file.write(&[**replace_char])?;
                        new_file_size += bytes_wrote;
                    }
                };
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
            action,
            file_path,
            bytes_read,
            new_file_size: if new_file_size != bytes_read {
                Some(new_file_size)
            } else {
                None
            },
            findings,
        };
        Ok(report)
    }
}
