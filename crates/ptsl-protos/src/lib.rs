//! PTSL Protobuf Definitions.

#![allow(async_fn_in_trait)]
#![allow(clippy::module_inception)]
#![deny(missing_docs)]

#[macro_use]
mod macros;

mod extension;

pub mod bridge;
pub mod error;
pub mod result;
pub mod traits;

pub mod types {
  //! Compiled protobuf types.
  #![allow(deprecated)]
  #![allow(missing_docs)]
  #![allow(clippy::deprecated_semver)]
  #![allow(clippy::tabs_in_doc_comments)]
  tonic::include_proto!("ptsl");
}
