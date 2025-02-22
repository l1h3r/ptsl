use crate::traits::Decode;
use crate::traits::Encode;
use crate::types::CommandId;

// =============================================================================
// Message
// =============================================================================

/// Extension trait to unify command requests and responses.
pub trait Message {
  /// The message command type.
  const TYPE: CommandId;

  /// Whether the message should periodically ping the gRPC server for progress.
  const SEND_PINGS: bool = false;

  /// Whether the message should be send as a gRPC stream.
  const VIA_STREAM: bool = true;

  /// The protobuf type sent to the server.
  type Send: Send + Encode;

  /// The protobuf type received from the server.
  type Recv: Send + for<'de> Decode<'de>;
}
