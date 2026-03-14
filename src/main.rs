mod cli;

use std::{fmt::Debug, process::ExitCode};

use ascii_cleaner::{Action, AsciiCleaner, AsciiCleanerError, AsciiCleanerResult, RunningMode};

use crate::cli::{Cli, CliError};

#[allow(unused)]
#[derive(Debug)]
struct Error<'a> {
    cli: Cli,
    error: &'a AsciiCleanerError,
}

fn main() -> ExitCode {
    let cli = match Cli::parse() {
        Ok(v) => v,
        Err(cli_error) => {
            match &cli_error {
                CliError::NoArgs | CliError::UnknownAction(_) => {
                    eprintln!("CliError: {:?}", &cli_error);
                    eprintln!("{}", Cli::usage())
                }

                CliError::AsciiCleaner(_) => eprintln!("CliError: {:?}", &cli_error),
                _ => eprintln!("CliError: {:?}", &cli_error),
            };
            return cli_error.into();
        }
    };
    match smain(&cli) {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            print_error(Error { cli, error: &err });

            let cli_error: CliError = err.into();

            cli_error.into()
        }
    }
}
// TODO: Implement json | text output mode
fn print_error(error: Error) {
    eprintln!("{error:?}")
}

fn smain(cli: &Cli) -> AsciiCleanerResult<()> {
    let ascii_cleaner = match cli.run_mode {
        RunningMode::PrintOnEachFinding => AsciiCleaner::builder()
            .action(cli.action.clone())?
            .file(cli.file_path.clone())?
            .print_each_finding()
            .finish(),
        RunningMode::ReportAlways => AsciiCleaner::new(cli.action.clone(), cli.file_path.clone())?,
        RunningMode::Quiet => AsciiCleaner::builder()
            .action(cli.action.clone())?
            .file(cli.file_path.clone())?
            .quiet_mode()
            .finish(),
    };

    let report = match cli.action {
        Action::Detect => ascii_cleaner.detect(),
        Action::Remove(_) => ascii_cleaner.remove(),
        Action::Replace(_, _) => ascii_cleaner.replace(),
    }?;

    match cli.run_mode {
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
