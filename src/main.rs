mod cli;

use std::{fs::File, process::{ExitCode, ExitStatus}};

use ascii_cleaner::{AsciiCleaner, AsciiCleanerError, AsciiCleanerResult};

use crate::cli::{CliError, CliResult};


fn main() -> ExitCode {
    ExitCode::SUCCESS
}
fn smain() -> CliResult<()> {
    let mut args = std::env::args();
    let maybe_verb = args.next();
    let maybe_input = args.next();

    let verb= maybe_verb.ok_or(CliError::MissingVerb)?;
    let path = maybe_input.ok_or(CliError::MissingInput)?;
    
    let mut file = File::open(path)?;
    let prg = AsciiCleaner::analyze(file)?;
    println!("Hello, world!");
    Ok(())
}



