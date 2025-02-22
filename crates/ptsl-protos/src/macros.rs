#[doc(hidden)]
#[macro_export]
macro_rules! feature {
  (
    #![cfg(feature = $feature:literal)]
    $($item:item)*
  ) => {
    $(
      #[cfg(feature = $feature)]
      #[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
      $item
    )*
  };
}
