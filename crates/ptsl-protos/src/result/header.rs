use crate::error::Result;
use crate::types::CommandId;

// =============================================================================
// Command Header
// =============================================================================

/// Command execution header.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandHeader {
  command: CommandId,
  task_id: String,
}

impl CommandHeader {
  /// Create a new `CommandHeader`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if `command` is not a valid [`CommandId`].
  #[inline]
  pub fn new(command: i32, task_id: String) -> Result<Self> {
    Ok(Self {
      command: command.try_into()?,
      task_id,
    })
  }

  /// Returns the command type.
  #[inline]
  pub const fn command(&self) -> CommandId {
    self.command
  }

  /// Returns the command identifier.
  #[inline]
  pub fn task_id(&self) -> &str {
    self.task_id.as_str()
  }
}
