//! Protobuf -> Rust build script
//!
//! Note: Set `env=PTSL_PATH` to specify the protobuf file directory
//! Note: Set `env=PROTOC` for custom protoc path
use std::env::var_os;
use std::ffi::OsString;
use tonic_build::configure;

fn get_env(name: &str) -> OsString {
  let Some(value) = var_os(name) else {
    panic!("Failed to find environment variable {name}.");
  };

  value
}

fn deprecated(since: &'static str, note: &'static str) -> String {
  format!(r#"#[deprecated(since = "{since}", note = "{note}")]"#)
}

fn deprecated_2023_3(note: &'static str) -> String {
  deprecated("2023.3", note)
}

fn deprecated_2023_6(note: &'static str) -> String {
  deprecated("2023.6", note)
}

fn deprecated_2023_9(note: &'static str) -> String {
  deprecated("2023.9", note)
}

#[rustfmt::skip]
fn main() {
  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-env-changed=PTSL_PATH");

  configure()
    .build_client(true)
    .build_server(false)
    .build_transport(true)
    .emit_rerun_if_changed(true)
    .enum_attribute(".", "#[derive(::serde::Deserialize, ::serde::Serialize)]")
    .message_attribute(".", "#[derive(::serde::Deserialize, ::serde::Serialize)]")
    // =========================================================================
    // Deprecated (2023.3)
    // =========================================================================
    .message_attribute("AuthorizeConnectionRequestBody", deprecated_2023_3("Use RegisterConnectionRequestBody"))
    .message_attribute("AuthorizeConnectionResponseBody", deprecated_2023_3("Use RegisterConnectionResponseBody"))
    .field_attribute("RenameTargetTrackRequestBody.track_id", deprecated_2023_3("Use current_name"))
    .field_attribute("ExportMixRequestBody.mix_source_list", deprecated_2023_3("Use files_list"))
    // =========================================================================
    // Deprecated (2023.6)
    // =========================================================================
    .field_attribute("SpotRequestBody.track_offset_options", deprecated_2023_6("Use location_data.location_options"))
    .field_attribute("CreateMemoryLocationRequestBody.reference", deprecated_2023_6("Use start_time"))
    .field_attribute("EditMemoryLocationRequestBody.reference", deprecated_2023_6("Use start_time"))
    // =========================================================================
    // Deprecated (2023.9)
    // =========================================================================
    .message_attribute("Pagination", deprecated_2023_9("Use PaginationRequest"))
    .field_attribute("AudioData.destination", deprecated_2023_9("Use audio_destination"))
    .field_attribute("AudioData.location", deprecated_2023_9("Use audio_location"))
    .field_attribute("ImportResponseBody.file_list", deprecated_2023_9("Use audio_data"))
    .field_attribute("ImportResponseBody.audio_operations", deprecated_2023_9("Use audio_data"))
    .field_attribute("ImportResponseBody.destination_path", deprecated_2023_9("Use audio_data"))
    .field_attribute("GetTrackListRequestBody.page_limit", deprecated_2023_9("Use pagination_request"))
    .field_attribute("GetTrackListResponseBody.stats", deprecated_2023_9("Use pagination_response"))
    .field_attribute("GetFileLocationRequestBody.page_limit", deprecated_2023_9("Use pagination_request"))
    .field_attribute("GetFileLocationResponseBody.stats", deprecated_2023_9("Use pagination_response"))
    // =========================================================================
    // Compile
    // =========================================================================
    .compile(&["PTSL.proto"], &[get_env("PTSL_PATH")])
    .expect("Failed to compile protobuf files");
}
