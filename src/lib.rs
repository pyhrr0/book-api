pub mod config;
mod handlers;
pub mod layers;
pub mod models;
pub mod repositories;
pub mod routes;
mod server;
mod types;
mod utils;

#[macro_use]
extern crate tracing;

/// Application name
pub const APP_NAME: &str = "Book API";

pub use server::start_server;
pub use types::{AppError, AppErrorCode, AppResult, CliError, CliResult};
