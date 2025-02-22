//! Command definitions.
#![allow(deprecated)]
#![allow(clippy::too_many_arguments)]

use ptsl_derive::commands;

use crate::types::AafFileBitDepth;
use crate::types::AudioData;
use crate::types::AutomationDataOptions;
use crate::types::BitDepth;
use crate::types::ClClipLocation;
use crate::types::CopyOption;
use crate::types::DynamicPropertyType;
use crate::types::EmAudioInfo;
use crate::types::EmDolbyAtmosInfo;
use crate::types::EmFileType;
use crate::types::EmLocationInfo;
use crate::types::EmSourceInfo;
use crate::types::EmVideoInfo;
use crate::types::EsiOutputType;
use crate::types::ExportAsAafFileType;
use crate::types::ExportFileType;
use crate::types::ExportFormat;
use crate::types::FadeHandlingType;
use crate::types::FileLocationTypeFilter;
use crate::types::FileType;
use crate::types::ImportType;
use crate::types::IoSettings;
use crate::types::MemoryLocationProperties;
use crate::types::MemoryLocationReference;
use crate::types::PasteSpecialOptions;
use crate::types::PmPlaybackMode;
use crate::types::ResolveDuplicateNamesBy;
use crate::types::RmRecordMode;
use crate::types::SampleRate;
use crate::types::SessionAudioFormat;
use crate::types::SessionData;
use crate::types::SessionFeetFramesRate;
use crate::types::SessionRatePull;
use crate::types::SessionTimeCodeRate;
use crate::types::SpotLocationData;
use crate::types::TextAsFileFormat;
use crate::types::TimeProperties;
use crate::types::TrackListInvertibleFilter;
use crate::types::TrackListType;
use crate::types::TrackOffsetOptions;
use crate::types::TripleBool;

feature! {
  #![cfg(feature = "sdk-2023-9")]
  use crate::types::EditMode;
  use crate::types::EditModeOptions;
  use crate::types::EditTool;
  use crate::types::PaginationRequest;
  use crate::types::SelectionMode;
  use crate::types::TimelineUpdateVideo;
  use crate::types::TrackFormat;
  use crate::types::TrackTimebase;
  use crate::types::TrackType;
}

