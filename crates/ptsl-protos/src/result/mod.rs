//! Command result types.

mod error;
mod header;
mod result;
mod status;

pub use self::error::CommandError;
pub use self::header::CommandHeader;
pub use self::result::CommandFail;
pub use self::result::CommandNone;
pub use self::result::CommandPass;
pub use self::result::CommandResult;
pub use self::status::CommandStatus;
