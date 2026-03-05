#[cfg(feature = "server")]
mod server;
#[cfg(feature = "server")]
pub use server::*;
#[cfg(feature = "server")]
pub type CommonConfig = server::ServerConfig;

#[cfg(not(feature = "server"))]
mod web;
#[cfg(not(feature = "server"))]
pub use web::*;
#[cfg(not(feature = "server"))]
pub type CommonConfig = web::WebConfig;

mod environment;
mod log_level;
mod portone_config;

pub use environment::*;
pub use log_level::*;
pub use portone_config::*;
