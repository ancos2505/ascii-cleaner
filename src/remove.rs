use std::{io::Read, num::NonZeroUsize, ops::Not};

use crate::{
    AsciiCleaner, AsciiCleanerResult,
    helper::now_in_unix_epoch,
    report::{AsciiCleanerReport, AsciiCleanerReportItem},
};

impl AsciiCleaner {
    pub fn remove(mut self) -> AsciiCleanerResult<AsciiCleanerReport> {
        if self.with_backup {
            let extension = self
                .file_path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_owned())
                .unwrap_or("bak".to_owned());
            dbg!(&extension);

            let mut new_file_path = self.file_path.clone();

            new_file_path.add_extension(format!("{extension}.{}", now_in_unix_epoch()?));

            // let bak_file = File::create_new(self.file_path)
            //     .map_or_else(|e| Ok(File::create_new(new_path)?), |f| Ok(f))?;

            std::fs::copy(self.file_path, new_file_path)?;
        }

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
