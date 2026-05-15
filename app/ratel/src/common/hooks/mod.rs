mod use_infinite_query;
mod use_interval;
#[cfg(feature = "mobile")]
mod use_loader;
mod use_origin;
mod use_platform;
mod use_scroll_lock;

pub use use_infinite_query::*;
pub use use_interval::*;
#[cfg(feature = "mobile")]
pub use use_loader::*;

// Pick `use_loader` matching the active `Loader<T>` type:
// - under `tauri-web`: our reqwest-backed Loader from `common::fullstack`
// - otherwise: dioxus's stock `use_loader`
#[cfg(all(not(feature = "mobile"), feature = "tauri-web"))]
pub use crate::common::fullstack::use_loader;
#[cfg(all(not(feature = "mobile"), not(feature = "tauri-web")))]
pub use dioxus::prelude::use_loader;

pub use use_origin::*;
pub use use_platform::*;
pub use use_scroll_lock::*;
