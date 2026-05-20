//! Game-specific channel handlers + event payloads for *Fact or
//! Fold*. Each handler `impl RoomChannel` and registers itself with
//! the global hub when the module loads (PR4f).

#[cfg(feature = "server")]
pub mod chat;
#[cfg(feature = "server")]
pub mod register;

#[cfg(feature = "server")]
pub use chat::*;
#[cfg(feature = "server")]
pub use register::*;
