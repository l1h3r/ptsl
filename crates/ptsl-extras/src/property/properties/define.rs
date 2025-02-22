#![allow(missing_docs)] // TODO

use ptsl_client::client::Client;
use ptsl_client::error::Result;
use ptsl_protos::bridge::CommandExt;
use ptsl_protos::types::GetPlaybackModeResponseBody;
use ptsl_protos::types::GetRecordModeResponseBody;
use ptsl_protos::types::GetSessionLengthResponseBody;
use ptsl_protos::types::GetSessionStartTimeResponseBody;
use ptsl_protos::types::TrackOffsetOptions;

use crate::property::PropertySet;
use crate::property::RemoteProperty;
use crate::property::RemoteSettings;

// =============================================================================
// Session Audio Format
// =============================================================================

define_property! {
  pub enum AudioFormat: SessionAudioFormat {
    const Wave = SafWave;
    const Aiff = SafAiff;
  }
}

define_settings!(AudioFormat => GetSessionAudioFormatResponseBody);
define_ptsl_get!(AudioFormat => get_session_audio_format);
define_ptsl_set!(AudioFormat => set_session_audio_format);

// =============================================================================
// Session Bit Depth
// =============================================================================

define_property! {
  pub enum BitDepth: BitDepth {
    const Bit16 = Bit16;
    const Bit24 = Bit24;
    const Bit32 = Bit32Float;
    const _None = BitNone; // TODO: Make this go away
  }
}

define_settings!(BitDepth => GetSessionBitDepthResponseBody);
define_ptsl_get!(BitDepth => get_session_bit_depth);
define_ptsl_set!(BitDepth => set_session_bit_depth);

// =============================================================================
// Edit Mode
// =============================================================================

feature! {
  #![cfg(feature = "sdk-2023-9")]
  define_property! {
    pub enum EditMode: EditMode {
      const Unknown = EmoUnknown;
      const Shuffle = EmoShuffle;
      const Slip = EmoSlip;
      const Spot = EmoSpot;
      const GridAbsolute = EmoGridAbsolute;
      const GridRelative = EmoGridRelative;
      const ShuffleSnapToGridAbsolute = EmoShuffleSnapToGridAbsolute;
      const SlipSnapToGridAbsolute = EmoSlipSnapToGridAbsolute;
      const SpotSnapToGridAbsolute = EmoSpotSnapToGridAbsolute;
      const ShuffleSnapToGridRelative = EmoShuffleSnapToGridRelative;
      const SlipSnapToGridRelative = EmoSlipSnapToGridRelative;
      const SpotSnapToGridRelative = EmoSpotSnapToGridRelative;
    }
  }

  define_settings!(EditMode => GetEditModeResponseBody);
  define_ptsl_get!(EditMode => get_edit_mode);
  define_ptsl_set!(EditMode => set_edit_mode);
}

// =============================================================================
// Edit Tool
// =============================================================================

feature! {
  #![cfg(feature = "sdk-2023-9")]
  define_property! {
    pub enum EditTool: EditTool {
      const Unknown = EtUnknown;
      const ZoomNormal = EtZoomNormal;
      const ZoomSingle = EtZoomSingle;
      const TrimStandard = EtTrimStandard;
      const TrimTce = EtTrimTce;
      const TrimScrub = EtTrimScrub;
      const TrimLoop = EtTrimLoop;
      const Selector = EtSelector;
      const GrabberTime = EtGrabberTime;
      const GrabberSeparation = EtGrabberSeparation;
      const GrabberObject = EtGrabberObject;
      const SmartTool = EtSmartTool;
      const Scrubber = EtScrubber;
      const PencilFreeHand = EtPencilFreeHand;
      const PencilLine = EtPencilLine;
      const PencilTriangle = EtPencilTriangle;
      const PencilSquare = EtPencilSquare;
      const PencilRandom = EtPencilRandom;
      const PencilParabolic = EtPencilParabolic;
      const PencilSCurve = EtPencilSCurve;
    }
  }

  define_settings!(EditTool => GetEditToolResponseBody);
  define_ptsl_get!(EditTool => get_edit_tool);
  define_ptsl_set!(EditTool => set_edit_tool);
}

// =============================================================================
// Session Feet Frames Rate
// =============================================================================

define_property! {
  pub enum FeetFramesRate: SessionFeetFramesRate {
    const Fps23976 = SffrFps23976;
    const Fps24 = SffrFps24;
    const Fps25 = SffrFps25;
  }
}

define_settings!(FeetFramesRate => GetSessionFeetFramesRateResponseBody);
define_ptsl_get!(FeetFramesRate => get_session_feet_frames_rate);
define_ptsl_set!(FeetFramesRate => set_session_feet_frames_rate);

// =============================================================================
// Session Interleaved State
// =============================================================================

define_property! {
  pub struct Interleaved(bool);
}

