//! Infrastructure layer for Velocity Console.
//!
//! This module contains platform-specific abstractions and external integrations:
//! - `platform`: Cross-distro path and terminal detection
//! - `database`: `SQLite` ledger operations (via tauri-plugin-sql)
//! - `credentials`: OS keyring integration

pub mod platform;

// These modules will be populated in later phases:
// pub mod database;
// pub mod credentials;
