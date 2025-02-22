//! UTF-8 Path/PathBuf

use std::borrow::Borrow;
use std::borrow::Cow;
use std::cmp::Ordering;
use std::env::var_os;
use std::error::Error;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::path::Component;
use std::path::Components;
use std::path::Path;
use std::path::PathBuf;

// =============================================================================
// ProTools Path
// =============================================================================

/// A UTF-8 path.
#[repr(transparent)]
pub struct PtPath(Path);

impl PtPath {
  /// Create a new `PtPath` from a string slice.
  #[inline]
  pub fn new<P>(path: &P) -> &Self
  where
    P: AsRef<str> + ?Sized,
  {
    // SAFETY: path is a str and therefore always valid UTF-8
    unsafe { Self::convert_str(path.as_ref()) }
  }

  /// Create a new `PtPath` from a stdlib [`Path`].
  #[inline]
  pub fn from_path(path: &Path) -> Option<&Self> {
    path.as_os_str().to_str().map(Self::new)
  }

  /// Returns a reference to the underlying [`OsStr`] slice.
  #[inline]
  pub fn as_os_str(&self) -> &OsStr {
    self.0.as_os_str()
  }

  /// Returns a reference to the underlying stdlib [`Path`].
  #[inline]
  pub fn as_std(&self) -> &Path {
    &self.0
  }

  /// Returns the `PtPath` as a string slice.
  #[inline]
  pub fn as_str(&self) -> &str {
    // SAFETY: PtPath is always valid UTF-8
    unsafe { Self::convert_os(self.as_os_str()) }
  }

  /// Converts the `PtPath` to an owned [`PtPathBuf`].
  #[inline]
  pub fn to_path_buf(&self) -> PtPathBuf {
    PtPathBuf(self.0.to_path_buf())
  }

  /// Returns `true` if the path points at an existing entity.
  #[inline]
  pub fn exists(&self) -> bool {
    self.0.exists()
  }

  /// Returns `true` if the `PtPath` is absolute.
  #[inline]
  pub fn is_absolute(&self) -> bool {
    self.0.is_absolute()
  }

  /// Returns `true` if the `PtPath` is relative.
  #[inline]
  pub fn is_relative(&self) -> bool {
    self.0.is_relative()
  }

  /// Returns `true` if the `PtPath` points to a symbolic link.
  #[inline]
  pub fn is_symlink(&self) -> bool {
    self.0.is_symlink()
  }

  /// Returns `true` if the `PtPath` points to a directory.
  #[inline]
  pub fn is_dir(&self) -> bool {
    self.0.is_dir()
  }

  /// Returns `true` if the `PtPath` points to a regular file.
  #[inline]
  pub fn is_file(&self) -> bool {
    self.0.is_file()
  }

  /// Returns the final component of the `PtPath`, if there is one.
  #[inline]
  pub fn file_name(&self) -> Option<&str> {
    self.0.file_name().map(|name| {
      // SAFETY: PtPath is always valid UTF-8
      unsafe { Self::convert_os(name) }
    })
  }

  /// Returns the `PtPath` without its final component, if there is one.
  #[inline]
  pub fn parent(&self) -> Option<&PtPath> {
    self.0.parent().map(|path| {
      // SAFETY: PtPath is always valid UTF-8
      unsafe { Self::convert_path(path) }
    })
  }

  pub(crate) fn root_name(&self) -> Option<&PtPath> {
    let mut components: Components<'_> = self.components();

    let Component::Normal(_) = components.next_back()? else {
      return None;
    };

    // SAFETY: PtPath is always valid UTF-8
    Some(unsafe { Self::convert_path(components.as_path()) })
  }

  pub(crate) fn split(&self) -> Option<(&str, &PtPath)> {
    let mut components: Components<'_> = self.components();

    let Component::Normal(name) = components.next_back()? else {
      return None;
    };

    // SAFETY: PtPath is always valid UTF-8
    let root: &PtPath = unsafe { Self::convert_path(components.as_path()) };
    let name: &str = name.to_str()?;

    Some((name, root))
  }

