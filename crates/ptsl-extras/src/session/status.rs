/// The current status of a Pro Tools session.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
  /// Session is open and able to receive commands.
  Active,
  /// Session is closed and cannot receive commands.
  Closed,
}

impl Status {
  #[inline]
  pub(crate) const fn assert_active(self) {
    let Self::Active = self else {
      panic!("Session Not Active");
    };
  }

  #[inline]
  pub(crate) const fn assert_closed(self) {
    let Self::Closed = self else {
      panic!("Session Not Closed");
    };
  }
}
