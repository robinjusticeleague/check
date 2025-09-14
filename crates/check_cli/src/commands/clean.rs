use crate::commands::daemon::default_check_log_path;
use crate::{CliDiagnostic, CliSession};
use check_flags::check_env;
use camino::Utf8PathBuf;
use std::fs::{create_dir, remove_dir_all};

/// Runs the clean command
pub fn clean(_cli_session: CliSession) -> Result<(), CliDiagnostic> {
    let logs_path = check_env()
        .check_log_path
        .value()
        .map_or(default_check_log_path(), Utf8PathBuf::from);
    remove_dir_all(logs_path.clone()).and_then(|_| create_dir(logs_path))?;
    Ok(())
}