  /// Creates an owned [`PtPathBuf`] with `path` adjoined to `self`.
  #[inline]
  pub fn join<P>(&self, path: P) -> PtPathBuf
  where
    P: AsRef<Self>,
  {
    PtPathBuf(self.0.join(&path.as_ref().0))
  }

  #[inline]
  pub(crate) fn join_ext<P>(&self, path: P, extension: &str) -> PtPathBuf
  where
    P: AsRef<Self>,
  {
    PtPathBuf(self.join(path).0.with_extension(extension))
  }

  #[inline]
  fn components(&self) -> Components<'_> {
    self.0.components()
  }

  #[inline]
  const unsafe fn convert_str(input: &str) -> &Self {
    &*(input as *const str as *const Path as *const Self)
  }

  #[inline]
  const unsafe fn convert_path(input: &Path) -> &Self {
    &*(input as *const Path as *const Self)
  }

  #[inline]
  const unsafe fn convert_os(input: &OsStr) -> &str {
    &*(input as *const OsStr as *const str)
  }
}

// =============================================================================
// ProTools PathBuf
// =============================================================================

/// An owned UTF-8 path.
#[derive(Clone, Default)]
#[repr(transparent)]
pub struct PtPathBuf(PathBuf);

impl PtPathBuf {
  /// Create a new `PtPathBuf`.
  #[inline]
  pub fn new() -> Self {
    Self(PathBuf::new())
  }

  /// Create a new `PtPath` from a stdlib [`PathBuf`].
  #[inline]
  pub fn from_path_buf(path: PathBuf) -> Result<Self, PathBuf> {
    path
      .into_os_string()
      .into_string()
      .map(Into::into)
      .map_err(Into::into)
  }

  /// Coerces to a `PtPath` slice.
  #[inline]
  pub fn as_path(&self) -> &PtPath {
    // SAFETY: PtPathBuf is always valid UTF-8
    unsafe { PtPath::convert_path(&self.0) }
  }

  /// Returns the path as a [`String`], consuming the `PtPathBuf`.
  #[inline]
  pub fn into_string(self) -> String {
    self.into_os_string().into_string().unwrap()
  }

  /// Returns the internal [`OsString`], consuming the `PtPathBuf`.
  #[inline]
  pub fn into_os_string(self) -> OsString {
    self.0.into_os_string()
  }
}

impl Deref for PtPathBuf {
  type Target = PtPath;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.as_path()
  }
}

// =============================================================================
// ProTools Directories
// =============================================================================

/// ProTools-specific path extension.
#[derive(Clone)]
#[repr(transparent)]
pub struct PtDirs(PtPathBuf);

impl PtDirs {
  const EXT: &'static str = "ptx";

  /// Create a new `PtDirs` from a `name` and default path.
  #[inline]
  pub fn new<T>(name: &T) -> Self
  where
    T: AsRef<str> + ?Sized,
  {
    Self::with_path(name, &Self::default_session_root())
  }

  /// Create a new `PtDirs` from explicit `name` and `path`.
  pub fn with_path<T, U>(name: &T, path: &U) -> Self
  where
    T: AsRef<str> + ?Sized,
    U: AsRef<PtPath> + ?Sized,
  {
    Self(path.as_ref().join(name.as_ref()))
  }

  /// Returns the Pro Tools session name.
  #[inline]
  pub fn name(&self) -> &str {
    self.file_name().expect("Invalid PtPath Components")
  }

  /// Returns the Pro Tools session directory.
  #[inline]
  pub fn root(&self) -> &PtPath {
    self.root_name().expect("Invalid PtPath Components")
  }

  /// Split the session path into (`name`, `directory`) parts.
  #[inline]
  pub fn split(&self) -> (&str, &PtPath) {
    self.0.split().expect("Invalid PtPath Components")
  }

  /// Returns a path pointing the a session file with the given `name`.
  #[inline]
  pub fn session<T>(&self, name: T) -> PtPathBuf
  where
    T: AsRef<PtPath>,
  {
    self.join_ext(name, Self::EXT)
  }

  /// Returns the default session file.
  #[inline]
  pub fn default_session(&self) -> PtPathBuf {
    self.session(self.name())
  }

