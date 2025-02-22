use ptsl_client::error::Error;
use ptsl_client::error::Result;

use crate::property::Container;
use crate::property::RemoteSettings;

/// Shared behaviour for session properties.
pub trait RemoteProperty: Sized {
  /// The type of settings that stores this property.
  type Settings: RemoteSettings;

  /// Convert `settings` to a property container.
  #[inline]
  fn convert(settings: Self::Settings) -> Result<Container<Self>>
  where
    Self: TryFrom<<Self::Settings as RemoteSettings>::Data>,
    Self::Error: Into<Error>,
  {
    settings.convert().map(Into::into)
  }
}
