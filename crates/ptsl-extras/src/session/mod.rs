//! Pro Tools session types.

mod dynamic_properties;
mod refresh_list;
mod session;
mod session_path;
mod status;

pub use self::dynamic_properties::DynContainer;
pub use self::dynamic_properties::DynDescriptor;
pub use self::dynamic_properties::DynGroup;
pub use self::dynamic_properties::DynamicProperties;
pub use self::refresh_list::RefreshList;
pub use self::session::Session;
pub use self::session_path::SessionPath;
pub use self::status::Status;

feature! {
  #![cfg(feature = "sdk-2023-9")]
  mod edit_mode_options;
  mod timeline_selection;

  pub use self::edit_mode_options::EditModeOptionsBuilder;
  pub use self::session::ZoomPreset;
  pub use self::timeline_selection::TimelineSelection;
}