  /// Returns the session `Audio Files` directory.
  #[inline]
  pub fn audio(&self) -> PtPathBuf {
    self.join("Audio Files")
  }

  /// Returns the session `Video Files` directory.
  #[inline]
  pub fn video(&self) -> PtPathBuf {
    self.join("Video Files")
  }

  /// Returns the session `Bounced Files` directory.
  #[inline]
  pub fn bounced(&self) -> PtPathBuf {
    self.join("Bounced Files")
  }

  /// Returns the session `Clip Groups` directory.
  #[inline]
  pub fn clips(&self) -> PtPathBuf {
    self.join("Clip Groups")
  }

  /// Returns the session `Session File Backups` directory.
  #[inline]
  pub fn backups(&self) -> PtPathBuf {
    self.join("Session File Backups")
  }

  /// Returns the session `WaveCache` file.
  #[inline]
  pub fn cache(&self) -> PtPathBuf {
    self.join("WaveCache.wfm")
  }

  fn default_session_root() -> PtPathBuf {
    if cfg!(target_os = "macos") {
      if let Some(Ok(home)) = var_os("HOME").map(PtPathBuf::try_from) {
        home.join("Documents/Pro Tools")
      } else {
        PtPathBuf::from("/")
      }
    } else {
      unimplemented!("PtDirs::default_session_root")
    }
  }
}

impl Deref for PtDirs {
  type Target = PtPathBuf;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

// =============================================================================
// Errors
// =============================================================================

/// Possible error caused by converting a [`Path`] to [`PtPath`].
#[derive(Clone, PartialEq, Eq)]
pub struct FromPathError<'a>(&'a OsStr);

impl Debug for FromPathError<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self, f)
  }
}

impl Display for FromPathError<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(
      f,
      "Path contains invalid UTF-8: {}",
      self.0.to_string_lossy()
    )
  }
}

impl Error for FromPathError<'_> {}

/// Possible error caused by converting a [`PathBuf`] to [`PtPathBuf`].
#[derive(Clone, PartialEq, Eq)]
pub struct FromPathBufError(PathBuf);

impl Debug for FromPathBufError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self, f)
  }
}

impl Display for FromPathBufError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "PathBuf contains invalid UTF-8: {}", self.0.display())
  }
}

impl Error for FromPathBufError {}

// =============================================================================
// Traits (PtPath core)
// =============================================================================

impl Debug for PtPath {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for PtPath {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_str(), f)
  }
}

impl Hash for PtPath {
  #[inline]
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    for component in self.components() {
      component.hash(hasher);
    }
  }
}

impl PartialEq for PtPath {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.components().eq(other.components())
  }
}

impl Eq for PtPath {}

impl PartialOrd for PtPath {
  #[inline]
  fn partial_cmp(&self, other: &PtPath) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PtPath {
  #[inline]
  fn cmp(&self, other: &PtPath) -> Ordering {
    self.components().cmp(other.components())
  }
}

// =============================================================================
// Traits (PtPathBuf core)
// =============================================================================

impl Debug for PtPathBuf {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for PtPathBuf {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_str(), f)
  }
}

impl Hash for PtPathBuf {
  #[inline]
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.as_path().hash(hasher);
  }
}

impl PartialEq for PtPathBuf {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.as_path().eq(other.as_path())
  }
}

impl Eq for PtPathBuf {}

impl PartialOrd for PtPathBuf {
  #[inline]
  fn partial_cmp(&self, other: &PtPathBuf) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PtPathBuf {
  #[inline]
  fn cmp(&self, other: &PtPathBuf) -> Ordering {
    self.as_path().cmp(other.as_path())
  }
}

// =============================================================================
// Traits (PtDirs core)
// =============================================================================

impl Debug for PtDirs {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for PtDirs {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(self.as_str(), f)
  }
}

impl Hash for PtDirs {
  #[inline]
  fn hash<H: Hasher>(&self, hasher: &mut H) {
    self.as_path().hash(hasher);
  }
}

impl PartialEq for PtDirs {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.as_path().eq(other.as_path())
  }
}

impl Eq for PtDirs {}

