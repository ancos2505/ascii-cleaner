mod cli;

use std::{fmt::Debug, process::ExitCode};

use ascii_cleaner::{Action, AsciiCleaner, RunningMode};

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
// TODO: Implement json | text output mode
fn print_error<D: Debug>(error: &D) {
    eprintln!("Error: {error:?}")
}

fn smain() -> CliResult<()> {
    let Cli {
        run_mode,
        file_path,
        action,
    } = Cli::parse()?;

    let ascii_cleaner = match run_mode {
        RunningMode::PrintOnEachFinding => AsciiCleaner::builder()
            .action(action.clone())?
            .file(file_path)?
            .print_each_finding()
            .finish(),
        RunningMode::ReportAlways => AsciiCleaner::new(action.clone(), file_path)?,
        RunningMode::Quiet => AsciiCleaner::builder()
            .action(action.clone())?
            .file(file_path)?
            .quiet_mode()
            .finish(),
    };

    let report = match action {
        Action::Detect => ascii_cleaner.detect(),
        Action::Remove(_) => ascii_cleaner.remove(),
        Action::Replace(_, _) => ascii_cleaner.replace(),
    }?;

    match run_mode {
        RunningMode::PrintOnEachFinding => (),
        RunningMode::ReportAlways => {
            println!("{report}");
        }
        RunningMode::Quiet => {
            if report.findings.len() > 0 {
                println!("{report}");
            }
        }
    };

    Ok(())
}
