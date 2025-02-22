//! PTSL Protobuf Definitions

pub mod error;

pub mod types {
  //! Compiled protobuf types.
  #![allow(deprecated)]
  #![allow(clippy::deprecated_semver)]
  #![allow(clippy::tabs_in_doc_comments)]
  tonic::include_proto!("ptsl");
}
