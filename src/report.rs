#[derive(Debug,Default)]
pub struct AsciiCleanerReport {
    pub(crate) idx: usize,
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) byte: u8
}