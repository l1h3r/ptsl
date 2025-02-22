use ptsl_protos::types::FileLocation;
use std::path::Path;
use std::path::PathBuf;

// =============================================================================
// Session Path
// =============================================================================

/// Session file location.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionPath {
  pub(crate) online: bool,
  pub(crate) ospath: PathBuf,
}

impl SessionPath {
  #[inline]
  pub(crate) fn new(path: FileLocation) -> Self {
    Self {
      online: path.info.map_or(false, |info| info.is_online),
      ospath: path.path.into(),
    }
  }

  /// Returns a reference to the OS path.
  #[inline]
  pub fn ospath(&self) -> &Path {
    &self.ospath
  }

  /// Returns `true` if the session is stored online.
  #[inline]
  pub const fn online(&self) -> bool {
    self.online
  }
}