impl PartialOrd for PtDirs {
  #[inline]
  fn partial_cmp(&self, other: &PtDirs) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for PtDirs {
  #[inline]
  fn cmp(&self, other: &PtDirs) -> Ordering {
    self.as_path().cmp(other.as_path())
  }
}

// =============================================================================
// Traits (Borrow/ToOwned)
// =============================================================================

impl Borrow<PtPath> for PtPathBuf {
  #[inline]
  fn borrow(&self) -> &PtPath {
    self.as_path()
  }
}

impl ToOwned for PtPath {
  type Owned = PtPathBuf;

  #[inline]
  fn to_owned(&self) -> PtPathBuf {
    self.to_path_buf()
  }
}

// =============================================================================
// Traits (From borrowed -> borrowed)
// =============================================================================

impl<'a> From<&'a str> for &'a PtPath {
  #[inline]
  fn from(other: &'a str) -> Self {
    PtPath::new(other)
  }
}

// =============================================================================
// Traits (From borrowed -> owned)
// =============================================================================

impl<T> From<&T> for PtPathBuf
where
  T: AsRef<str> + ?Sized,
{
  #[inline]
  fn from(other: &T) -> Self {
    other.as_ref().to_owned().into()
  }
}

impl<'a> From<&'a PtPath> for Cow<'a, PtPath> {
  #[inline]
  fn from(other: &'a PtPath) -> Self {
    Cow::Borrowed(other)
  }
}

// =============================================================================
// Traits (From owned -> owned)
// =============================================================================

impl From<String> for PtPathBuf {
  #[inline]
  fn from(other: String) -> Self {
    Self(other.into())
  }
}

impl From<PtPathBuf> for String {
  #[inline]
  fn from(other: PtPathBuf) -> Self {
    other.into_string()
  }
}

impl From<PtPathBuf> for OsString {
  #[inline]
  fn from(other: PtPathBuf) -> Self {
    other.into_os_string()
  }
}

impl<'a> From<PtPathBuf> for Cow<'a, PtPath> {
  #[inline]
  fn from(other: PtPathBuf) -> Self {
    Cow::Owned(other)
  }
}

// =============================================================================
// Traits (TryFrom)
// =============================================================================

impl<'a> TryFrom<&'a Path> for &'a PtPath {
  type Error = FromPathError<'a>;

  #[inline]
  fn try_from(other: &'a Path) -> Result<Self, Self::Error> {
    PtPath::from_path(other).ok_or_else(|| FromPathError(other.as_os_str()))
  }
}

impl<'a> TryFrom<&'a OsStr> for &'a PtPath {
  type Error = FromPathError<'a>;

  #[inline]
  fn try_from(other: &'a OsStr) -> Result<Self, Self::Error> {
    PtPath::from_path(Path::new(other)).ok_or_else(|| FromPathError(other))
  }
}

impl TryFrom<PathBuf> for PtPathBuf {
  type Error = FromPathBufError;

  #[inline]
  fn try_from(other: PathBuf) -> Result<Self, Self::Error> {
    PtPathBuf::from_path_buf(other).map_err(FromPathBufError)
  }
}

impl TryFrom<OsString> for PtPathBuf {
  type Error = FromPathBufError;

  #[inline]
  fn try_from(other: OsString) -> Result<Self, Self::Error> {
    PtPathBuf::from_path_buf(other.into()).map_err(FromPathBufError)
  }
}

// =============================================================================
// Traits (AsRef PtPath)
// =============================================================================

impl AsRef<PtPath> for PtPath {
  #[inline]
  fn as_ref(&self) -> &PtPath {
    self
  }
}

impl AsRef<PtPath> for PtPathBuf {
  #[inline]
  fn as_ref(&self) -> &PtPath {
    self.as_path()
  }
}

impl AsRef<PtPath> for PtDirs {
  #[inline]
  fn as_ref(&self) -> &PtPath {
    self.as_path()
  }
}

impl AsRef<PtPath> for str {
  #[inline]
  fn as_ref(&self) -> &PtPath {
    PtPath::new(self)
  }
}

impl AsRef<PtPath> for String {
  #[inline]
  fn as_ref(&self) -> &PtPath {
    PtPath::new(self)
  }
}

