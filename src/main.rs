mod cli;

use std::{
    fs::File,
    process::{ExitCode, ExitStatus},
};

use ascii_cleaner::{AsciiCleaner, AsciiCleanerError, AsciiCleanerResult};

use crate::cli::{Cli, CliError, CliResult};

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            dbg!(err);
            ExitCode::FAILURE
        },
    }
}

fn smain() -> CliResult<()> {
    let mut args = std::env::args();

    let verb = match args.next() {
        Some(verb) => verb,
        None => {
            println!("{}", Cli::usage());
            return Err(CliError::MissingVerb);
        }
    };
    let maybe_input = args.next();

    let mut file = File::open(path)?;

    let report = match verb.as_ref() {
        "analyze" => AsciiCleaner::analyze(file)?,
        _ => return Err(CliError::UnknownVerb(verb)),
    };

    // let verb = maybe_verb.ok_or(CliError::MissingVerb)?;
    // let path = maybe_input.ok_or(CliError::MissingInput)?;

    Ok(())
}

// 1. Detect
// 2. Store report item
// 3. Optional: sanitize
// 4.   Sanitazing action: replace for what char ?
// 5. Return full Report