impl Interleaved {
  pub const SIZE: usize = 2;
  pub const LIST: [Self; Self::SIZE] = [Self::new(true), Self::new(false)];
}

define_settings!(Interleaved => GetSessionInterleavedStateResponseBody, bool);
define_ptsl_get!(Interleaved => get_session_interleaved_state);
define_ptsl_set!(Interleaved => set_session_interleaved_state);

// =============================================================================
// Session Length
// =============================================================================

define_property! {
  #[no_copy]
  pub struct Length(String);
}

impl AsRef<str> for Length {
  #[inline]
  fn as_ref(&self) -> &str {
    self.inner.as_ref()
  }
}

impl RemoteSettings for GetSessionLengthResponseBody {
  type Data = String;
  type Iter = Option<String>;

  #[inline]
  fn expand(self) -> (Self::Data, Self::Iter) {
    (self.session_length, None)
  }
}

impl RemoteProperty for Length {
  type Settings = GetSessionLengthResponseBody;
}

define_ptsl_get!(Length => get_session_length);
define_ptsl_set!(Length => set_session_length);

// =============================================================================
// Session Playback Mode
// =============================================================================

define_property! {
  pub struct PlaybackMode: PmPlaybackMode as i32 {
    const NORMAL  = 0b00000001 => PmNormal;
    const LOOP    = 0b00000010 => PmLoop;
    const DYNAMIC = 0b00000100 => PmDynamicTransport;
  }
}

impl PlaybackMode {
  pub const SIZE: usize = 3;
  pub const LIST: [Self; Self::SIZE] = [Self::NORMAL, Self::LOOP, Self::DYNAMIC];
}

impl RemoteSettings for GetPlaybackModeResponseBody {
  type Data = PlaybackMode;
  type Iter = PlaybackMode;

  #[inline]
  fn expand(self) -> (Self::Data, Self::Iter) {
    (self.current_settings.into(), self.possible_settings.into())
  }
}

impl RemoteProperty for PlaybackMode {
  type Settings = GetPlaybackModeResponseBody;
}

define_ptsl_get!(PlaybackMode => get_playback_mode);

impl PropertySet for PlaybackMode {
  #[inline]
  async fn set(client: &mut Client, value: Self) -> Result<()> {
    for mode in value.expand() {
      client.set_playback_mode(mode).await?;
    }

    Ok(())
  }
}

// =============================================================================
// Session Rate Pull
// =============================================================================

define_property! {
  pub enum RatePull: SessionRatePull {
    const None = SrpNone;
    const Up01 = SrpUp01;
    const Down01 = SrpDown01;
    const Up4 = SrpUp4;
    const Up4Up01 = SrpUp4Up01;
    const Up4Down01 = SrpUp4Down01;
    const Down4 = SrpDown4;
    const Down4Up01 = SrpDown4Up01;
    const Down4Down01 = SrpDown4Down01;
  }
}

impl RatePull {
  #[inline]
  pub const fn audio(self) -> AudioRatePull {
    AudioRatePull::new(self)
  }

  #[inline]
  pub const fn video(self) -> VideoRatePull {
    VideoRatePull::new(self)
  }
}

// =============================================================================
// Session Audio Rate Pull
// =============================================================================

define_property! {
  pub struct AudioRatePull {
    @inner = RatePull;
    @outer = SessionRatePull;
  }
}

impl AudioRatePull {
  pub const LIST: [Self; RatePull::SIZE] = [
    Self::new(RatePull::None),
    Self::new(RatePull::Up01),
    Self::new(RatePull::Down01),
    Self::new(RatePull::Up4),
    Self::new(RatePull::Up4Up01),
    Self::new(RatePull::Up4Down01),
    Self::new(RatePull::Down4),
    Self::new(RatePull::Down4Up01),
    Self::new(RatePull::Down4Down01),
  ];
}

define_settings!(AudioRatePull => GetSessionAudioRatePullSettingsResponseBody);
define_ptsl_get!(AudioRatePull => get_session_audio_rate_pull_settings);
define_ptsl_set!(AudioRatePull => set_session_audio_rate_pull_settings);

// =============================================================================
// Session Video Rate Pull
// =============================================================================

define_property! {
  pub struct VideoRatePull {
    @inner = RatePull;
    @outer = SessionRatePull;
  }
}

impl VideoRatePull {
  pub const LIST: [Self; RatePull::SIZE] = [
    Self::new(RatePull::None),
    Self::new(RatePull::Up01),
    Self::new(RatePull::Down01),
    Self::new(RatePull::Up4),
    Self::new(RatePull::Up4Up01),
    Self::new(RatePull::Up4Down01),
    Self::new(RatePull::Down4),
    Self::new(RatePull::Down4Up01),
    Self::new(RatePull::Down4Down01),
  ];
}

define_settings!(VideoRatePull => GetSessionVideoRatePullSettingsResponseBody);
define_ptsl_get!(VideoRatePull => get_session_video_rate_pull_settings);
define_ptsl_set!(VideoRatePull => set_session_video_rate_pull_settings);

