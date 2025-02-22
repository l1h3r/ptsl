use ptsl_client::client::Client;
use ptsl_client::error::Result;

use crate::property::PropertyGet;

/// Implemented by properties that support updating values.
pub trait PropertySet: PropertyGet {
  /// Set the current value of the property.
  async fn set(client: &mut Client, value: Self) -> Result<()>;
}
