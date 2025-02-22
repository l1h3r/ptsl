use crate::types::TaskStatus;

// =============================================================================
// Task Status Extensions
// =============================================================================

impl TaskStatus {
  /// Returns `true` if the task is queued.
  #[inline]
  pub const fn is_queued(&self) -> bool {
    matches!(self, Self::Queued)
  }

  /// Returns `true` if the task is pending.
  #[inline]
  pub const fn is_pending(&self) -> bool {
    matches!(self, Self::Pending)
  }

  /// Returns `true` if the task is in-progress.
  #[inline]
  pub const fn is_progress(&self) -> bool {
    matches!(self, Self::InProgress)
  }

  /// Returns `true` if the task completed.
  #[inline]
  pub const fn is_completed(&self) -> bool {
    matches!(self, Self::Completed)
  }

  /// Returns `true` if the task failed.
  #[inline]
  pub const fn is_failed(&self) -> bool {
    matches!(self, Self::Failed)
  }

  /// Returns `true` if the task is waiting for user input.
  #[inline]
  pub const fn is_waiting(&self) -> bool {
    matches!(self, Self::WaitingForUserInput)
  }

  /// Returns `true` if the task completed with an invalid server response.
  #[inline]
  pub const fn is_completed_invalid(&self) -> bool {
    matches!(self, Self::CompletedWithBadResponse)
  }

  /// Returns `true` if the task failed with an invalid server response.
  #[inline]
  pub const fn is_failed_invalid(&self) -> bool {
    matches!(self, Self::FailedWithBadErrorResponse)
  }

  /// Returns `true` if the task completed, possibly with an invalid response.
  #[inline]
  pub const fn is_completed_any(&self) -> bool {
    self.is_completed() || self.is_completed_invalid()
  }

  /// Returns `true` if the task failed, possibly with an invalid response.
  #[inline]
  pub const fn is_failed_any(&self) -> bool {
    self.is_failed() || self.is_failed_invalid()
  }

  /// Returns `true` if the task ended, possibly with an invalid response.
  #[inline]
  pub const fn is_done(&self) -> bool {
    self.is_completed_any() || self.is_failed_any()
  }
}