// =============================================================================
// Record Mode (disarmed)
// =============================================================================

define_property! {
  pub enum RecordMode: RmRecordMode {
    const Normal = RmNormal;
    const Loop = RmLoop;
    const Destructive = RmDestructive;
    const QuickPunch = RmQuickPunch;
    const TrackPunch = RmTrackPunch;
    const DestructivePunch = RmDestructivePunch;
  }
}

impl RecordMode {
  #[inline]
  pub const fn armed(self) -> RecordModeArmed {
    RecordModeArmed::new(self)
  }
}

define_settings!(RecordMode => GetRecordModeResponseBody);
define_ptsl_get!(RecordMode => get_record_mode);
define_ptsl_set!(RecordMode => set_record_mode(false));

// =============================================================================
// Record Mode (armed)
// =============================================================================

define_property! {
  pub struct RecordModeArmed {
    @inner = RecordMode;
    @outer = RmRecordMode;
  }
}

impl RecordModeArmed {
  pub const LIST: [Self; RecordMode::SIZE] = [
    Self::new(RecordMode::Normal),
    Self::new(RecordMode::Loop),
    Self::new(RecordMode::Destructive),
    Self::new(RecordMode::QuickPunch),
    Self::new(RecordMode::TrackPunch),
    Self::new(RecordMode::DestructivePunch),
  ];

  #[inline]
  pub const fn disarmed(self) -> RecordMode {
    self.into_inner()
  }
}

impl RemoteProperty for RecordModeArmed {
  type Settings = GetRecordModeResponseBody;
}

define_ptsl_get!(RecordModeArmed => get_record_mode);
define_ptsl_set!(RecordModeArmed => set_record_mode(true));

// =============================================================================
// Session Start Time
// =============================================================================

define_property! {
  #[no_copy]
  pub struct StartTime(String);
}

impl AsRef<str> for StartTime {
  #[inline]
  fn as_ref(&self) -> &str {
    self.inner.as_ref()
  }
}

impl RemoteSettings for GetSessionStartTimeResponseBody {
  type Data = String;
  type Iter = Option<String>;

  #[inline]
  fn expand(self) -> (Self::Data, Self::Iter) {
    (self.session_start_time, None)
  }
}

impl RemoteProperty for StartTime {
  type Settings = GetSessionStartTimeResponseBody;
}

define_ptsl_get!(StartTime => get_session_start_time);
define_ptsl_set!(StartTime => set_session_start_time(TrackOffsetOptions::TimeCode, false));

// =============================================================================
// Session TimeCode Rate
// =============================================================================

define_property! {
  pub enum TimeCodeRate: SessionTimeCodeRate {
    const Fps23976 = StcrFps23976;
    const Fps24 = StcrFps24;
    const Fps25 = StcrFps25;
    const Fps2997 = StcrFps2997;
    const Fps2997Drop = StcrFps2997Drop;
    const Fps30 = StcrFps30;
    const Fps30Drop = StcrFps30Drop;
    const Fps47952 = StcrFps47952;
    const Fps48 = StcrFps48;
    const Fps50 = StcrFps50;
    const Fps5994 = StcrFps5994;
    const Fps5994Drop = StcrFps5994Drop;
    const Fps60 = StcrFps60;
    const Fps60Drop = StcrFps60Drop;
    const Fps100 = StcrFps100;
    const Fps11988 = StcrFps11988;
    const Fps11988Drop = StcrFps11988Drop;
    const Fps120 = StcrFps120;
    const Fps120Drop = StcrFps120Drop;
  }
}

define_settings!(TimeCodeRate => GetSessionTimeCodeRateResponseBody);
define_ptsl_get!(TimeCodeRate => get_session_time_code_rate);
define_ptsl_set!(TimeCodeRate => set_session_time_code_rate);

// =============================================================================
// Session Transport State
// =============================================================================

define_property! {
  pub enum TransportState: TsTransportState {
    const Playing = TsTransportPlaying;
    const Stopped = TsTransportStopped;
    const Recording = TsTransportRecording;
    const PlayingHalfSpeed = TsTransportPlayingHalfSpeed;
    const RecordingHalfSpeed = TsTransportRecordingHalfSpeed;
    const FastForward = TsTransportFastForward;
    const Rewind = TsTransportRewind;
    const Scrub = TsTransportScrub;
    const Shuttle = TsTransportShuttle;
    const Primed = TsTransportPrimed;
    const IsCueing = TsTransportIsCueing;
    const IsCued = TsTransportIsCued;
    const IsCuedForPreview = TsTransportIsCuedForPreview;
    const IsStopping = TsTransportIsStopping;
    const IsPreviewing = TsTransportIsPreviewing;
  }
}

define_settings!(TransportState => GetTransportStateResponseBody);
define_ptsl_get!(TransportState => get_transport_state);
