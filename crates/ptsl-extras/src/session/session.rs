use ptsl_client::client::Client;
use ptsl_client::error::Result;
use ptsl_protos::bridge::CommandExt;
use ptsl_protos::types::AutomationDataOptions;
use ptsl_protos::types::EditModeOptions;
use ptsl_protos::types::EsiOutputType;
use ptsl_protos::types::ExportSessionInfoAsTextRequestBody;
use ptsl_protos::types::FadeHandlingType;
use ptsl_protos::types::PasteSpecialOptions;
use ptsl_protos::types::SampleRate;
use ptsl_protos::types::TextAsFileFormat;
use ptsl_protos::types::TrackListType;
use ptsl_protos::types::TrackOffsetOptions;

use crate::path::PtDirs;
use crate::path::PtPath;
use crate::property::AudioFormat;
use crate::property::AudioRatePull;
use crate::property::BitDepth;
use crate::property::Container;
use crate::property::EditMode;
use crate::property::EditTool;
use crate::property::FeetFramesRate;
use crate::property::Interleaved;
use crate::property::Length;
use crate::property::PlaybackMode;
use crate::property::PropertyGet;
use crate::property::PropertySet;
use crate::property::RecordMode;
use crate::property::StartTime;
use crate::property::TimeCodeRate;
use crate::property::TransportState;
use crate::property::VideoRatePull;
use crate::session::EditModeOptionsBuilder;
use crate::session::SessionPath;
use crate::session::Status;
use crate::session::TimelineSelection;
use crate::utils::try_from_proto;

/// A high-level wrapper for Pro Tools sessions.
#[derive(Debug)]
pub struct Session {
  client: Client,
  status: Status,
  ptdirs: PtDirs,
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

  /// Returns the session directory.
  pub const fn ptdirs(&self) -> &PtDirs {
    &self.ptdirs
  }

  // ===========================================================================
  // PTSL Core
  // ===========================================================================

  /// Returns the current version of the PTSL host.
  pub async fn version(&mut self) -> Result<i32> {
    self
      .client
      .get_ptsl_version()
      .await
      .map(|recv| recv.version)
  }

  // ===========================================================================
  // Session File
  // ===========================================================================

  /// Open the session at the given `path`.
  pub async fn open<P>(path: &P) -> Result<Self>
  where
    P: AsRef<PtPath> + ?Sized,
  {
    let mut client: Client = Client::new().await?;
    let ptdirs: PtDirs = PtDirs::new(path.as_ref());

    client
      .register_connection(Self::APPNAME.into(), Self::COMPANY.into())
      .await?;

    client.open_session(ptdirs.to_string()).await?;

    Ok(Self {
      client,
      ptdirs,
      status: Status::Active,
    })
  }

  /// Open a previously closed Pro Tools session.
  pub async fn reopen(&mut self) -> Result<()> {
    self.status.assert_closed();

    self.client.open_session(self.ptdirs.to_string()).await?;
    self.status = Status::Active;

    Ok(())
  }

  /// Close the Pro Tools session.
  pub async fn close(&mut self, save: bool) -> Result<()> {
    self.status.assert_active();
    self.client.close_session(save).await?;
    self.status = Status::Closed;

    Ok(())
  }

  /// Send a `SaveSession` command to the PTSL server.
  #[inline]
  pub async fn save(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.save_session().await
  }

  /// Send a `SaveSessionAs` command to the PTSL server.
  pub async fn save_as<P>(&mut self, path: &P) -> Result<()>
  where
    P: AsRef<PtPath> + ?Sized,
  {
    self.status.assert_active();

    let dirs: PtDirs = PtDirs::new(path.as_ref());
    let name: String = dirs.name().to_owned();
    let path: String = dirs.root().to_string();

    self.client.save_session_as(name, path).await
  }

  /// Export session info as a string.
  pub async fn export_text(&mut self) -> Result<String> {
    self.status.assert_active();

    export_base(EsiOutputType::EsiString, None)
      .send(&mut self.client)
      .await
      .map(|recv| recv.session_info)
  }

  /// Export session info to a file at the given `path`.
  pub async fn export_file<P>(&mut self, path: &P) -> Result<()>
  where
    P: AsRef<PtPath> + ?Sized,
  {
    self.status.assert_active();

    export_base(EsiOutputType::EsiFile, Some(path.as_ref().to_string()))
      .send(&mut self.client)
      .await
      .map(|_| ())
  }

  // ===========================================================================
  // Session Properties (Static)
  // ===========================================================================

  /// Returns the session display name.
  pub async fn name(&mut self) -> Result<String> {
    self.status.assert_active();

    self
      .client
      .get_session_name()
      .await
      .map(|recv| recv.session_name)
  }

  /// Returns the session file path.
  pub async fn path(&mut self) -> Result<SessionPath> {
    self.status.assert_active();

    self
      .client
      .get_session_path()
      .await
      .map(|recv| recv.session_path.expect("Session Path"))
      .map(SessionPath::new)
  }

  /// Returns the session sample rate.
  pub async fn sample_rate(&mut self) -> Result<SampleRate> {
    self.status.assert_active();

    self
      .client
      .get_session_sample_rate()
      .await
      .and_then(|recv| try_from_proto(recv.sample_rate))
  }

