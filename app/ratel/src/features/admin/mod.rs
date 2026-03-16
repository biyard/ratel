mod components;
mod controllers;
mod layout;
mod models;

mod views;

use components::*;
use controllers::*;
use models::*;

pub use layout::AppLayout as AdminLayout;
pub use views::AdminMainPage;

use crate::common::models::*;
use crate::*;
