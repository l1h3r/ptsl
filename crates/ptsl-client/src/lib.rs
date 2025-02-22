//! PTSL gRPC Transport

#![feature(async_fn_in_trait)]
#![feature(cfg_match)]
#![feature(exit_status_error)]
#![deny(missing_docs)]

mod tonic;

pub mod client;
pub mod consts;
pub mod error;
pub mod types;
