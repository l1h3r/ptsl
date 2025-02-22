use ptsl_client::client::Client;
use ptsl_client::error::Result;
use ptsl_protos::bridge::CommandExt;
use ptsl_protos::error::Error;
use ptsl_protos::types::FileLocation;
use ptsl_protos::types::SampleRate;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;

use crate::property::AudioFormat;
use crate::property::AudioRatePull;
use crate::property::BitDepth;
use crate::property::Container;
use crate::property::FeetFramesRate;
use crate::property::Interleaved;
use crate::property::Length;
use crate::property::PlaybackMode;
use crate::property::PropertyGet;
use crate::property::PropertySet;
use crate::property::RecordMode;
use crate::property::StartTime;
use crate::property::TimeCodeRate;
use crate::property::VideoRatePull;
use crate::session::Status;

/// A high-level wrapper for Pro Tools sessions.
#[derive(Debug)]
pub struct Session {
  client: Client,
  status: Status,
}

impl Session {
  /// Default application name for client registration.
  pub const APPNAME: &'static str = "PTSL";

  /// Default company name for client registration.
  pub const COMPANY: &'static str = "PTSL";

  /// Returns a reference to the gRPC client.
  pub const fn client(&self) -> &Client {
    &self.client
  }

  /// Returns a mutable reference to the gRPC client.
  pub fn client_mut(&mut self) -> &mut Client {
    &mut self.client
  }

  /// Returns the current session status.
  pub const fn status(&self) -> Status {
    self.status
  }

  // ===========================================================================
  // PTSL Core
  // ===========================================================================

  /// Returns the current version of the PTSL host.
  pub async fn version(&mut self) -> Result<i32> {
    self.get_ptsl_version().await.map(|recv| recv.version)
  }

  // ===========================================================================
  // Session Properties (Static)
  // ===========================================================================

  /// Returns the session display name.
  pub async fn name(&mut self) -> Result<String> {
    self.status.assert_active();
    self.get_session_name().await.map(|recv| recv.session_name)
  }

  /// Returns the session file path.
  pub async fn path(&mut self) -> Result<SessionPath> {
    self.status.assert_active();

    self
      .get_session_path()
      .await
      .map(|recv| recv.session_path.expect("Session Path"))
      .map(SessionPath::new)
  }

  /// Returns the session sample rate.
  pub async fn sample_rate(&mut self) -> Result<SampleRate> {
    self.status.assert_active();

    self
      .get_session_sample_rate()
      .await
      .and_then(|recv| try_from_proto(recv.sample_rate))
  }

  /// Returns `true` if the session transport is armed.
  pub async fn transport_armed(&mut self) -> Result<bool> {
    self.status.assert_active();

    self
      .get_transport_armed()
      .await
      .map(|recv| recv.is_transport_armed)
  }

  // ===========================================================================
  // Session Properties (Dynamic)
  // ===========================================================================

  async fn get_property<T: PropertyGet>(&mut self) -> Result<Container<T>> {
    self.status.assert_active();
    T::get(&mut self.client).await
  }

  async fn set_property<T: PropertySet>(&mut self, value: T) -> Result<()> {
    self.status.assert_active();
    T::set(&mut self.client, value).await
  }

  /// Returns the playback mode session property.
  #[inline]
  pub async fn playback_mode(&mut self) -> Result<Container<PlaybackMode>> {
    self.get_property().await
  }

  /// Set the current value of the playback mode session property.
  #[inline]
  pub async fn set_playback_mode(&mut self, value: PlaybackMode) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the record mode session property.
  #[inline]
  pub async fn record_mode(&mut self) -> Result<Container<RecordMode>> {
    self.get_property().await
  }

  /// Set the current value of the record mode session property.
  #[inline]
  pub async fn set_record_mode(&mut self, value: RecordMode, armed: bool) -> Result<()> {
    if armed {
      self.set_property(value.armed()).await
    } else {
      self.set_property(value).await
    }
  }

