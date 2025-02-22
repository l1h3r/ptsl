use crate::error::Result;
use crate::traits::Decode;
use crate::types::CommandError as ProtoError;
use crate::types::CommandErrorType;

// =============================================================================
// Command Error
// =============================================================================

/// Command execution error.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandError {
  kind: CommandErrorType,
  message: String,
  warning: bool,
}

impl CommandError {
  /// Create a new `CommandError`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if `string` is not valid [`CommandError`][ProtoError] JSON.
  pub fn new(string: &str) -> Result<Self> {
    let error: ProtoError = ProtoError::decode(string)?;

    Ok(Self {
      kind: error.command_error_type.try_into()?,
      message: error.command_error_message,
      warning: error.is_warning,
    })
  }

  /// Returns the command error type.
  pub const fn kind(&self) -> CommandErrorType {
    self.kind
  }

  /// Returns the error message (Note: might be empty).
  pub fn message(&self) -> &str {
    self.message.as_str()
  }

  /// Indicates that this error is a warning and command execution succeeded.
  pub const fn warning(&self) -> bool {
    self.warning
  }
}
