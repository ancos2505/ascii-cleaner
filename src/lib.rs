mod report;
mod result;

use std::io::Read;

pub use result::{AsciiCleanerError, AsciiCleanerResult};

use self::report::AsciiCleanerReport;

pub struct AsciiCleaner;

impl AsciiCleaner {
    pub fn analyze<R: Read>(reader: R) -> AsciiCleanerResult<AsciiCleanerReport> {
        todo!();
        Ok(Default::default())
    }

    // pub fn sanitize<R: Read>(reader: R) -> AsciiCleanerResult<()> {
    //     todo!();
    //     Ok(())
    // }
}
