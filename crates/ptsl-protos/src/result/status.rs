use crate::error::Result;
use crate::types::TaskStatus;

// =============================================================================
// Command Status
// =============================================================================

/// Command execution status.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandStatus {
  progress: i32,
  status: TaskStatus,
}

impl CommandStatus {
  /// Create a new `CommandStatus`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if `status` is not a valid [`TaskStatus`].
  #[inline]
  pub fn new(progress: i32, status: i32) -> Result<Self> {
    Ok(Self {
      progress,
      status: status.try_into()?,
    })
  }

  /// Returns the command execution progress.
  #[inline]
  pub const fn progress(&self) -> i32 {
    self.progress
  }

  /// Returns the command execution status.
  #[inline]
  pub const fn status(&self) -> TaskStatus {
    self.status
  }
}
