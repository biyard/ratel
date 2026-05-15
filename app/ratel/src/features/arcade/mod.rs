//! 라텔 오락실 — 미니게임 플랫폼.
//!
//! arcade-level 추상(wallet / realtime / services)과 그 위에 얹히는
//! 게임 모듈들 (`games::<name>`) 의 owner. v1은 Fact or Fold 1개.
//!
//! Module layout (design doc 2026-05-15):
//! - `wallet/`    — `ArcadeWallet` trait (chip ↔ RP, buy_in, settle)
//! - `realtime/`  — `RoomChannel` trait + in-process hub (SSE-first, future WS)
//! - `services/`  — `StageScheduler` trait + generic `advance_if_due`
//! - `models/`    — arcade-level DDB entities (wallet balance, txn, settings)
//! - `games/`     — registered mini-games (each implements the traits)
//! - `error.rs`   — `ArcadeError` umbrella
//!
//! pages / hooks / components / layout / route etc. land in
//! follow-up PRs (PR4c+).

pub mod controllers;
pub mod error;
pub mod games;
#[cfg(feature = "server")]
pub mod models;
pub mod realtime;
#[cfg(feature = "server")]
pub mod server;
pub mod services;
pub mod types;
#[cfg(feature = "server")]
pub mod wallet;

pub use controllers::*;
pub use error::*;
pub use games::*;
#[cfg(feature = "server")]
pub use models::*;
pub use realtime::*;
pub use services::*;
pub use types::*;
#[cfg(feature = "server")]
pub use wallet::*;
