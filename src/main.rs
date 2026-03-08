mod cli;

use std::{fmt::Debug, process::ExitCode};

use ascii_cleaner::{Action, AsciiCleaner, LogMode};

use crate::cli::{Cli, CliError, CliResult};

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            match &err {
                CliError::NoArgs | CliError::UnknownAction(_) => {
                    print_error(&err);
                    eprintln!("{}", Cli::usage())
                }

                CliError::AsciiCleaner(_) => print_error(&err),
                _ => print_error(&err),
            };
            err.into()
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

    let ascii_cleaner = if log_mode == LogMode::PrintOnEachFinding {
        AsciiCleaner::builder()
            .action(action.clone())?
            .file(file_path)?
            .log_mode()
            .finish()
    } else {
        AsciiCleaner::new(action.clone(), file_path)?
    };

    let report = match action {
        Action::Detect => ascii_cleaner.detect(),
        Action::Remove(_) => ascii_cleaner.remove(),
        Action::Replace(_, _) => ascii_cleaner.replace(),
    }?;

    if log_mode == LogMode::No {
        println!("{report}");
    }

    Ok(())
}
