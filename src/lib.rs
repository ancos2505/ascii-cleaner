mod report;
mod result;

use std::{fs::File, io::Read, num::NonZeroUsize, ops::Not};

pub use result::{AsciiCleanerError, AsciiCleanerResult};

use crate::report::{AsciiCleanerReportItem, AsciiCleanerReportItemByte};

use self::report::AsciiCleanerReport;

pub struct AsciiCleaner;

impl AsciiCleaner {
    pub fn detect(mut file: File) -> AsciiCleanerResult<AsciiCleanerReport> {
        let mut buf: Vec<u8> = vec![];
        let bytes_read = file.read_to_end(&mut buf)?;
        let mut findings: Vec<AsciiCleanerReportItem> = vec![];
        let mut line = NonZeroUsize::new(1).unwrap();
        let mut column = NonZeroUsize::new(1).unwrap();
        let mut success = true;
        // TODO:
        // if (mode == "verbose"){println!("HEADER")
        for (idx, c) in buf.iter().enumerate() {
            if c.is_ascii().not() {
                let found = AsciiCleanerReportItem {
                    idx,
                    line,
                    column,
                    byte: (*c).into(),
                };
                // TODO:
                // if (mode == "verbose"){println!("{found}")
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
        // TODO:
        // if (mode == "verbose"){println!("FOOTER")
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
