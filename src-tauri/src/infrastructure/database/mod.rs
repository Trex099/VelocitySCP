//! Database infrastructure for Velocity Console.
//!
//! Provides `SQLite` database management including:
//! - Connection handling with foreign key enforcement
//! - Migration system for schema versioning
//! - Database initialization at application startup

mod connection;
mod migrations;
mod models;

pub use connection::{Database, DatabaseError};
pub use migrations::{Migration, MigrationError, MigrationRunner};
pub use models::MigrationRecord;

/// Database file name within the app data directory.
pub const DATABASE_FILENAME: &str = "velocity.db";

/// Application name for directory paths.
pub const APP_NAME: &str = "velocity-console";
