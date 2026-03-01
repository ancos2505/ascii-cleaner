mod cli;

use std::{fmt::Debug, path::PathBuf, process::ExitCode};

use ascii_cleaner::AsciiCleaner;

use crate::cli::{Cli, CliError, CliResult};

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            match &err {
                CliError::NoArgs => {
                    println!("{}", Cli::usage())
                }
                CliError::UnknownVerb(_) => print_error(&err),
                CliError::StdIo(_) => print_error(&err),
                CliError::AsciiCleaner(_) => print_error(&err),
                _ => print_error(&err),
            };
            err.into()
        }
    }
}
fn print_error<D: Debug>(error: &D) {
    eprintln!("{error:?}")
}

fn smain() -> CliResult<()> {
    let mut args = std::env::args();

    let args_len = args.len();

    let _ = args.next();

    let verb = match args.next() {
        Some(verb) => verb,
        None => {
            return Err(CliError::NoArgs);
        }
    };

    let verb = match verb.as_ref() {
        "detect" => Verb::Detect,
        "remove" => Verb::Remove(with_backup),
        "replace" => Verb::Replace(with_backup, replace_char),
        _ => return Err(CliError::UnknownVerb(verb)),
    };

    // TODO: check if is ascii before define here
    let replace_char = ReplaceChar::default();
    let with_backup = WithBackup::Yes;

    // let ascii_cleaner = AsciiCleaner::builder().file(path)?.finish();

    // TODO: Get log_mode from args
    // let ascii_cleaner = AsciiCleaner::builder().file(path)?.log_mode().finish();

    let ascii_cleaner = AsciiCleaner::new(path)?;

    let report = match verb.as_ref() {
        "detect" => ascii_cleaner.detect()?,
        "remove" => ascii_cleaner.remove()?,
        "replace" => ascii_cleaner.replace(replace_char)?,
        _ => return Err(CliError::UnknownVerb(verb)),
    };

    let path = args
        .next()
        .map(PathBuf::from)
        .ok_or(CliError::MissingFilePath)
        .into_iter()
        .filter(|path| path.is_file())
        .next()
        .ok_or(CliError::InvalidFilePath)?;
    println!("{report}");

    Ok(())
}

#[derive(Debug)]
pub(crate) enum Verb {
    Detect,
    Remove(WithBackup),
    Replace(WithBackup, ReplaceChar),
}

#[derive(Debug)]
pub(crate) enum WithBackup {
    Yes,
    No,
}

#[derive(Debug)]
pub(crate) struct ReplaceChar(u8);
impl Default for ReplaceChar {
    fn default() -> Self {
        Self('?' as u8)
    }
}

impl From<ReplaceChar> for char {
    fn from(value: ReplaceChar) -> Self {
        value.0
    }
}

// 1. Detect
// 2. Store report item
// 3. Optional: sanitize
// 4.   Sanitazing action: replace for what char ?
// 5. Return full Report
