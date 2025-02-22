//! Constant values.

use std::time::Duration;

use crate::types::VersionList;

/// Default server endpoint for gRPC communication.
pub const ENDPOINT: &str = "http://localhost:31416";

/// Platform name of the Pro Tools application.
pub const PTSL_APPNAME: &str = "Pro Tools";

/// Current PTSL version.
pub const PTSL_VERSION: i32 = 1;

/// Timeout applied when connecting to the gRPC endpoint.
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(5);

/// Timeout applied to each gRPC request.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Maximum number of times to retry initial gRPC connection.
pub const RETRY_ATTEMPTS: u32 = 20;

/// Maximum time to retry initial connection attempts.
pub const RETRY_INTERVAL: Duration = Duration::from_secs(5);

/// Current PTSL client version.
pub static CLIENT_VERSION_LIST: VersionList = &[("Client", PTSL_VERSION)];

/// Current PTSL client API versions.
pub static METHOD_VERSION_LIST: VersionList = &[
  ("AuthorizeConnection", 1),
  ("Clear", 1),
  ("ClearMemoryLocation", 1),
  ("ClearSpecial", 1),
  ("CloseSession", 1),
  ("ConsolidateClip", 1),
  ("Copy", 1),
  ("CopySpecial", 1),
  ("CreateFadesBasedOnPreset", 1),
  ("CreateMemoryLocation", 1),
  ("CreateNewTracks", 1),
  ("CreateSession", 1),
  ("Cut", 1),
  ("CutSpecial", 1),
  ("EditMemoryLocation", 1),
  ("ExportClipsAsFiles", 1),
  ("ExportMix", 1),
  ("ExportSelectedTracksAsAAFOMF", 1),
  ("ExportSessionInfoAsText", 1),
  ("ExtendSelectionToTargetTracks", 1),
  ("GetDynamicProperties", 1),
  ("GetEditMode", 1),
  ("GetEditModeOptions", 1),
  ("GetEditTool", 1),
  ("GetFileLocation", 1),
  ("GetMemoryLocations", 1),
  ("GetPlaybackMode", 1),
  ("GetPTSLVersion", 1),
  ("GetRecordMode", 1),
  ("GetSessionAudioFormat", 1),
  ("GetSessionAudioRatePullSettings", 1),
  ("GetSessionBitDepth", 1),
  ("GetSessionFeetFramesRate", 1),
  ("GetSessionInterleavedState", 1),
  ("GetSessionLength", 1),
  ("GetSessionName", 1),
  ("GetSessionPath", 1),
  ("GetSessionSampleRate", 1),
  ("GetSessionStartTime", 1),
  ("GetSessionTimeCodeRate", 1),
  ("GetSessionVideoRatePullSettings", 1),
  ("GetTaskStatus", 1),
  ("GetTimelineSelection", 1),
  ("GetTrackList", 1),
  ("GetTransportArmed", 1),
  ("GetTransportState", 1),
  ("HostReadyCheck", 1),
  ("Import", 1),
  ("OpenSession", 1),
  ("Paste", 1),
  ("PasteSpecial", 1),
  ("PlayHalfSpeed", 1),
  ("RecallZoomPreset", 1),
  ("RecordHalfSpeed", 1),
  ("RefreshAllModifiedAudioFiles", 1),
  ("RefreshTargetAudioFiles", 1),
  ("RegisterConnection", 1),
  ("RenameSelectedClip", 1),
  ("RenameTargetClip", 1),
  ("RenameTargetTrack", 1),
  ("SaveSession", 1),
  ("SaveSessionAs", 1),
  ("SelectAllClipsOnTrack", 1),
  ("SelectTracksByName", 1),
  ("SetEditMode", 1),
  ("SetEditModeOptions", 1),
  ("SetEditTool", 1),
  ("SetPlaybackMode", 1),
  ("SetRecordMode", 1),
  ("SetSessionAudioFormat", 1),
  ("SetSessionAudioRatePullSettings", 1),
  ("SetSessionBitDepth", 1),
  ("SetSessionFeetFramesRate", 1),
  ("SetSessionInterleavedState", 1),
  ("SetSessionLength", 1),
  ("SetSessionStartTime", 1),
  ("SetSessionTimeCodeRate", 1),
  ("SetSessionVideoRatePullSettings", 1),
  ("SetTimelineSelection", 1),
  ("Spot", 1),
  ("TogglePlayState", 1),
  ("ToggleRecordEnable", 1),
  ("TrimToSelection", 1),
];
