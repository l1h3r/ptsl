macro_rules! define_property {
  (@copy #[no_copy] $_name:ident) => {
    // do nothing
  };

  (@copy $name:ident) => {
    impl Copy for $name {}
  };

  (
    pub enum $name:ident: $protobuf:ident {
      $(const $variant:ident = $discriminant:ident;)+
    }
  ) => {
    #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(u8)]
    pub enum $name {
      $($variant),+
    }

    impl $name {
      #[doc(hidden)]
      pub const SIZE: usize = ::core::mem::variant_count::<Self>();

      #[doc(hidden)]
      pub const LIST: [Self; Self::SIZE] = [$(Self::$variant),+];
    }

    impl ::core::fmt::Display for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        match self {
          $(Self::$variant => f.write_str(stringify!($variant))),+
        }
      }
    }

    impl From<::ptsl_protos::types::$protobuf> for $name {
      #[inline]
      fn from(other: ::ptsl_protos::types::$protobuf) -> Self {
        match other {
          $(::ptsl_protos::types::$protobuf::$discriminant => Self::$variant),+
        }
      }
    }

    impl From<$name> for ::ptsl_protos::types::$protobuf {
      #[inline]
      fn from(other: $name) -> Self {
        match other {
          $($name::$variant => Self::$discriminant),+
        }
      }
    }

    impl TryFrom<i32> for $name {
      type Error = ::ptsl_client::error::Error;

      #[inline]
      fn try_from(other: i32) -> Result<Self, Self::Error> {
        ::ptsl_protos::types::$protobuf::try_from(other)
          .map(Into::into)
          .map_err(::ptsl_protos::error::Error::from)
          .map_err(::ptsl_client::error::Error::from)
      }
    }
  };

  (
    pub struct $name:ident {
      @inner = $inner:ident;
      @outer = $outer:ident;
    }
  ) => {
    #[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct $name {
      inner: $inner,
    }

    impl $name {
      /// Create a new session property.
      #[inline]
      pub const fn new(inner: $inner) -> Self {
        Self { inner }
      }

      /// Retrieve the inner value from this property.
      #[inline]
      pub const fn into_inner(self) -> $inner {
        self.inner
      }
    }

    impl ::core::fmt::Debug for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{}({:?})", stringify!($name), self.inner)
      }
    }

    impl ::core::fmt::Display for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        write!(f, "{}({})", stringify!($name), self.inner)
      }
    }

    impl From<$inner> for $name {
      #[inline]
      fn from(other: $inner) -> Self {
        Self::new(other)
      }
    }

    impl From<$name> for $inner {
      #[inline]
      fn from(other: $name) -> Self {
        other.into_inner()
      }
    }

    impl From<::ptsl_protos::types::$outer> for $name {
      #[inline]
      fn from(other: ::ptsl_protos::types::$outer) -> Self {
        Self::new(other.into())
      }
    }

    impl From<$name> for ::ptsl_protos::types::$outer {
      #[inline]
      fn from(other: $name) -> Self {
        other.into_inner().into()
      }
    }

    impl TryFrom<i32> for $name {
      type Error = ::ptsl_client::error::Error;

      #[inline]
      fn try_from(other: i32) -> Result<Self, Self::Error> {
        $inner::try_from(other).map(Self::new)
      }
    }
  };

  (
    $(#[$meta:tt])*
    pub struct $name:ident($inner:ty);
  ) => {
    #[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    pub struct $name {
      inner: $inner,
    }

    define_property!(@copy $(#[$meta])* $name);

    impl $name {
      /// Create a new session property.
      #[inline]
      pub const fn new(inner: $inner) -> Self {
        Self { inner }
      }

      /// Retrieve the inner value from this property.
      #[inline]
      pub fn into_inner(self) -> $inner {
        self.inner
      }
    }

    impl ::core::fmt::Debug for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Debug::fmt(&self.inner, f)
      }
    }

    impl ::core::fmt::Display for $name {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::fmt::Display::fmt(&self.inner, f)
      }
    }

    impl From<$inner> for $name {
      #[inline]
      fn from(other: $inner) -> Self {
        Self::new(other)
      }
    }

    impl From<$name> for $inner {
      #[inline]
      fn from(other: $name) -> Self {
        other.into_inner()
      }
    }
  };

  (
    pub struct $name:ident: $protobuf:ident as $repr:ty {
      $(const $flag:ident = $value:expr => $discriminant:ident;)+
    }
  ) => {
    ::bitflags::bitflags! {
      #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
      pub struct $name: $repr {
        $(const $flag = $value;)+
      }
    }

    impl $name {
      fn expand(self) -> impl Iterator<Item = ::ptsl_protos::types::$protobuf> + 'static {
        self.iter().flat_map(Self::convert_into)
      }

      #[inline]
      fn convert_into(self) -> Option<::ptsl_protos::types::$protobuf> {
        $(
          if self.contains(Self::$flag) {
            return Some(::ptsl_protos::types::$protobuf::$discriminant);
          }
        )+

        None
      }

      #[inline]
      fn convert_from(value: $repr) -> Option<Self> {
        $(
          if value == ::ptsl_protos::types::$protobuf::$discriminant as $repr {
            return Some(Self::$flag);
          }
        )+

        None
      }
    }

    impl ::core::fmt::Display for PlaybackMode {
      fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::bitflags::parser::to_writer(self, f)
      }
    }

    impl From<::ptsl_protos::types::$protobuf> for $name {
      #[inline]
      fn from(other: ::ptsl_protos::types::$protobuf) -> Self {
        match other {
          $(::ptsl_protos::types::$protobuf::$discriminant => Self::$flag,)+
        }
      }
    }

    impl From<$name> for Vec<::ptsl_protos::types::$protobuf> {
      #[inline]
      fn from(other: $name) -> Self {
        other.expand().collect()
      }
    }

    impl From<Vec<::ptsl_protos::types::$protobuf>> for $name {
      #[inline]
      fn from(other: Vec<::ptsl_protos::types::$protobuf>) -> $name {
        other.into_iter().map(Into::into).collect()
      }
    }

    impl From<Vec<i32>> for $name {
      #[inline]
      fn from(other: Vec<i32>) -> Self {
        other.into_iter().flat_map(Self::convert_from).collect()
      }
    }
  };
}

