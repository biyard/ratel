//! Launchpad company-point integration.
//!
//! Ratel acts as the external point provider for Launchpad's per-service
//! token economy. Launchpad calls our HMAC-signed callbacks; we delegate
//! balance/deduct to the Biyard console via `BiyardService`. We own no
//! point state except an idempotency ledger (`LaunchpadDeduction`).

pub mod config;
pub mod entry;
pub mod error;
pub mod handback;
pub mod round_info;
pub mod token_balance;
pub mod types;
pub mod views;

pub use i18n::*;

// Uses aes-gcm/hmac which are server/full-only deps (not in the web feature),
// and token encryption / signature verification only run server-side.
#[cfg(feature = "server")]
pub mod crypto;

#[cfg(feature = "server")]
pub mod controllers;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod server;

mod i18n;
