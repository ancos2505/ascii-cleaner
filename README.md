# ascii-cleaner


```sh

$ ascii-cleaner
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

$ ascii-cleaner detect ./myfile.txt
File: ./myfile.txt
Mode: detect
Non-ASCII characters found:
  - Total non-ASCII characters: 1
  - Lines affected: 1

$ ascii-cleaner sanitize ./myfile.txt
File: ./myfile.txt
Mode: sanitize
Non-ASCII characters found:
  - Total non-ASCII characters: 1
  - Lines affected: 1
File has been sanitized

$ ascii-cleaner detect ./myfile.txt
File: ./myfile.txt
Mode: detect
File is already ASCII-clean

```