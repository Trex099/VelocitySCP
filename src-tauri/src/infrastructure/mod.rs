//! Infrastructure layer for Velocity Console.
//!
//! This module contains platform-specific abstractions and external integrations:
//! - `platform`: Cross-distro path and terminal detection
//! - `database`: `SQLite` ledger operations
//! - `credentials`: OS keyring integration

pub mod database;
pub mod platform;

// This module will be populated in Phase 1.3:
// pub mod credentials;
