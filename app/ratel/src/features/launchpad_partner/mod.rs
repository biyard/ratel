//! Launchpad company-point integration.
//!
//! Ratel acts as the external point provider for Launchpad's per-service
//! token economy. Launchpad calls our HMAC-signed callbacks; we delegate
//! balance/deduct to the Biyard console via `BiyardService`. We own no
//! point state except an idempotency ledger (`LaunchpadDeduction`).

pub mod config;
pub mod crypto;
pub mod error;
pub mod types;
pub mod views;

#[cfg(feature = "server")]
pub mod controllers;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod server;

mod i18n;
