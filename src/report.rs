use std::{
    fmt::{Debug, Display},
    num::NonZeroUsize,
    ops::Deref,
};

#[derive(Debug)]
pub struct AsciiCleanerReport {
    // file_name: PathBuf,
    pub(crate) success: bool,
    pub(crate) bytes_read: usize,
    pub(crate) findings: Vec<AsciiCleanerReportItem>,
}

impl Display for AsciiCleanerReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push('{');
        output.push_str(format!(r#""success":{},"#, self.success).as_str());
        output.push_str(format!(r#""bytes_read":{},"#, self.bytes_read).as_str());
        output.push_str(format!(r#""findings":"#).as_str());
        if self.findings.len() > 0 {
            output.push('[');
            let len = self.findings.len();
            for (idx, item) in self.findings.iter().enumerate() {
                output.push_str(format!(r#"{item}"#,).as_str());
                if idx < len - 1 {
                    output.push(',');
                }
            }

            output.push(']');
        } else {
            output.push_str("null");
        }

        output.push('}');
        write!(f, "{output}")
    }
}

#[derive(Debug)]
pub struct AsciiCleanerReportItem {
    pub(crate) offset: ItemOffset,
    pub(crate) line: NonZeroUsize,
    pub(crate) column: NonZeroUsize,
    pub(crate) byte: ItemByte,
}
impl Display for AsciiCleanerReportItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push('{');
        output.push_str(format!(r#""offset":{},"#, self.offset).as_str());
        output.push_str(format!(r#""line":{},"#, self.line).as_str());
        output.push_str(format!(r#""column":{},"#, self.column).as_str());
        output.push_str(format!(r#""byte":{}"#, self.byte).as_str());
        output.push('}');
        write!(f, "{output}")
    }
}

pub struct ItemOffset(usize);

impl Display for ItemOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{:x}""#, self.0)
    }
}

impl Debug for ItemOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Deref for ItemOffset {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for ItemOffset {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

pub struct ItemByte(u8);

impl Display for ItemByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#""{:x}""#, self.0)
    }
}

impl Debug for ItemByte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl Deref for ItemByte {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u8> for ItemByte {
    fn from(value: u8) -> Self {
        Self(value)
    }
}
