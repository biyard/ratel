mod constants;
mod utils;
mod views;

#[cfg(feature = "server")]
mod services;

pub mod components;
pub mod config;
pub mod controllers;
pub mod models;
pub mod types;

pub use common::utils::time::time_ago;
use components::*;
use config::*;
use constants::*;
use controllers::*;
use services::*;
use utils::*;

use auth::User;
pub use views::*;

use crate::*;
