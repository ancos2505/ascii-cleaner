use crate::cli::Cli;

impl Cli {
    pub(crate) fn usage() -> String {
        format!(
            r#"
ASCII Cleaner v{}

USAGE:
    ascii-cleaner <ACTION> <FILE> [OPTIONS]

ACTIONS:
    detect      Detect non-ASCII characters in file
    remove      Remove non-ASCII characters
    replace     Replace non-ASCII characters

OPTIONS (for sanitize action):
    --no-backup         Don't create backup file
    --char=CHAR      Replace non-ASCII characters with CHAR (default: '?')

OPTIONS (for report):
    --log-mode  Print a json object on each finding and then a full report.
    --quiet     Print report only on findings

EXAMPLES:
    ascii-cleaner detect myfile.txt
    ascii-cleaner detect myfile.txt --log-mode
    ascii-cleaner detect myfile.txt --quiet
    ascii-cleaner remove myfile.txt 
    ascii-cleaner remove myfile.txt --log-mode
    ascii-cleaner remove myfile.txt --no-backup
    ascii-cleaner remove myfile.txt --no-backup --log-mode
    ascii-cleaner remove myfile.txt --no-backup --quiet
    ascii-cleaner replace myfile.txt --log-mode
    ascii-cleaner replace myfile.txt --quiet
    ascii-cleaner replace myfile.txt --char='%'
    ascii-cleaner replace myfile.txt --char='*' --log-mode

EXIT STATUS:
    0      if OK,
    1      if cli action problems (e.g., unkonwn action),
    2      if cli argument problems (e.g., no arg for file path),
    3      if io access problems (e.g., file not found),
    4      if serious trouble (e.g., can't read file).
"#,
            env!("CARGO_PKG_VERSION")
        )
    }
}
