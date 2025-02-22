use ptsl_client::client::Client;
use ptsl_client::error::Result;

use crate::property::Container;
use crate::property::RemoteProperty;

/// Implemented by properties that support retrieving values.
pub trait PropertyGet: RemoteProperty {
  /// Get the current/allowed values of the property.
  async fn get(client: &mut Client) -> Result<Container<Self>>;
}
