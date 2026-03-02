use crate::{AsciiCleaner, AsciiCleanerResult};

pub(crate) fn now_in_unix_epoch() -> AsciiCleanerResult<u64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
impl AsciiCleaner {
    pub fn is_allowed_ascii(c: char) -> bool {
        c.is_ascii_alphanumeric()
            || c.is_ascii_graphic()
            || c.is_ascii_whitespace()
            || c == '\n'
            || c == '\t'
            || c == '\r'
    }
}

pub(crate) fn backup_file(ctx: &AsciiCleaner) -> AsciiCleanerResult<u64> {
    let mut new_file_path = ctx.file_path.clone();

    new_file_path.add_extension(format!("bak.{}", now_in_unix_epoch()?));

    // let bak_file = File::create_new(ctx.file_path)
    //     .map_or_else(|e| Ok(File::create_new(new_path)?), |f| Ok(f))?;

    let bytes_transferred = std::fs::copy(&ctx.file_path, new_file_path)?;
    Ok(bytes_transferred)
}