// =============================================================================
// Traits (AsRef Path)
// =============================================================================

impl AsRef<Path> for PtPath {
  #[inline]
  fn as_ref(&self) -> &Path {
    self.as_std()
  }
}

impl AsRef<Path> for PtPathBuf {
  #[inline]
  fn as_ref(&self) -> &Path {
    self.as_path().as_ref()
  }
}

impl AsRef<Path> for PtDirs {
  #[inline]
  fn as_ref(&self) -> &Path {
    self.as_path().as_ref()
  }
}

// =============================================================================
// Traits (AsRef str)
// =============================================================================

impl AsRef<str> for PtPath {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl AsRef<str> for PtPathBuf {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl AsRef<str> for PtDirs {
  #[inline]
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

// =============================================================================
// Traits (AsRef OsStr)
// =============================================================================

impl AsRef<OsStr> for PtPath {
  #[inline]
  fn as_ref(&self) -> &OsStr {
    self.as_os_str()
  }
}

impl AsRef<OsStr> for PtPathBuf {
  #[inline]
  fn as_ref(&self) -> &OsStr {
    self.as_os_str()
  }
}

impl AsRef<OsStr> for PtDirs {
  #[inline]
  fn as_ref(&self) -> &OsStr {
    self.as_os_str()
  }
}

// =============================================================================
// Comparisons
// =============================================================================

macro_rules! impl_cmp {
  (@core, $lhs:ty, $rhs:ty) => {
    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialEq<$rhs> for $lhs {
      #[inline]
      fn eq(&self, other: &$rhs) -> bool {
        <PtPath as PartialEq>::eq(self, other)
      }
    }

    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialEq<$lhs> for $rhs {
      #[inline]
      fn eq(&self, other: &$lhs) -> bool {
        <PtPath as PartialEq>::eq(self, other)
      }
    }

    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialOrd<$rhs> for $lhs {
      #[inline]
      fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
        <PtPath as PartialOrd>::partial_cmp(self, other)
      }
    }

    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialOrd<$lhs> for $rhs {
      #[inline]
      fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
        <PtPath as PartialOrd>::partial_cmp(self, other)
      }
    }
  };

  (@str, $lhs:ty, $rhs:ty) => {
    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialEq<$rhs> for $lhs {
      #[inline]
      fn eq(&self, other: &$rhs) -> bool {
        <PtPath as PartialEq>::eq(self, PtPath::new(other))
      }
    }

    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialEq<$lhs> for $rhs {
      #[inline]
      fn eq(&self, other: &$lhs) -> bool {
        <PtPath as PartialEq>::eq(PtPath::new(self), other)
      }
    }

    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialOrd<$rhs> for $lhs {
      #[inline]
      fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
        <PtPath as PartialOrd>::partial_cmp(self, PtPath::new(other))
      }
    }

    #[allow(clippy::extra_unused_lifetimes)]
    impl<'a, 'b> PartialOrd<$lhs> for $rhs {
      #[inline]
      fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
        <PtPath as PartialOrd>::partial_cmp(PtPath::new(self), other)
      }
    }
  };
}

impl_cmp!(@core, PtPathBuf, PtPath);
impl_cmp!(@core, PtPathBuf, &'a PtPath);
impl_cmp!(@core, Cow<'a, PtPath>, PtPath);
impl_cmp!(@core, Cow<'a, PtPath>, &'a PtPath);
impl_cmp!(@core, Cow<'a, PtPath>, PtPathBuf);

impl_cmp!(@str, PtPathBuf, str);
impl_cmp!(@str, PtPathBuf, &'a str);
impl_cmp!(@str, PtPathBuf, Cow<'a, str>);
impl_cmp!(@str, PtPathBuf, String);
impl_cmp!(@str, PtPath, str);
impl_cmp!(@str, PtPath, &'a str);
impl_cmp!(@str, PtPath, Cow<'a, str>);
impl_cmp!(@str, PtPath, String);
impl_cmp!(@str, &'a PtPath, str);
impl_cmp!(@str, &'a PtPath, Cow<'a, str>);
impl_cmp!(@str, &'a PtPath, String);