  /// Returns `true` if the session transport is armed.
  pub async fn transport_armed(&mut self) -> Result<bool> {
    self.status.assert_active();

    self
      .client
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

  /// Returns the edit mode session property.
  #[inline]
  pub async fn edit_mode(&mut self) -> Result<Container<EditMode>> {
    self.get_property().await
  }

  /// Set the current value of the edit mode session property.
  #[inline]
  pub async fn set_edit_mode(&mut self, value: EditMode) -> Result<()> {
    self.set_property(value).await
  }

  /// Returns the edit tool session property.
  #[inline]
  pub async fn edit_tool(&mut self) -> Result<Container<EditTool>> {
    self.get_property().await
  }

  /// Set the current value of the edit tool session property.
  #[inline]
  pub async fn set_edit_tool(&mut self, value: EditTool) -> Result<()> {
    self.set_property(value).await
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

  /// Returns the session transport state property.
  #[inline]
  pub async fn transport_state(&mut self) -> Result<Container<TransportState>> {
    self.get_property().await
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

  // ===========================================================================
  // Session Properties (Extra)
  // ===========================================================================

  /// Returns the session edit mode options.
  #[inline]
  pub async fn edit_mode_options(&mut self) -> Result<Option<EditModeOptions>> {
    self
      .client
      .get_edit_mode_options()
      .await
      .map(|recv| recv.edit_mode_options)
  }

  /// Set the session edit mode options.
  #[inline]
  pub async fn set_edit_mode_options(&mut self) -> EditModeOptionsBuilder<'_> {
    EditModeOptionsBuilder::new(&mut self.client)
  }

  /// Returns the session timeline selection.
  #[inline]
  pub async fn timeline_selection(
    &mut self,
    time_scale: TrackOffsetOptions,
  ) -> Result<TimelineSelection> {
    self
      .client
      .get_timeline_selection(time_scale)
      .await
      .map(TimelineSelection::from_response)
  }

  /// Set the session timeline selection.
  #[inline]
  pub async fn set_timeline_selection(&mut self, value: TimelineSelection) -> Result<()> {
    value.into_request().send(&mut self.client).await
  }

  // ===========================================================================
  // Basic Edit Commands
  // ===========================================================================

  /// Send a `ConsolidateClip` command to the PTSL server.
  #[inline]
  pub async fn consolidate(&mut self) -> Result<()> {
    self.client.consolidate_clip().await
  }

  /// Send a `Clear` command to the PTSL server.
  #[inline]
  pub async fn clear(&mut self) -> Result<()> {
    self.client.clear().await
  }

  /// Send a `ClearSpecial` command to the PTSL server.
  #[inline]
  pub async fn clear_special(&mut self, options: AutomationDataOptions) -> Result<()> {
    self.client.clear_special(options).await
  }

  /// Send a `Copy` command to the PTSL server.
  #[inline]
  pub async fn copy(&mut self) -> Result<()> {
    self.client.copy().await
  }

  /// Send a `CopySpecial` command to the PTSL server.
  #[inline]
  pub async fn copy_special(&mut self, options: AutomationDataOptions) -> Result<()> {
    self.client.copy_special(options).await
  }

  /// Send a `Cut` command to the PTSL server.
  #[inline]
  pub async fn cut(&mut self) -> Result<()> {
    self.client.cut().await
  }

  /// Send a `CutSpecial` command to the PTSL server.
  #[inline]
  pub async fn cut_special(&mut self, options: AutomationDataOptions) -> Result<()> {
    self.client.cut_special(options).await
  }

  /// Send a `Paste` command to the PTSL server.
  #[inline]
  pub async fn paste(&mut self) -> Result<()> {
    self.client.paste().await
  }

  /// Send a `PasteSpecial` command to the PTSL server.
  #[inline]
  pub async fn paste_special(&mut self, options: PasteSpecialOptions) -> Result<()> {
    self.client.paste_special(options).await
  }

  /// Send a `TrimToSelection` command to the PTSL server.
  #[inline]
  pub async fn trim(&mut self) -> Result<()> {
    self.client.trim_to_selection().await
  }

  /// Send a `RefreshAllModifiedAudioFiles` command to the PTSL server.
  #[inline]
  pub async fn refresh_all(&mut self) -> Result<()> {
    self.client.refresh_all_modified_audio_files().await
  }

  // ===========================================================================
  // Basic Playback Commands
  // ===========================================================================

  /// Set a `PlayHalfSpeed` command to the PTSL server.
  #[inline]
  pub async fn play_half_speed(&mut self) -> Result<()> {
    self.client.play_half_speed().await
  }

  /// Set a `RecordHalfSpeed` command to the PTSL server.
  #[inline]
  pub async fn record_half_speed(&mut self) -> Result<()> {
    self.client.record_half_speed().await
  }

  /// Set a `TogglePlayState` command to the PTSL server.
  #[inline]
  pub async fn toggle_play_state(&mut self) -> Result<()> {
    self.client.toggle_play_state().await
  }

  /// Set a `ToggleRecordEnable` command to the PTSL server.
  #[inline]
  pub async fn toggle_record_enable(&mut self) -> Result<()> {
    self.client.toggle_record_enable().await
  }
}

fn export_base(kind: EsiOutputType, path: Option<String>) -> ExportSessionInfoAsTextRequestBody {
  ExportSessionInfoAsTextRequestBody {
    include_file_list: true,
    include_clip_list: true,
    include_markers: true,
    include_plugin_list: true,
    include_track_edls: true,
    show_sub_frames: true,
    include_user_timestamps: true,
    track_list_type: TrackListType::AllTracks.into(),
    fade_handling_type: FadeHandlingType::ShowCrossfades.into(),
    track_offset_options: TrackOffsetOptions::MinSecs.into(),
    text_as_file_format: TextAsFileFormat::Utf8.into(),
    output_type: kind.into(),
    output_path: path.unwrap_or_default(),
  }
}
