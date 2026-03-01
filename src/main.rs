mod cli;

use std::{fmt::Debug, fs::File, path::PathBuf, process::ExitCode};

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

    let _ = args.next();

    let verb = match args.next() {
        Some(verb) => verb,
        None => {
            return Err(CliError::NoArgs);
        }
    };
    let path = args
        .next()
        .map(PathBuf::from)
        .ok_or(CliError::MissingFilePath)
        .into_iter()
        .filter(|path| path.is_file())
        .next()
        .ok_or(CliError::InvalidFilePath)?;

    // TODO: check if is ascii before define here
    let replace_char = '?' as u8;

    let file = File::open(path)?;
    // let ascii_cleaner = AsciiCleaner::builder().file(file).verbose().finish();
    // let ascii_cleaner = AsciiCleaner::builder().file(file).finish();
    let ascii_cleaner = AsciiCleaner::new(file);
    let report = match verb.as_ref() {
        "detect" => ascii_cleaner.detect()?,
        "remove" => ascii_cleaner.remove()?,
        "replace" => ascii_cleaner.replace(replace_char)?,
        _ => return Err(CliError::UnknownVerb(verb)),
    };
    println!("{report}");

    Ok(())
}

// 1. Detect
// 2. Store report item
// 3. Optional: sanitize
// 4.   Sanitazing action: replace for what char ?
// 5. Return full Report
