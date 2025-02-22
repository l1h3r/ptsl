use std::process::Command;
use std::process::Stdio;

use crate::consts::PTSL_APPNAME;
use crate::error::OsProcessError;

// =============================================================================
// Launch ProTools
// =============================================================================

/// Launch Pro Tools application.
///
/// # Errors
///
/// Returns [`Err`] if launching the application fails.
pub fn launch() -> Result<(), OsProcessError> {
  command()
    .stdin(Stdio::null()) // ignore stdin
    .stdout(Stdio::null()) // ignore stdout
    .stderr(Stdio::null()) // ignore stderr
    .status()
    .map_err(OsProcessError::Launch)?
    .exit_ok()
    .map_err(OsProcessError::Status)
}

// =============================================================================
// Platform Support
// =============================================================================

cfg_match! {
  cfg(target_os = "macos") => {
    #[allow(unused_results)]
    fn command() -> Command {
      let mut command: Command = Command::new("/usr/bin/open");
      command.arg("-g"); // Don't bring the app to the foreground
      command.arg("-a");
      command.arg(PTSL_APPNAME);
      command
    }
  }
  cfg(target_os = "windows") => {
    fn command() -> Command {
      unimplemented!("windows")
    }
  }
  _ => {
    fn command() -> Command {
      unimplemented!("unknown")
    }
  }
}
