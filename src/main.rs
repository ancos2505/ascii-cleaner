mod cli;

use std::{fmt::Debug, process::ExitCode};

use ascii_cleaner::{AsciiCleaner, LogMode, ReplaceChar, WithBackup};

use crate::cli::{Cli, CliError, CliResult};

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            match &err {
                CliError::NoArgs => println!("{}", Cli::usage()),
                CliError::UnknownAction(_) => {
                    print_error(&err);
                    eprintln!("{}", Cli::usage())
                }
                CliError::StdIo(_) => print_error(&err),
                CliError::AsciiCleaner(_) => print_error(&err),
                _ => print_error(&err),
            };

            match &err {
                CliError::NoArgs => ExitCode::SUCCESS,
                _ => err.into(),
            }
        }
    }
}
fn print_error<D: Debug>(error: &D) {
    eprintln!("Error: {error:?}")
}

fn smain() -> CliResult<()> {
    let Cli {
        log_mode,
        file_path,
        action,
    } = Cli::parse()?;

    // let ascii_cleaner = AsciiCleaner::new(path)?;

    let ascii_cleaner = if log_mode == LogMode::PrintOnEachFinding {
        AsciiCleaner::builder().file(file_path)?.log_mode().finish()
    } else {
        AsciiCleaner::builder().file(file_path)?.finish()
    };

    let report = match action {
        Action::Detect => ascii_cleaner.detect(),
        Action::Remove(with_backup) => ascii_cleaner.remove(with_backup),
        Action::Replace(with_backup, replace_char) => {
            ascii_cleaner.replace(with_backup.into(), replace_char.into())
        }
    }?;

    if log_mode == LogMode::No {
        println!("{report}");
    }

    Ok(())
}

#[derive(Debug)]
pub(crate) enum Action {
    Detect,
    Remove(WithBackup),
    Replace(WithBackup, ReplaceChar),
}
