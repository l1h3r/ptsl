//! Misc. transport types.

/// List of PTSL method versions.
pub type VersionList = &'static [(&'static str, i32)];

/// Version data request.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionType {
  /// PTSL client version.
  Client,
  /// PTSL client API versions.
  ClientAPI,
  /// PTSL server version.
  Server,
}

/// Version data response.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum VersionData {
  /// PTSL client version info.
  Client(VersionList),
  /// PTSL server version info.
  Server(i32),
}

/// PTSL SDK Versions.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SdkVersion {
  /// Version 2022.12.
  V2022_12,
  /// Version 2023.3.
  V2023_3,
  /// Version 2023.6.
  V2023_6,
  /// Version 2023.9.
  V2023_9,
}

impl SdkVersion {
  /// Returns a string representation of the SDK version.
  #[inline]
  pub const fn as_str(&self) -> &'static str {
    match self {
      Self::V2022_12 => "2022.12",
      Self::V2023_3 => "2023.3",
      Self::V2023_6 => "2023.6",
      Self::V2023_9 => "2023.9",
    }
  }
}
