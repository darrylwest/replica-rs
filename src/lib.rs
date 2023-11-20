#![doc = include_str!("../README.md")]

pub mod backup_process;
pub mod config;
pub mod file_model;
pub mod file_walker;
pub mod db_ops;

/// The current version as read from the cargo toml file
///
/// # Version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The current pid file that exists while the application is active.  
/// Use this to determine if the process is running.
///
/// # PID File
pub const PID_FILE: &str = "replica.pid";
