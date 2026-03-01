use std::{io::Read, num::NonZeroUsize, ops::Not};

use crate::{
    AsciiCleaner, AsciiCleanerResult,
    report::{AsciiCleanerReport, AsciiCleanerReportItem},
};

impl AsciiCleaner {
    pub fn replace(mut self, replace_char: u8) -> AsciiCleanerResult<AsciiCleanerReport> {
        let mut buf: Vec<u8> = vec![];
        let bytes_read = self.file.read_to_end(&mut buf)?;
        let mut findings: Vec<AsciiCleanerReportItem> = vec![];
        let mut line = NonZeroUsize::new(1).unwrap();
        let mut column = NonZeroUsize::new(1).unwrap();
        let mut success = true;

        for (idx, c) in buf.iter().enumerate() {
            if c.is_ascii().not() {
                let found = AsciiCleanerReportItem {
                    offset: idx.into(),
                    line,
                    column,
                    byte: (*c).into(),
                };
                // TODO:
                if self.log_mode == true {
                    println!("{found}")
                }
                findings.push(found);
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

        let report = AsciiCleanerReport {
            success,
            bytes_read,
            findings,
        };
        Ok(report)
    }

    // pub fn sanitize<R: Read>(reader: R) -> AsciiCleanerResult<()> {
    //     todo!();
    //     Ok(())
    // }
}
