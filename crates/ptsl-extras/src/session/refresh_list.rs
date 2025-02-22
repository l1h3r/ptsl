use ptsl_protos::types::RefreshTargetAudioFilesResponseBody;

/// Details describing a set of refreshed audio files.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RefreshList {
  success: i32,
  failure: i32,
  failure_list: Vec<String>,
}

impl RefreshList {
  pub(crate) fn new(value: RefreshTargetAudioFilesResponseBody) -> Self {
    Self {
      success: value.success_count,
      failure: value.failure_count,
      failure_list: value.failure_list,
    }
  }

  /// Returns the total number of successful refreshes.
  #[inline]
  pub const fn success(&self) -> i32 {
    self.success
  }

  /// Returns the total number of failed refreshes.
  #[inline]
  pub const fn failure(&self) -> i32 {
    self.failure
  }

  /// Returns a list of files that failed to refresh.
  #[inline]
  pub fn failure_list(&self) -> &[String] {
    self.failure_list.as_slice()
  }
}
