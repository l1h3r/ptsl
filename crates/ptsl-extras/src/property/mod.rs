//! Extensions for Pro Tools session properties.

mod container;
mod properties;
mod traits;

pub use self::container::Container;
pub use self::properties::AudioFormat;
pub use self::properties::AudioRatePull;
pub use self::properties::BitDepth;
pub use self::properties::EditMode;
pub use self::properties::EditTool;
pub use self::properties::FeetFramesRate;
pub use self::properties::Interleaved;
pub use self::properties::Length;
pub use self::properties::PlaybackMode;
pub use self::properties::RatePull;
pub use self::properties::RecordMode;
pub use self::properties::RecordModeArmed;
pub use self::properties::StartTime;
pub use self::properties::TimeCodeRate;
pub use self::properties::TransportState;
pub use self::properties::VideoRatePull;
pub use self::traits::PropertyGet;
pub use self::traits::PropertySet;
pub use self::traits::RemoteProperty;
pub use self::traits::RemoteSettings;
