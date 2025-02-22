use ptsl_client::client::Client;
use ptsl_client::error::Result;
use ptsl_protos::bridge::CommandExt;
use ptsl_protos::types::EditModeOptions;

// =============================================================================
// Edit Mode Options Builder
// =============================================================================

/// Builder type for [`EditModeOptions`] command.
#[derive(Debug)]
pub struct EditModeOptionsBuilder<'a> {
  client: &'a mut Client,
  option: EditModeOptions,
}

impl<'a> EditModeOptionsBuilder<'a> {
  pub(crate) fn new(client: &'a mut Client) -> Self {
    Self {
      client,
      option: EditModeOptions {
        tab_to_transients: false,
        link_timeline_and_edit_selection: false,
        link_track_and_edit_selection: false,
        insertion_follows_playback: false,
        automation_follows_edit: false,
        markers_follow_edit: false,
        mirrored_midi_editing: false,
        layered_editing: false,
      },
    }
  }

  /// Set the value of the `tab_to_transients` field.
  #[inline]
  pub const fn tab_to_transients(mut self, value: bool) -> Self {
    self.option.tab_to_transients = value;
    self
  }

  /// Set the value of the `link_timeline_and_edit_selection` field.
  #[inline]
  pub const fn link_timeline_and_edit_selection(mut self, value: bool) -> Self {
    self.option.link_timeline_and_edit_selection = value;
    self
  }

  /// Set the value of the `link_track_and_edit_selection` field.
  #[inline]
  pub const fn link_track_and_edit_selection(mut self, value: bool) -> Self {
    self.option.link_track_and_edit_selection = value;
    self
  }

  /// Set the value of the `insertion_follows_playback` field.
  #[inline]
  pub const fn insertion_follows_playback(mut self, value: bool) -> Self {
    self.option.insertion_follows_playback = value;
    self
  }

  /// Set the value of the `automation_follows_edit` field.
  #[inline]
  pub const fn automation_follows_edit(mut self, value: bool) -> Self {
    self.option.automation_follows_edit = value;
    self
  }

  /// Set the value of the `markers_follow_edit` field.
  #[inline]
  pub const fn markers_follow_edit(mut self, value: bool) -> Self {
    self.option.markers_follow_edit = value;
    self
  }

  /// Set the value of the `mirrored_midi_editing` field.
  #[inline]
  pub const fn mirrored_midi_editing(mut self, value: bool) -> Self {
    self.option.mirrored_midi_editing = value;
    self
  }

  /// Set the value of the `layered_editing` field.
  #[inline]
  pub const fn layered_editing(mut self, value: bool) -> Self {
    self.option.layered_editing = value;
    self
  }

  /// Send the command to the PTSL server.
  pub async fn send(self) -> Result<()> {
    self.client.set_edit_mode_options(Some(self.option)).await
  }
}
