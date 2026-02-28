use crate::cli::Cli;

impl Cli {
    pub(crate) fn usage() -> String {
        let output = r#"
ASCII File Sanitizer

USAGE:
    ascii-cleaner <COMMAND> <FILE> [OPTIONS]

COMMANDS:
    detect      Detect non-ASCII characters in file
    sanitize    Remove or replace non-ASCII characters

OPTIONS (for sanitize command):
    --no-backup         Don't create backup file
    --remove            Remove non-ASCII characters instead of replacing
    --no-replace        Don't replace non-ASCII characters (remove them)
    --replace=CHAR      Replace non-ASCII characters with CHAR (default: '?')

EXAMPLES:
    ascii-cleaner detect myfile.txt
    ascii-cleaner sanitize myfile.txt
    ascii-cleaner sanitize myfile.txt --replace=*
    ascii-cleaner sanitize myfile.txt --remove --no-backup
"#;
        output.to_owned()
    }
}
