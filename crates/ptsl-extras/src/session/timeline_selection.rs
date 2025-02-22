use ptsl_protos::types::GetTimelineSelectionResponseBody;
use ptsl_protos::types::SetTimelineSelectionRequestBody;
use ptsl_protos::types::TimelineUpdateVideo;

use crate::utils::triple_bool;

// =============================================================================
// Timeline Selection
// =============================================================================

/// Session timeline selection properties.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimelineSelection {
  play_start_marker_time: String,
  in_time: String,
  out_time: String,
  pre_roll_start_time: String,
  post_roll_stop_time: String,
  pre_roll_enabled: Option<bool>,
  post_roll_enabled: Option<bool>,
  update_video_to: TimelineUpdateVideo,
  propagate_to_satellites: Option<bool>,
}

impl TimelineSelection {
  pub(crate) fn from_response(value: GetTimelineSelectionResponseBody) -> Self {
    Self {
      play_start_marker_time: value.play_start_marker_time,
      in_time: value.in_time,
      out_time: value.out_time,
      pre_roll_start_time: value.pre_roll_start_time,
      post_roll_stop_time: value.post_roll_stop_time,
      pre_roll_enabled: Some(value.pre_roll_enabled),
      post_roll_enabled: Some(value.post_roll_enabled),
      update_video_to: TimelineUpdateVideo::TuvUnknown,
      propagate_to_satellites: None,
    }
  }

  pub(crate) fn into_request(self) -> SetTimelineSelectionRequestBody {
    SetTimelineSelectionRequestBody {
      play_start_marker_time: self.play_start_marker_time,
      in_time: self.in_time,
      out_time: self.out_time,
      pre_roll_start_time: self.pre_roll_start_time,
      post_roll_stop_time: self.post_roll_stop_time,
      pre_roll_enabled: triple_bool(self.pre_roll_enabled).into(),
      post_roll_enabled: triple_bool(self.post_roll_enabled).into(),
      update_video_to: self.update_video_to.into(),
      propagate_to_satellites: triple_bool(self.propagate_to_satellites).into(),
    }
  }

  /// Create a new `TimelineSelection`.
  pub fn new() -> Self {
    Self {
      play_start_marker_time: String::new(),
      in_time: String::new(),
      out_time: String::new(),
      pre_roll_start_time: String::new(),
      post_roll_stop_time: String::new(),
      pre_roll_enabled: None,
      post_roll_enabled: None,
      update_video_to: TimelineUpdateVideo::TuvNone,
      propagate_to_satellites: None,
    }
  }

  // ===========================================================================
  // Getters
  // ===========================================================================

  /// Returns the value of the `play_start_marker_time` field.
  #[inline]
  pub fn play_start_marker_time(&self) -> &str {
    self.play_start_marker_time.as_str()
  }

  /// Returns the value of the `in_time` field.
  #[inline]
  pub fn in_time(&self) -> &str {
    self.in_time.as_str()
  }

  /// Returns the value of the `out_time` field.
  #[inline]
  pub fn out_time(&self) -> &str {
    self.out_time.as_str()
  }

  /// Returns the value of the `pre_roll_start_time` field.
  #[inline]
  pub fn pre_roll_start_time(&self) -> &str {
    self.pre_roll_start_time.as_str()
  }

  /// Returns the value of the `post_roll_stop_time` field.
  #[inline]
  pub fn post_roll_stop_time(&self) -> &str {
    self.post_roll_stop_time.as_str()
  }

  /// Returns the value of the `pre_roll_enabled` field.
  #[inline]
  pub fn pre_roll_enabled(&self) -> bool {
    self.pre_roll_enabled.unwrap_or_default()
  }

  /// Returns the value of the `post_roll_enabled` field.
  #[inline]
  pub fn post_roll_enabled(&self) -> bool {
    self.post_roll_enabled.unwrap_or_default()
  }

  /// Returns the value of the `update_video_to` field.
  #[inline]
  pub fn update_video_to(&self) -> TimelineUpdateVideo {
    self.update_video_to
  }

  /// Returns the value of the `propagate_to_satellites` field.
  #[inline]
  pub fn propagate_to_satellites(&self) -> bool {
    self.propagate_to_satellites.unwrap_or_default()
  }

  // ===========================================================================
  // Setters
  // ===========================================================================

  /// Sets the value of the `play_start_marker_time` field.
  #[inline]
  pub fn set_play_start_marker_time(mut self, value: impl Into<String>) -> Self {
    self.play_start_marker_time = value.into();
    self
  }

  /// Sets the value of the `in_time` field.
  #[inline]
  pub fn set_in_time(mut self, value: impl Into<String>) -> Self {
    self.in_time = value.into();
    self
  }

  /// Sets the value of the `out_time` field.
  #[inline]
  pub fn set_out_time(mut self, value: impl Into<String>) -> Self {
    self.out_time = value.into();
    self
  }

  /// Sets the value of the `pre_roll_start_time` field.
  #[inline]
  pub fn set_pre_roll_start_time(mut self, value: impl Into<String>) -> Self {
    self.pre_roll_start_time = value.into();
    self
  }

  /// Sets the value of the `post_roll_stop_time` field.
  #[inline]
  pub fn set_post_roll_stop_time(mut self, value: impl Into<String>) -> Self {
    self.post_roll_stop_time = value.into();
    self
  }

  /// Sets the value of the `pre_roll_enabled` field.
  #[inline]
  pub fn set_pre_roll_enabled(mut self, value: bool) -> Self {
    self.pre_roll_enabled = Some(value);
    self
  }

  /// Sets the value of the `post_roll_enabled` field.
  #[inline]
  pub fn set_post_roll_enabled(mut self, value: bool) -> Self {
    self.post_roll_enabled = Some(value);
    self
  }

  /// Sets the value of the `update_video_to` field.
  #[inline]
  pub fn set_update_video_to(mut self, value: TimelineUpdateVideo) -> Self {
    self.update_video_to = value;
    self
  }

  /// Sets the value of the `propagate_to_satellites` field.
  #[inline]
  pub fn set_propagate_to_satellites(mut self, value: bool) -> Self {
    self.propagate_to_satellites = Some(value);
    self
  }
}
