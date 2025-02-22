//! Protobuf -> Rust build script
//!
//! Note: Set `env=PTSL_PATH` to specify the protobuf file directory
//! Note: Set `env=PROTOC` for custom protoc path

use std::env::var_os;
use std::ffi::OsString;
use tonic_build::configure;
use tonic_build::Builder;

fn get_env(name: &str) -> OsString {
  let Some(value) = var_os(name) else {
    panic!("Failed to find environment variable {name}.");
  };

  value
}

fn deprecated(since: &'static str, note: &'static str) -> String {
  format!(r#"#[deprecated(since = "{since}", note = "{note}")]"#)
}

#[rustfmt::skip]
fn deprecated_2023_3(builder: Builder) -> Builder {
  static SINCE: &str = "2023.3";

  builder
    .message_attribute("AuthorizeConnectionRequestBody", deprecated(SINCE, "Use RegisterConnectionRequestBody"))
    .message_attribute("AuthorizeConnectionResponseBody", deprecated(SINCE, "Use RegisterConnectionResponseBody"))
    .field_attribute("RenameTargetTrackRequestBody.track_id", deprecated(SINCE, "Use current_name"))
    .field_attribute("ExportMixRequestBody.files_list", deprecated(SINCE, "Use mix_source_list"))
}

#[rustfmt::skip]
fn deprecated_2023_6(builder: Builder) -> Builder {
  static SINCE: &str = "2023.6";

  builder
    .field_attribute("SpotRequestBody.track_offset_options", deprecated(SINCE, "Use location_data.location_options"))
    .field_attribute("CreateMemoryLocationRequestBody.reference", deprecated(SINCE, "Use start_time"))
    .field_attribute("EditMemoryLocationRequestBody.reference", deprecated(SINCE, "Use start_time"))
}

#[rustfmt::skip]
fn deprecated_2023_9(builder: Builder) -> Builder {
  static SINCE: &str = "2023.9";

  builder
    .message_attribute("Pagination", deprecated(SINCE, "Use PaginationRequest"))
    .field_attribute("AudioData.destination", deprecated(SINCE, "Use audio_destination"))
    .field_attribute("AudioData.location", deprecated(SINCE, "Use audio_location"))
    .field_attribute("ImportResponseBody.file_list", deprecated(SINCE, "Use audio_data"))
    .field_attribute("ImportResponseBody.audio_operations", deprecated(SINCE, "Use audio_data"))
    .field_attribute("ImportResponseBody.destination_path", deprecated(SINCE, "Use audio_data"))
    .field_attribute("GetTrackListRequestBody.page_limit", deprecated(SINCE, "Use pagination_request"))
    .field_attribute("GetTrackListResponseBody.stats", deprecated(SINCE, "Use pagination_response"))
    .field_attribute("GetFileLocationRequestBody.page_limit", deprecated(SINCE, "Use pagination_request"))
    .field_attribute("GetFileLocationResponseBody.stats", deprecated(SINCE, "Use pagination_response"))
}

fn assert_version() {
  if cfg!(feature = "sdk-2023-3") {
    println!("Using SDK Version: 2023.3");
  } else if cfg!(feature = "sdk-2023-6") {
    println!("Using SDK Version: 2023.6");
  } else if cfg!(feature = "sdk-2023-9") {
    println!("Using SDK Version: 2023.9");
  } else {
    panic!("SDK version not set. Expected SDK version feature.");
  }
}

fn main() {
  assert_version();

  println!("cargo:rerun-if-changed=build.rs");
  println!("cargo:rerun-if-env-changed=PTSL_PATH");

  let mut config: Builder = configure()
    .build_client(true)
    .build_server(false)
    .build_transport(true)
    .emit_rerun_if_changed(true)
    .enum_attribute(".", "#[derive(::ptsl_derive::ProtoType)]")
    .message_attribute(".", "#[derive(::ptsl_derive::ProtoType)]");

  if cfg!(feature = "sdk-2023-3") {
    config = deprecated_2023_3(config);
  }

  if cfg!(feature = "sdk-2023-6") {
    config = deprecated_2023_3(config);
    config = deprecated_2023_6(config);
  }

  if cfg!(feature = "sdk-2023-9") {
    config = deprecated_2023_3(config);
    config = deprecated_2023_6(config);
    config = deprecated_2023_9(config);
  }

  config
    .compile(&["PTSL.proto"], &[get_env("PTSL_PATH")])
    .expect("Failed to compile protobuf files");
}
