mod components;
mod context;
mod models;
mod types;
mod views;

pub mod controllers;

use components::*;
use context::Context;
use context::*;
use controllers::*;

pub use models::*;
pub use types::*;

pub use views::*;

use super::poll::{Answer, ChoiceQuestion, Question};
use crate::*;

/// Maximum total attempts allowed (1 initial attempt + retries).
/// Enforced server-side to prevent unbounded DynamoDB queries and abuse.
pub const MAX_TOTAL_ATTEMPTS: i64 = 100;
