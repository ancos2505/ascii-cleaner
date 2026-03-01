use crate::AsciiCleanerResult;

pub(crate) fn now_in_unix_epoch() -> AsciiCleanerResult<u64> {
    use std::time::{SystemTime, UNIX_EPOCH};
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
