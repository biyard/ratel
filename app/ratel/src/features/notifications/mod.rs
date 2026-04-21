pub mod types;

pub mod controllers;

#[cfg(feature = "server")]
pub mod route;

#[cfg(not(feature = "server"))]
pub mod hooks;

#[cfg(not(feature = "server"))]
pub mod components;

#[cfg(not(feature = "server"))]
pub mod i18n;

#[cfg(not(feature = "server"))]
pub use components::*;

pub use types::*;
