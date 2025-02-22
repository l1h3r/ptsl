//! PTSL Protobuf Definitions.

#![allow(clippy::module_inception)]
#![deny(missing_docs)]

mod extension;

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
