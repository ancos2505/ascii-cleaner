use std::{fmt::Debug, num::NonZeroUsize, ops::Deref};

#[derive(Debug)]
pub struct AsciiCleanerReport {
    // file_name: PathBuf,
    pub(crate) success: bool,
    pub(crate) bytes_read: usize,
    pub(crate) findings: Vec<AsciiCleanerReportItem>,
}

#[derive(Debug)]
pub struct AsciiCleanerReportItem {
    pub(crate) idx: usize,
    pub(crate) line: NonZeroUsize,
    pub(crate) column: NonZeroUsize,
    pub(crate) byte: AsciiCleanerReportItemByte,
}

pub struct AsciiCleanerReportItemByte(u8);
impl Debug for AsciiCleanerReportItemByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Deref for AsciiCleanerReportItemByte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for AsciiCleanerReportItemByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
