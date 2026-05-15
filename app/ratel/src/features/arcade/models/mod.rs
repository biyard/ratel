//! arcade-level DynamoDB entities (server-only).
//!
//! - `ArcadeWalletBalance` — per-user chip balance singleton
//! - `ArcadeWalletTransaction` — append-only ledger row (convert /
//!   buy-in / settle)
//! - `ArcadeSettings` — singleton arcade-wide tunables
//!
//! Game-specific entities live under `games::<name>::models`.

pub mod arcade_settings;
pub mod arcade_wallet_balance;
pub mod arcade_wallet_transaction;

pub use arcade_settings::*;
pub use arcade_wallet_balance::*;
pub use arcade_wallet_transaction::*;