macro_rules! define_settings {
  ($name:ident => $settings:ident, $data:ty, $iter:ty) => {
    impl $crate::property::RemoteSettings for ::ptsl_protos::types::$settings {
      type Data = $data;
      type Iter = $iter;

      #[inline]
      fn expand(self) -> (Self::Data, Self::Iter) {
        (self.current_setting, self.possible_settings)
      }
    }

    impl $crate::property::RemoteProperty for $name {
      type Settings = ::ptsl_protos::types::$settings;
    }
  };
  ($name:ident => $settings:ident, $data:ty) => {
    define_settings!($name => $settings, $data, Vec<$data>);
  };
  ($name:ident => $settings:ident) => {
    define_settings!($name => $settings, i32);
  };
}

macro_rules! define_ptsl_get {
  ($name:ident => $function:ident) => {
    impl $crate::property::PropertyGet for $name {
      #[inline]
      async fn get(
        client: &mut ::ptsl_client::client::Client,
      ) -> ::ptsl_client::error::Result<$crate::property::Container<Self>> {
        ::ptsl_protos::bridge::CommandExt::$function(client)
          .await
          .and_then($crate::property::RemoteProperty::convert)
      }
    }
  };
}

macro_rules! define_ptsl_set {
  ($name:ident => $function:ident $( ( $($param:expr),+ ) )?) => {
    impl $crate::property::PropertySet for $name {
      #[inline]
      async fn set(
        client: &mut ::ptsl_client::client::Client,
        value: Self,
      ) -> ::ptsl_client::error::Result<()> {
        ::ptsl_protos::bridge::CommandExt::$function(
          client,
          value.into(),
          $($($param),+)?
        ).await
      }
    }
  };
}
