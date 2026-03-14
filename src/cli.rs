mod result;
mod usage;

use std::{ops::Not, path::PathBuf};

use ascii_cleaner::{Action, AsciiCleaner, BackupFile, ReplaceChar, RunningMode, WithBackup};

pub(crate) use self::result::{CliError, CliResult};

pub(crate) struct Cli {
    pub(crate) run_mode: RunningMode,
    pub(crate) file_path: PathBuf,
    pub(crate) action: Action,
}
impl Cli {
    pub(crate) fn parse() -> CliResult<Self> {
        let mut args = std::env::args();

        assert!(args.len() <= 8);

        let _ = args.next();

        let action_str = match args.next() {
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

        let file_path = args
            .next()
            .map(PathBuf::from)
            .ok_or(CliError::MissingFilePath)?;

        if file_path.is_file().not() {
            return Err(CliError::InvalidFilePath);
        }

        let mut run_mode = RunningMode::ReportAlways;
        let mut backup_file_arg = None;
        let mut is_quiet_mode = false;
        let mut accept_bac = false;

        let mut replace_char = Some(ReplaceChar::default());

        for current_arg in args.into_iter().collect::<Vec<String>>() {
            if &current_arg == "--quiet" {
                is_quiet_mode = true;
            }
            if &current_arg == format!("--accept-{}", BackupFile::FILE_EXTENSION).as_str() {
                accept_bac = true;
            }
            if &current_arg == "--no-backup" {
                backup_file_arg = Some(WithBackup::NoBackupFile);
            }

            if &current_arg == "--log-mode" {
                run_mode = RunningMode::PrintOnEachFinding
            }

            if current_arg.contains("--char=") {
                replace_char = current_arg
                    .split('=')
                    .nth(1)
                    .and_then(|s| s.chars().next())
                    .filter(|c| AsciiCleaner::is_allowed_ascii(*c as char))
                    .map(|c| c as u8)
                    .map(|c| c.into());
            }
        }

        match file_path.extension() {
            Some(ext) => {
                if accept_bac == false && ext == BackupFile::FILE_EXTENSION {
                    return Err(CliError::BackupFileExtension);
                }
            }
            None => (),
        };

        if is_quiet_mode {
            run_mode = RunningMode::Quiet
        }

        let with_backup = match backup_file_arg {
            Some(no_backup) => no_backup,
            None => WithBackup::BackupFile(BackupFile::new(&file_path)?),
        };

        let action = match action_str.as_ref() {
            "detect" => Action::detect(),
            "remove" => Action::remove(with_backup),
            "replace" => match replace_char {
                Some(replace_char) => Action::replace(with_backup, replace_char),
                None => return Err(CliError::InvalidReplaceCharArg(action_str)),
            },
            _ => return Err(CliError::UnknownAction(action_str)),
        };

        Ok(Cli {
            run_mode,
            file_path,
            action,
        })
    }
}
