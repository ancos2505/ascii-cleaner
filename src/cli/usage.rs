use crate::cli::Cli;

impl Cli {
    pub(crate) fn usage() -> String {
        let output = r#"
ASCII Cleaner v0.1.2

USAGE:
    ascii-cleaner <ACTION> <FILE> [OPTIONS]

ACTIONS:
    detect      Detect non-ASCII characters in file
    remove      Remove non-ASCII characters
    replace     Replace non-ASCII characters

OPTIONS (for sanitize action):
    --no-backup         Don't create backup file
    --char=CHAR      Replace non-ASCII characters with CHAR (default: '?')

EXAMPLES:
    ascii-cleaner detect myfile.txt
    ascii-cleaner detect myfile.txt --log-mode
    ascii-cleaner remove myfile.txt 
    ascii-cleaner remove myfile.txt --log-mode
    ascii-cleaner remove myfile.txt --no-backup
    ascii-cleaner remove myfile.txt --no-backup --log-mode
    ascii-cleaner replace myfile.txt --log-mode
    ascii-cleaner replace myfile.txt --char='%'
    ascii-cleaner replace myfile.txt --char='*' --log-mode    
"#;
        output.to_owned()
    }
}