  /// Returns the audio format session property.
  #[inline]
  pub async fn audio_format(&mut self) -> Result<Container<AudioFormat>> {
    self.get_property().await
  }

  /// Set the current value of the audio format session property.
  #[inline]
  pub async fn set_audio_format(&mut self, value: AudioFormat) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the audio rate pull session property.
  #[inline]
  pub async fn audio_rate_pull(&mut self) -> Result<Container<AudioRatePull>> {
    self.get_property().await
  }

  /// Set the current value of the audio rate pull session property.
  #[inline]
  pub async fn set_audio_rate_pull(&mut self, value: AudioRatePull) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the bit depth session property.
  #[inline]
  pub async fn bit_depth(&mut self) -> Result<Container<BitDepth>> {
    self.get_property().await
  }

  /// Set the current value of the bit depth session property.
  #[inline]
  pub async fn set_bit_depth(&mut self, value: BitDepth) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the feet frames rate session property.
  #[inline]
  pub async fn feet_frames_rate(&mut self) -> Result<Container<FeetFramesRate>> {
    self.get_property().await
  }

  /// Set the current value of the feet frames rate session property.
  #[inline]
  pub async fn set_feet_frames_rate(&mut self, value: FeetFramesRate) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the interleaved state session property.
  #[inline]
  pub async fn interleaved_state(&mut self) -> Result<Container<Interleaved>> {
    self.get_property().await
  }

  /// Set the current value of the interleaved state session property.
  #[inline]
  pub async fn set_interleaved_state(&mut self, value: impl Into<Interleaved>) -> Result<()> {
    self.set_property(value.into()).await
  }

  /// Returns the session length property.
  #[inline]
  pub async fn length(&mut self) -> Result<Length> {
    self.get_property().await.map(Container::into_inner)
  }

  /// Set the current value of the session length property.
  #[inline]
  pub async fn set_length(&mut self, value: Length) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the session start time property.
  #[inline]
  pub async fn start_time(&mut self) -> Result<StartTime> {
    self.get_property().await.map(Container::into_inner)
  }

  /// Set the current value of the session start time property.
  #[inline]
  pub async fn set_start_time(&mut self, value: StartTime) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the time code rate session property.
  #[inline]
  pub async fn time_code_rate(&mut self) -> Result<Container<TimeCodeRate>> {
    self.get_property().await
  }

  /// Set the current value of the time code rate session property.
  #[inline]
  pub async fn set_time_code_rate(&mut self, value: TimeCodeRate) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the video rate pull session property.
  #[inline]
  pub async fn video_rate_pull(&mut self) -> Result<Container<VideoRatePull>> {
    self.get_property().await
  }

  /// Set the current value of the video rate pull session property.
  #[inline]
  pub async fn set_video_rate_pull(&mut self, value: VideoRatePull) -> Result<()> {
    self.set_property(value).await
  }
}

impl Deref for Session {
  type Target = Client;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.client()
  }
}

impl DerefMut for Session {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.client_mut()
  }
}

fn try_from_proto<T, U>(value: T) -> Result<U>
where
  U: TryFrom<T>,
  U::Error: Into<Error>,
{
  U::try_from(value).map_err(Into::into).map_err(Into::into)
}

// =============================================================================
// Session Path
// =============================================================================

/// Session file location.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionPath {
  pub(crate) online: bool,
  pub(crate) ospath: PathBuf,
}

impl SessionPath {
  #[inline]
  fn new(path: FileLocation) -> Self {
    Self {
      online: path.info.map_or(false, |info| info.is_online),
      ospath: path.path.into(),
    }
  }

  /// Returns a reference to the OS path.
  #[inline]
  pub fn ospath(&self) -> &Path {
    &self.ospath
  }

  /// Returns `true` if the session is stored online.
  #[inline]
  pub const fn online(&self) -> bool {
    self.online
  }
}
