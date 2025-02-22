//! PTSL Protobuf Definitions.

#![deny(missing_docs)]

pub mod error;
pub mod traits;

pub mod types {
  //! Compiled protobuf types.
  #![allow(deprecated)]
  #![allow(missing_docs)]
  #![allow(clippy::deprecated_semver)]
  #![allow(clippy::tabs_in_doc_comments)]
  tonic::include_proto!("ptsl");
}
