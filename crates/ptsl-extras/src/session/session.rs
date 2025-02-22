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
use crate::session::RefreshList;
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
  #[inline]
  pub const fn client(&self) -> &Client {
    &self.client
  }

  /// Returns a mutable reference to the gRPC client.
  #[inline]
  pub fn client_mut(&mut self) -> &mut Client {
    &mut self.client
  }

  /// Returns the current session status.
  #[inline]
  pub const fn status(&self) -> Status {
    self.status
  }

  /// Returns the session directory.
  #[inline]
  pub const fn ptdirs(&self) -> &PtDirs {
    &self.ptdirs
  }

  // ===========================================================================
  // PTSL Core Commands
  // ===========================================================================

  /// Returns the current version of the PTSL host.
  #[inline]
  pub async fn version(&mut self) -> Result<i32> {
    self
      .client
      .get_ptsl_version()
      .await
      .map(|recv| recv.version)
  }

  // ===========================================================================
  // Session File Commands
  // ===========================================================================

  /// Open the Pro Tools session at the given `path`.
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

  /// Save the Pro Tools session.
  pub async fn save(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.save_session().await
  }

  /// Save the Pro Tools session as a new session at the given `path`.
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

  /// Export session info as a file at the given `path`.
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

  /// Get the current session name.
  pub async fn name(&mut self) -> Result<String> {
    self.status.assert_active();

    self
      .client
      .get_session_name()
      .await
      .map(|recv| recv.session_name)
  }

  /// Get the current session path.
  pub async fn path(&mut self) -> Result<SessionPath> {
    self.status.assert_active();

    self
      .client
      .get_session_path()
      .await
      .map(|recv| recv.session_path.expect("Session Path"))
      .map(SessionPath::new)
  }

  /// Get the current session sample rate.
  pub async fn sample_rate(&mut self) -> Result<SampleRate> {
    self.status.assert_active();

    self
      .client
      .get_session_sample_rate()
      .await
      .and_then(|recv| try_from_proto(recv.sample_rate))
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

  /// Get the edit mode property.
  #[inline]
  pub async fn edit_mode(&mut self) -> Result<Container<EditMode>> {
    self.get_property().await
  }

  /// Set the edit mode property.
  #[inline]
  pub async fn set_edit_mode(&mut self, value: EditMode) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the edit mode options.
  #[inline]
  pub async fn edit_mode_options(&mut self) -> Result<Option<EditModeOptions>> {
    self.status.assert_active();

    self
      .client
      .get_edit_mode_options()
      .await
      .map(|recv| recv.edit_mode_options)
  }

  /// Set the edit mode options.
  #[inline]
  pub async fn set_edit_mode_options(&mut self) -> EditModeOptionsBuilder<'_> {
    self.status.assert_active();
    EditModeOptionsBuilder::new(&mut self.client)
  }

  /// Get the edit tool property.
  #[inline]
  pub async fn edit_tool(&mut self) -> Result<Container<EditTool>> {
    self.get_property().await
  }

  /// Set the edit tool property.
  #[inline]
  pub async fn set_edit_tool(&mut self, value: EditTool) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the audio file format property.
  #[inline]
  pub async fn audio_format(&mut self) -> Result<Container<AudioFormat>> {
    self.get_property().await
  }

  /// Set the audio file format property.
  #[inline]
  pub async fn set_audio_format(&mut self, value: AudioFormat) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the audio rate pull property.
  #[inline]
  pub async fn audio_rate_pull(&mut self) -> Result<Container<AudioRatePull>> {
    self.get_property().await
  }

  /// Set the audio rate pull property.
  #[inline]
  pub async fn set_audio_rate_pull(&mut self, value: AudioRatePull) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the bit depth property.
  #[inline]
  pub async fn bit_depth(&mut self) -> Result<Container<BitDepth>> {
    self.get_property().await
  }

  /// Set the bit depth property.
  #[inline]
  pub async fn set_bit_depth(&mut self, value: BitDepth) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the feet+frames rate property.
  #[inline]
  pub async fn feet_frames_rate(&mut self) -> Result<Container<FeetFramesRate>> {
    self.get_property().await
  }

  /// Set the feet+frames rate property.
  #[inline]
  pub async fn set_feet_frames_rate(&mut self, value: FeetFramesRate) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the interleaved state property.
  #[inline]
  pub async fn interleaved_state(&mut self) -> Result<Container<Interleaved>> {
    self.get_property().await
  }

  /// Set the interleaved state property.
  #[inline]
  pub async fn set_interleaved_state(&mut self, value: impl Into<Interleaved>) -> Result<()> {
    self.set_property(value.into()).await
  }

  /// Get the session length property.
  #[inline]
  pub async fn length(&mut self) -> Result<Length> {
    self.get_property().await.map(Container::into_inner)
  }

  /// Set the session length property.
  #[inline]
  pub async fn set_length(&mut self, value: Length) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the start time property.
  #[inline]
  pub async fn start_time(&mut self) -> Result<StartTime> {
    self.get_property().await.map(Container::into_inner)
  }

  /// Set the start time property.
  #[inline]
  pub async fn set_start_time(&mut self, value: StartTime) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the time code rate property.
  #[inline]
  pub async fn time_code_rate(&mut self) -> Result<Container<TimeCodeRate>> {
    self.get_property().await
  }

  /// Set the time code rate property.
  #[inline]
  pub async fn set_time_code_rate(&mut self, value: TimeCodeRate) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the video rate pull property.
  #[inline]
  pub async fn video_rate_pull(&mut self) -> Result<Container<VideoRatePull>> {
    self.get_property().await
  }

  /// Set the video rate pull property.
  #[inline]
  pub async fn set_video_rate_pull(&mut self, value: VideoRatePull) -> Result<()> {
    self.set_property(value).await
  }

  // ===========================================================================
  // Basic Edit Commands
  // ===========================================================================

  /// Consolidate the selected clips into a single clip.
  #[inline]
  pub async fn consolidate(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.consolidate_clip().await
  }

  /// Clear data on the current track.
  #[inline]
  pub async fn clear(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.clear().await
  }

  /// Clear data on the current track (with additional filters).
  #[inline]
  pub async fn clear_special(&mut self, options: AutomationDataOptions) -> Result<()> {
    self.status.assert_active();
    self.client.clear_special(options).await
  }

  /// Copy data on the current track.
  #[inline]
  pub async fn copy(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.copy().await
  }

  /// Copy data on the current track (with additional filters).
  #[inline]
  pub async fn copy_special(&mut self, options: AutomationDataOptions) -> Result<()> {
    self.status.assert_active();
    self.client.copy_special(options).await
  }

  /// Cut data on the current track.
  #[inline]
  pub async fn cut(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.cut().await
  }

  /// Cut data on the current track (with additional filters).
  #[inline]
  pub async fn cut_special(&mut self, options: AutomationDataOptions) -> Result<()> {
    self.status.assert_active();
    self.client.cut_special(options).await
  }

  /// Paste data on the current track.
  #[inline]
  pub async fn paste(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.paste().await
  }

  /// Paste data on the current track (with additional filters).
  #[inline]
  pub async fn paste_special(&mut self, options: PasteSpecialOptions) -> Result<()> {
    self.status.assert_active();
    self.client.paste_special(options).await
  }

  // ===========================================================================
  // Edit Commands
  // ===========================================================================

  /// Trim clips to the current selection.
  #[inline]
  pub async fn trim(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.trim_to_selection().await
  }

  /// Recall the specified zoom `preset`.
  #[inline]
  pub async fn recall_zoom(&mut self, preset: ZoomPreset) -> Result<()> {
    self.status.assert_active();
    self.client.recall_zoom_preset(preset.into_i32()).await
  }

  /// Refresh all modified audio files.
  #[inline]
  pub async fn refresh_all(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.refresh_all_modified_audio_files().await
  }

  /// Refresh the given list of audio files.
  #[inline]
  pub async fn refresh<I, P>(&mut self, iter: I) -> Result<RefreshList>
  where
    I: IntoIterator<Item = P>,
    P: AsRef<PtPath>,
  {
    self.status.assert_active();

    self
      .client
      .refresh_target_audio_files(collect_paths(iter))
      .await
      .map(RefreshList::new)
  }

  // ===========================================================================
  // Tranport Commands
  // ===========================================================================

  /// Get the current playback mode.
  #[inline]
  pub async fn playback_mode(&mut self) -> Result<Container<PlaybackMode>> {
    self.get_property().await
  }

  /// Set the current playback mode.
  #[inline]
  pub async fn set_playback_mode(&mut self, value: PlaybackMode) -> Result<()> {
    self.set_property(value).await
  }

  /// Get the current record mode.
  #[inline]
  pub async fn record_mode(&mut self) -> Result<Container<RecordMode>> {
    self.get_property().await
  }

  /// Set the current record mode.
  #[inline]
  pub async fn set_record_mode(&mut self, value: RecordMode, armed: bool) -> Result<()> {
    if armed {
      self.set_property(value.armed()).await
    } else {
      self.set_property(value).await
    }
  }

  /// Get the timeline selection.
  #[inline]
  pub async fn timeline_selection(
    &mut self,
    time_scale: TrackOffsetOptions,
  ) -> Result<TimelineSelection> {
    self.status.assert_active();

    self
      .client
      .get_timeline_selection(time_scale)
      .await
      .map(TimelineSelection::from_response)
  }

  /// Set the timeline selection.
  #[inline]
  pub async fn set_timeline_selection(&mut self, value: TimelineSelection) -> Result<()> {
    self.status.assert_active();
    value.into_request().send(&mut self.client).await
  }

  /// Returns `true` if the transport is currently armed.
  pub async fn transport_armed(&mut self) -> Result<bool> {
    self.status.assert_active();

    self
      .client
      .get_transport_armed()
      .await
      .map(|recv| recv.is_transport_armed)
  }

  /// Get the current transport state.
  #[inline]
  pub async fn transport_state(&mut self) -> Result<Container<TransportState>> {
    self.get_property().await
  }

  /// Start playback at half speed.
  #[inline]
  pub async fn play_half_speed(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.play_half_speed().await
  }

  /// Start recording at half speed.
  #[inline]
  pub async fn record_half_speed(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.record_half_speed().await
  }

  /// Toggle the current play state.
  #[inline]
  pub async fn toggle_play_state(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.toggle_play_state().await
  }

  /// Toggle the current record state.
  #[inline]
  pub async fn toggle_record_enable(&mut self) -> Result<()> {
    self.status.assert_active();
    self.client.toggle_record_enable().await
  }
}

// =============================================================================
// Misc. Utilities
// =============================================================================

fn collect_paths<I, P>(iter: I) -> Vec<String>
where
  I: IntoIterator<Item = P>,
  P: AsRef<PtPath>,
{
  iter
    .into_iter()
    .map(|path| path.as_ref().to_string())
    .collect()
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(i32)]
pub enum ZoomPreset {
  Z1 = 1,
  Z2 = 2,
  Z3 = 3,
  Z4 = 4,
  Z5 = 5,
}

impl ZoomPreset {
  #[inline]
  const fn into_i32(self) -> i32 {
    self as i32
  }
}
