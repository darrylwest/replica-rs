#![doc = include_str!("../README.md")]

pub mod config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PID_FILE: &str = "replica.pid";
