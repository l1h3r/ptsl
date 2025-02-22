//! Traits for working with protobuf types.

mod decode;
mod encode;
mod message;

pub use self::decode::Decode;
pub use self::encode::Encode;
pub use self::message::Message;
