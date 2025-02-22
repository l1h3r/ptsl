//! Pro Tools Scripting Library (PTSL)

pub mod client {
  #[doc(inline)]
  pub use ptsl_client::client;

  #[doc(inline)]
  pub use ptsl_client::consts;

  #[doc(inline)]
  pub use ptsl_client::error;

  #[doc(inline)]
  pub use ptsl_client::types;
}

pub mod future {
  #[doc(inline)]
  pub use ptsl_future::retry;
}

pub mod protos {
  #[doc(inline)]
  pub use ptsl_protos::bridge;

  #[doc(inline)]
  pub use ptsl_protos::error;

  #[doc(inline)]
  pub use ptsl_protos::result;

  #[doc(inline)]
  pub use ptsl_protos::traits;

  #[doc(inline)]
  pub use ptsl_protos::types;
}
