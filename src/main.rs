mod cli;

use std::{fmt::Debug, path::PathBuf, process::ExitCode};

use ascii_cleaner::{AsciiCleaner, LogMode, ReplaceChar, WithBackup};

use crate::cli::{Cli, CliError, CliResult};

fn main() -> ExitCode {
    match smain() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            match &err {
                CliError::NoArgs | CliError::UnknownAction(_) => {
                    print_error(&err);
                    println!("{}", Cli::usage())
                }
                CliError::StdIo(_) => print_error(&err),
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
    let mut args = std::env::args();

    assert!(args.len() <= 5);

    let _ = args.next();

    let action = match args.next() {
        Some(action) => {
            match action.as_str() {
                "detect" | "remove" | "replace" => (),
                _ => return Err(CliError::UnknownAction(action)),
            };
            action
        }
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

    let mut log_mode = LogMode::No;
    let mut with_backup = WithBackup::BackupFile;

    let mut maybe_replace_char = Some(ReplaceChar::default());

    for item in args.into_iter().collect::<Vec<String>>() {
        if &item == "--no-backup" {
            with_backup = WithBackup::NoBackupFile
        }

        if &item == "--log-mode" {
            log_mode = LogMode::PrintOnEachFinding
        }

        if item.contains("--char=") {
            maybe_replace_char = item
                .split('=')
                .nth(1)
                .and_then(|s| s.chars().next())
                .filter(|c| AsciiCleaner::is_allowed_ascii(*c as char))
                .map(|c| c as u8)
                .map(|c| c.into());
        }
    }

    let action = match action.as_ref() {
        "detect" => Action::Detect,
        "remove" => Action::Remove(with_backup),
        "replace" => match maybe_replace_char {
            Some(replace_char) => Action::Replace(with_backup, replace_char),
            None => return Err(CliError::InvalidReplaceCharArg(action)),
        },
        _ => return Err(CliError::UnknownAction(action)),
    };

    // let ascii_cleaner = AsciiCleaner::new(path)?;

    let ascii_cleaner = if log_mode == LogMode::PrintOnEachFinding {
        AsciiCleaner::builder().file(path)?.log_mode().finish()
    } else {
        AsciiCleaner::builder().file(path)?.finish()
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
