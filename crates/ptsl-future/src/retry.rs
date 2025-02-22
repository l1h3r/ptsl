//! Retry futures extension.

use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::time::sleep;
use tokio::time::Sleep;

// =============================================================================
// Retry Extension
// =============================================================================

pin_project! {
  /// A future with retry capabilities.
  #[project = RetryProjection]
  pub struct Retry<C, F> {
    create: C,
    config: Config,
    count: u32,
    #[pin]
    state: State<F>,
  }
}

impl<C, F> Retry<C, F> {
  /// Create a new retryable future.
  #[inline]
  pub fn new(create: C, attempts: u32) -> Self {
    Self::with_config(create, Config::new(attempts))
  }

  /// Create a new retryable future with the given `config`.
  #[inline]
  pub fn with_config(create: C, config: Config) -> Self {
    Self {
      create,
      config,
      count: 0,
      state: State::Init,
    }
  }

  /// Set the max delay between retries.
  #[inline]
  pub const fn max_delay(mut self, value: Duration) -> Self {
    self.config = self.config.max_delay(value);
    self
  }

  /// Set the backoff strategy for retries.
  #[inline]
  pub const fn strategy(mut self, value: Backoff) -> Self {
    self.config = self.config.strategy(value);
    self
  }

  /// Use [`Exponential`][Exponential] as the backoff strategy.
  #[inline]
  pub const fn exponential(self, delay: Duration) -> Self {
    self.strategy(Backoff::new_exponential(delay))
  }

  /// Use [`Fixed`][Fixed] as the backoff strategy.
  #[inline]
  pub const fn fixed(self, delay: Duration) -> Self {
    self.strategy(Backoff::new_fixed(delay))
  }

  /// Use [`Instant`][Instant] as the backoff strategy.
  #[inline]
  pub const fn instant(self) -> Self {
    self.strategy(Backoff::new_instant())
  }

  /// Use [`Linear`][Linear] as the backoff strategy.
  #[inline]
  pub const fn linear(self, delay: Duration) -> Self {
    self.strategy(Backoff::new_linear(delay))
  }
}

impl<C, F, T, E> Future for Retry<C, F>
where
  C: FnMut() -> F,
  F: Future<Output = Result<T, E>>,
{
  type Output = Result<T, E>;

  fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
    loop {
      let this: RetryProjection<'_, C, F> = self.as_mut().project();

      let next: State<F> = match this.state.project() {
        StateProjection::Init => State::Wait {
          inner: (this.create)(),
        },
        StateProjection::Wait { inner } => match inner.poll(context) {
          Poll::Pending => {
            return Poll::Pending;
          }
          Poll::Ready(Ok(output)) => {
            return Poll::Ready(Ok(output));
          }
          Poll::Ready(Err(error)) => {
            if *this.count == this.config.max_retry {
              return Poll::Ready(Err(error));
            }

            *this.count += 1;

            State::Time {
              inner: sleep(this.config.delay(*this.count)),
            }
          }
        },
        StateProjection::Time { inner } => match inner.poll(context) {
          Poll::Pending => {
            return Poll::Pending;
          }
          Poll::Ready(()) => State::Wait {
            inner: (this.create)(),
          },
        },
      };

      self.as_mut().project().state.set(next);
    }
  }
}

// =============================================================================
// Retry State
// =============================================================================

