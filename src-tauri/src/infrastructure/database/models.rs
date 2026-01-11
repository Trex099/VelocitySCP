//! Database models for internal use.

use serde::{Deserialize, Serialize};

/// Record of an applied migration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// Auto-incremented ID.
    pub id: i64,
    /// Migration name (e.g., `001_initial_schema`).
    pub name: String,
    /// ISO 8601 timestamp when applied.
    pub applied_at: String,
}
