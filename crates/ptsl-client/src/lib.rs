//! PTSL gRPC Transport

#![feature(cfg_match)]
#![feature(exit_status_error)]
#![allow(async_fn_in_trait)]
#![deny(missing_docs)]

mod tonic;

pub mod client;
pub mod consts;
pub mod error;
pub mod types;