pin_project! {
  #[project = StateProjection]
  enum State<F> {
    Init,
    Wait { #[pin] inner: F },
    Time { #[pin] inner: Sleep },
  }
}

// =============================================================================
// Retry Configuration
// =============================================================================

/// Retry configuration.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Config {
  max_delay: Option<Duration>,
  max_retry: u32,
  strategy: Backoff,
}

impl Config {
  /// Create a new configuration with a maximum number of retries.
  #[inline]
  pub const fn new(max_retry: u32) -> Self {
    Self {
      max_delay: None,
      max_retry,
      strategy: Backoff::new_instant(),
    }
  }

  /// Set the max delay between retries.
  #[inline]
  pub const fn max_delay(mut self, value: Duration) -> Self {
    self.max_delay = Some(value);
    self
  }

  /// Set the backoff strategy for retries.
  #[inline]
  pub const fn strategy(mut self, value: Backoff) -> Self {
    self.strategy = value;
    self
  }

  /// Use [`Exponential`][Exponential] as the backoff strategy.
  #[inline]
  pub const fn exponential(self, delay: Duration) -> Self {
    self.strategy(Backoff::new_exponential(delay))
  }

  /// Use [`Fixed`][Fixed] as the backoff strategy.
  #[inline]
  pub const fn fixed(self, delay: Duration) -> Self {
    self.strategy(Backoff::new_fixed(delay))
  }

  /// Use [`Instant`][Instant] as the backoff strategy.
  #[inline]
  pub const fn instant(self) -> Self {
    self.strategy(Backoff::new_instant())
  }

  /// Use [`Linear`][Linear] as the backoff strategy.
  #[inline]
  pub const fn linear(self, delay: Duration) -> Self {
    self.strategy(Backoff::new_linear(delay))
  }

  #[inline]
  fn delay(&mut self, attempt: u32) -> Duration {
    let mut duration: Duration = self.strategy.delay(attempt);

    if let Some(maximum) = self.max_delay {
      duration = duration.min(maximum);
    }

    duration
  }
}

// =============================================================================
// Backoff Strategy
// =============================================================================

/// Backoff strategy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Backoff {
  /// Exponential backoff.
  Exponential(Exponential),
  /// Fixed interval backoff.
  Fixed(Fixed),
  /// Instant backoff (retry immediately).
  Instant(Instant),
  /// Linear backoff.
  Linear(Linear),
}

impl Backoff {
  /// Create a new [`Exponential`][Exponential] backoff strategy.
  #[inline]
  pub const fn new_exponential(delay: Duration) -> Self {
    Self::Exponential(Exponential::new(delay))
  }

  /// Create a new [`Fixed`][Fixed] backoff strategy.
  #[inline]
  pub const fn new_fixed(delay: Duration) -> Self {
    Self::Fixed(Fixed::new(delay))
  }

  /// Create a new [`Instant`][Instant] backoff strategy.
  #[inline]
  pub const fn new_instant() -> Self {
    Self::Instant(Instant)
  }

  /// Create a new [`Linear`][Linear] backoff strategy.
  #[inline]
  pub const fn new_linear(delay: Duration) -> Self {
    Self::Linear(Linear::new(delay))
  }

  #[inline]
  fn delay(&mut self, attempt: u32) -> Duration {
    match self {
      Self::Exponential(inner) => inner.delay(),
      Self::Fixed(inner) => inner.delay(),
      Self::Instant(inner) => inner.delay(),
      Self::Linear(inner) => inner.delay(attempt),
    }
  }
}

// =============================================================================
// Exponential Backoff
// =============================================================================

/// Exponential backoff strategy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Exponential {
  delay: Duration,
}

impl Exponential {
  /// Create a new [`Exponential`][Self] backoff strategy.
  #[inline]
  pub const fn new(delay: Duration) -> Self {
    Self { delay }
  }

  #[inline]
  fn delay(&mut self) -> Duration {
    let delay: Duration = self.delay;

    self.delay = self.delay.saturating_mul(2);

    delay
  }
}

// =============================================================================
// Instant Backoff
// =============================================================================

/// Instant backoff strategy (retry immediately).
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Instant;

impl Instant {
  #[inline]
  const fn delay(&self) -> Duration {
    Duration::new(0, 0)
  }
}

// =============================================================================
// Fixed Interval Backoff
// =============================================================================

/// Fixed interval backoff strategy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Fixed {
  delay: Duration,
}

impl Fixed {
  /// Create a new [`Fixed`][Self] backoff strategy.
  #[inline]
  pub const fn new(delay: Duration) -> Self {
    Self { delay }
  }

  #[inline]
  const fn delay(&self) -> Duration {
    self.delay
  }
}

// =============================================================================
// Linear Backoff
// =============================================================================

/// Linear backoff strategy.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Linear {
  delay: Duration,
}

impl Linear {
  /// Create a new [`Linear`][Self] backoff strategy.
  #[inline]
  pub const fn new(delay: Duration) -> Self {
    Self { delay }
  }

  #[inline]
  const fn delay(&self, attempt: u32) -> Duration {
    self.delay.saturating_mul(attempt)
  }
}