commands! {
  // ===========================================================================
  // Base Commands
  // ===========================================================================

  command AuthorizeConnection(send, recv) {
    auth_string: String,
  }

  command Clear() {}

  command ClearMemoryLocation(send) {
    location_list: Vec<i32>,
  }

  command ClearSpecial(send) {
    automation_data_option: AutomationDataOptions,
  }

  command CloseSession(ping, send) {
    save_on_close: bool,
  }

  command ConsolidateClip() {}

  command Copy() {}

  command CopySpecial(send) {
    automation_data_option: AutomationDataOptions,
  }

  command CreateFadesBasedOnPreset(send, recv) {
    fade_preset_name: String,
    auto_adjust_bounds: bool,
  }

  command CreateMemoryLocation(send, recv) {
    number: i32,
    name: String,
    start_time: String,
    end_time: String,
    time_properties: TimeProperties,
    reference: MemoryLocationReference,
    general_properties: Option<MemoryLocationProperties>,
    comments: String,
  }

  command CreateSession(ping, send) {
    session_name: String,
    create_from_template: bool,
    template_group: String,
    template_name: String,
    file_type: FileType,
    sample_rate: SampleRate,
    input_output_settings: IoSettings,
    is_interleaved: bool,
    session_location: String,
    is_cloud_project: bool,
    create_from_aaf: bool,
    path_to_aaf: String,
    bit_depth: BitDepth,
  }

  command Cut() {}

  command CutSpecial(send) {
    automation_data_option: AutomationDataOptions,
  }

  command EditMemoryLocation(send, recv) {
    number: i32,
    name: String,
    start_time: String,
    end_time: String,
    time_properties: TimeProperties,
    reference: MemoryLocationReference, // deprecated
    general_properties: Option<MemoryLocationProperties>,
    comments: String,
  }

  command ExportClipsAsFiles(ping, send) {
    file_path: String,
    format: ExportFormat,
    file_type: ExportFileType,
    bit_depth: BitDepth,
    duplicate_names: ResolveDuplicateNamesBy,
    enforce_avid_compatibility: bool,
  }

  command ExportMix(ping, send) {
    preset_path: String,
    file_name: String,
    file_type: EmFileType,
    files_list: Vec<EmSourceInfo>,
    audio_info: Option<EmAudioInfo>,
    video_info: Option<EmVideoInfo>,
    location_info: Option<EmLocationInfo>,
    dolby_atmos_info: Option<EmDolbyAtmosInfo>,
    offline_bounce: TripleBool,
    mix_source_list: Vec<EmSourceInfo>,
  }

  command ExportSelectedTracksAsAafomf(ping, send) {
    file_type: ExportAsAafFileType,
    bit_depth: AafFileBitDepth,
    copy_option: CopyOption,
    enforce_media_composer_compatibility: bool,
    quantize_edits_to_frame_boundaries: bool,
    export_stereo_as_multichannel: bool,
    container_file_name: String,
    container_file_location: String,
    asset_file_location: String,
    comments: String,
    sequence_name: String,
  }

  command ExportSessionInfoAsText(ping, send, recv) {
    include_file_list: bool,
    include_clip_list: bool,
    include_markers: bool,
    include_plugin_list: bool,
    include_track_edls: bool,
    show_sub_frames: bool,
    include_user_timestamps: bool,
    track_list_type: TrackListType,
    fade_handling_type: FadeHandlingType,
    track_offset_options: TrackOffsetOptions,
    text_as_file_format: TextAsFileFormat,
    output_type: EsiOutputType,
    output_path: String,
  }

  command ExtendSelectionToTargetTracks(send, recv) {
    tracks_to_extend_to: Vec<String>,
  }

  command GetDynamicProperties(send, recv) {
    property_type: DynamicPropertyType,
  }

  command GetFileLocation(send, recv) {
    page_limit: i32,
    file_filters: Vec<FileLocationTypeFilter>,
    #[cfg(feature = "sdk-2023-9")]
    pagination_request: Option<PaginationRequest>,
  }

  command GetMemoryLocations(send, recv) {}

  command GetPlaybackMode(recv) {}

  command GetPtslVersion(recv) {}

  command GetRecordMode(recv) {}

  command GetSessionAudioFormat(recv) {}

  command GetSessionAudioRatePullSettings(recv) {}

  command GetSessionBitDepth(recv) {}

  command GetSessionFeetFramesRate(recv) {}

  command GetSessionInterleavedState(recv) {}

  command GetSessionLength(recv) {}

  command GetSessionName(recv) {}

  command GetSessionPath(recv) {}

  command GetSessionSampleRate(recv) {}

  command GetSessionStartTime(recv) {}

  command GetSessionTimeCodeRate(recv) {}

  command GetSessionVideoRatePullSettings(recv) {}

  command GetTaskStatus(sync, send, recv) {
    task_id: String,
  }

  command GetTrackList(send, recv) {
    page_limit: i32,
    track_filter_list: Vec<TrackListInvertibleFilter>,
    is_filter_list_additive: bool,
    #[cfg(feature = "sdk-2023-9")]
    pagination_request: Option<PaginationRequest>,
  }

  command GetTransportArmed(recv) {}

  command GetTransportState(recv) {}

  command HostReadyCheck() {}

  command Import(ping, send) { // TODO: recv
    session_path: String,
    import_type: ImportType,
    session_data: Option<SessionData>,
    audio_data: Option<AudioData>,
  }

  command OpenSession(ping, send) {
    session_path: String,
  }

  command Paste() {}

  command PasteSpecial(send) {
    paste_special_option: PasteSpecialOptions,
  }

  command PlayHalfSpeed() {}

  command RecordHalfSpeed() {}

  command RefreshAllModifiedAudioFiles() {}

  command RefreshTargetAudioFiles(send, recv) {
    file_list: Vec<String>,
  }

  command RegisterConnection(send, recv) {
    company_name: String,
    application_name: String,
  }

  command RenameSelectedClip(send) {
    clip_location: ClClipLocation,
    new_name: String, // TODO: required
    rename_file: bool,
  }

  command RenameTargetClip(send) {
    clip_name: String, // TODO: required (or new_name)
    new_name: String, // TODO: required (or clip_name)
    rename_file: bool,
  }

  command RenameTargetTrack(send) {
    track_id: String,
    new_name: String, // TODO: required (or current_name)
    current_name: String, // TODO: required (or new_name)
  }

  command SaveSession(ping) {}

  command SaveSessionAs(ping, send) {
    session_name: String,
    session_location: String,
  }

  command SelectAllClipsOnTrack(send, recv) {
    track_name: String,
  }

  command SetPlaybackMode(send) { // TODO: recv
    playback_mode: PmPlaybackMode,
  }

  command SetRecordMode(send) { // TODO: recv
    record_mode: RmRecordMode,
    record_arm_transport: bool,
  }

  command SetSessionAudioFormat(send) {
    audio_format: SessionAudioFormat,
  }

  command SetSessionAudioRatePullSettings(send) {
    audio_rate_pull: SessionRatePull,
  }

  command SetSessionBitDepth(send) {
    bit_depth: BitDepth,
  }

  command SetSessionFeetFramesRate(send) {
    feet_frames_rate: SessionFeetFramesRate,
  }

  command SetSessionInterleavedState(send) {
    interleaved_state: bool,
  }

  command SetSessionLength(send) {
    session_length: String,
  }

  command SetSessionStartTime(send) {
    session_start_time: String,
    track_offset_opts: TrackOffsetOptions,
    maintain_relative_position: bool,
  }

  command SetSessionTimeCodeRate(send) {
    time_code_rate: SessionTimeCodeRate,
  }

  command SetSessionVideoRatePullSettings(send) {
    video_rate_pull: SessionRatePull,
  }

  command Spot(ping, send) {
    track_offset_options: TrackOffsetOptions,
    location_data: Option<SpotLocationData>,
  }

  command TogglePlayState() {}

  command ToggleRecordEnable() {}

  command TrimToSelection() {}

  // ===========================================================================
  // 2023.9
  // ===========================================================================

  #[cfg(feature = "sdk-2023-9")]
  command CreateNewTracks(send, recv) {
    number_of_tracks: i32,
    track_name: String,
    track_format: TrackFormat,
    track_type: TrackType,
    track_timebase: TrackTimebase,
  }

  #[cfg(feature = "sdk-2023-9")]
  command SelectTracksByName(send, recv) {
    track_names: Vec<String>,
    selection_mode: SelectionMode,
    pagination_request: Option<PaginationRequest>,
  }

  #[cfg(feature = "sdk-2023-9")]
  command GetEditMode(recv) {}

  #[cfg(feature = "sdk-2023-9")]
  command SetEditMode(send) {
    edit_mode: EditMode,
  }

  #[cfg(feature = "sdk-2023-9")]
  command GetEditTool(recv) {}

  #[cfg(feature = "sdk-2023-9")]
  command SetEditTool(send) {
    edit_tool: EditTool,
  }

  #[cfg(feature = "sdk-2023-9")]
  command RecallZoomPreset(send) {
    zoom_preset: i32,
  }

  #[cfg(feature = "sdk-2023-9")]
  command GetEditModeOptions(recv) {}

  #[cfg(feature = "sdk-2023-9")]
  command SetEditModeOptions(send) {
    edit_mode_options: Option<EditModeOptions>,
  }

  #[cfg(feature = "sdk-2023-9")]
  command GetTimelineSelection(send, recv) {
    time_scale: TrackOffsetOptions,
  }

  #[cfg(feature = "sdk-2023-9")]
  command SetTimelineSelection(send) {
    play_start_marker_time: String,
    in_time: String,
    out_time: String,
    pre_roll_start_time: String,
    post_roll_stop_time: String,
    pre_roll_enabled: TripleBool,
    post_roll_enabled: TripleBool,
    update_video_to: TimelineUpdateVideo,
    propagate_to_satellites: TripleBool,
  }
}
