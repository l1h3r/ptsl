use hyper::rt::Executor as HyperExecutor;
use std::future::Future;
use std::sync::Arc;

use crate::tonic::BoxFuture;

#[derive(Clone, Copy)]
struct Tokio;

impl<F> HyperExecutor<F> for Tokio
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  #[inline]
  fn execute(&self, future: F) {
    tokio::spawn(future);
  }
}

#[derive(Clone)]
pub struct Executor {
  inner: Arc<dyn HyperExecutor<BoxFuture<'static, ()>> + Send + Sync + 'static>,
}

impl Executor {
  #[inline]
  pub fn tokio() -> Self {
    Self {
      inner: Arc::new(Tokio),
    }
  }
}

impl HyperExecutor<BoxFuture<'static, ()>> for Executor {
  #[inline]
  fn execute(&self, future: BoxFuture<'static, ()>) {
    self.inner.execute(future);
  }
}
