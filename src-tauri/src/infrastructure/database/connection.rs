//! Database connection management.
//!
//! Provides the core [`Database`] struct for `SQLite` operations.

use std::path::PathBuf;

use thiserror::Error;

use crate::infrastructure::platform::app_data_dir;

use super::{MigrationRunner, DATABASE_FILENAME};

/// Errors that can occur during database operations.
#[derive(Debug, Error)]
pub enum DatabaseError {
    /// Failed to create database directory.
    #[error("failed to create database directory: {0}")]
    CreateDirFailed(#[from] std::io::Error),

    /// `SQLite` error.
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Migration error.
    #[error("migration failed: {0}")]
    Migration(#[from] super::MigrationError),
}

/// `SQLite` database wrapper with foreign key enforcement.
pub struct Database {
    /// Path to the database file.
    path: PathBuf,
    /// `SQLite` connection.
    connection: rusqlite::Connection,
}

impl Database {
    /// Opens or creates the database at the default application data directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The directory cannot be created
    /// - `SQLite` connection fails
    /// - Foreign key pragma fails
    pub fn open() -> Result<Self, DatabaseError> {
        let data_dir = app_data_dir();
        std::fs::create_dir_all(&data_dir)?;

        let path = data_dir.join(DATABASE_FILENAME);
        Self::open_at(path)
    }

    /// Opens or creates the database at a specific path.
    ///
    /// # Errors
    ///
    /// Returns an error if `SQLite` connection or pragma setup fails.
    pub fn open_at(path: PathBuf) -> Result<Self, DatabaseError> {
        let connection = rusqlite::Connection::open(&path)?;

        // Enable foreign key enforcement
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;

        // Enable WAL mode for better concurrent access
        connection.execute_batch("PRAGMA journal_mode = WAL;")?;

        Ok(Self { path, connection })
    }

    /// Opens an in-memory database for testing.
    ///
    /// # Errors
    ///
    /// Returns an error if `SQLite` connection fails.
    pub fn open_in_memory() -> Result<Self, DatabaseError> {
        let connection = rusqlite::Connection::open_in_memory()?;
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;

        Ok(Self {
            path: PathBuf::from(":memory:"),
            connection,
        })
    }

    /// Returns the path to the database file.
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Returns a reference to the underlying `SQLite` connection.
    #[must_use]
    pub fn connection(&self) -> &rusqlite::Connection {
        &self.connection
    }

    /// Returns a mutable reference to the underlying `SQLite` connection.
    pub fn connection_mut(&mut self) -> &mut rusqlite::Connection {
        &mut self.connection
    }

    /// Runs all pending migrations.
    ///
    /// # Errors
    ///
    /// Returns an error if any migration fails.
    pub fn run_migrations(&mut self) -> Result<Vec<String>, DatabaseError> {
        let runner = MigrationRunner::new();
        let applied = runner.run_all(&mut self.connection)?;
        Ok(applied)
    }

    /// Checks if foreign keys are enabled.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn foreign_keys_enabled(&self) -> Result<bool, DatabaseError> {
        let enabled: i32 = self
            .connection
            .query_row("PRAGMA foreign_keys;", [], |row| row.get(0))?;
        Ok(enabled == 1)
    }

    /// Executes a raw SQL statement.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails.
    pub fn execute(&self, sql: &str) -> Result<usize, DatabaseError> {
        Ok(self.connection.execute(sql, [])?)
    }

    /// Executes a batch of SQL statements.
    ///
    /// # Errors
    ///
    /// Returns an error if execution fails.
    pub fn execute_batch(&self, sql: &str) -> Result<(), DatabaseError> {
        Ok(self.connection.execute_batch(sql)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_in_memory() {
        let db = Database::open_in_memory();
        assert!(db.is_ok());
    }

    #[test]
    fn test_foreign_keys_enabled() {
        let db = Database::open_in_memory().unwrap();
        assert!(db.foreign_keys_enabled().unwrap());
    }

    #[test]
    fn test_execute_basic_sql() {
        let db = Database::open_in_memory().unwrap();
        let result = db.execute("CREATE TABLE test (id INTEGER PRIMARY KEY)");
        assert!(result.is_ok());
    }

    #[test]
    fn test_foreign_key_enforcement() {
        let db = Database::open_in_memory().unwrap();

        // Create parent table
        db.execute_batch(
            "CREATE TABLE parent (id INTEGER PRIMARY KEY);
             CREATE TABLE child (id INTEGER, parent_id INTEGER REFERENCES parent(id));",
        )
        .unwrap();

        // Insert parent
        db.execute("INSERT INTO parent (id) VALUES (1)").unwrap();

        // Insert child with valid FK should work
        let valid = db.execute("INSERT INTO child (id, parent_id) VALUES (1, 1)");
        assert!(valid.is_ok());

        // Insert child with invalid FK should fail
        let invalid = db.execute("INSERT INTO child (id, parent_id) VALUES (2, 999)");
        assert!(invalid.is_err());
    }
}
