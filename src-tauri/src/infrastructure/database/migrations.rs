//! Database migration system.
//!
//! Runs SQL migrations in order and tracks which have been applied.

use std::collections::HashSet;

use thiserror::Error;

use super::models::MigrationRecord;

/// Errors that can occur during migrations.
#[derive(Debug, Error)]
pub enum MigrationError {
    /// `SQLite` error during migration.
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    /// Migration file not found or invalid.
    #[error("migration '{0}' failed: {1}")]
    Failed(String, String),
}

/// A database migration definition.
#[derive(Debug, Clone)]
pub struct Migration {
    /// Migration name (e.g., `001_initial_schema`).
    pub name: &'static str,
    /// SQL to execute.
    pub sql: &'static str,
}

/// Runs database migrations.
pub struct MigrationRunner {
    migrations: Vec<(&'static str, &'static str)>,
}

impl MigrationRunner {
    /// Creates a new migration runner with all known migrations.
    #[must_use]
    pub fn new() -> Self {
        Self {
            migrations: vec![(
                "001_initial_schema",
                include_str!("../../../migrations/001_initial_schema.sql"),
            )],
        }
    }

    /// Ensures the migrations tracking table exists.
    fn ensure_migrations_table(conn: &rusqlite::Connection) -> Result<(), MigrationError> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS _migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;
        Ok(())
    }

    /// Gets the set of already-applied migration names.
    fn get_applied_migrations(
        conn: &rusqlite::Connection,
    ) -> Result<HashSet<String>, MigrationError> {
        let mut stmt = conn.prepare("SELECT name FROM _migrations")?;
        let names = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<HashSet<_>, _>>()?;
        Ok(names)
    }

    /// Runs all pending migrations.
    ///
    /// # Returns
    ///
    /// A list of migration names that were applied.
    ///
    /// # Errors
    ///
    /// Returns an error if any migration fails.
    pub fn run_all(&self, conn: &mut rusqlite::Connection) -> Result<Vec<String>, MigrationError> {
        Self::ensure_migrations_table(conn)?;
        let applied = Self::get_applied_migrations(conn)?;

        let mut newly_applied = Vec::new();

        for (name, sql) in &self.migrations {
            if applied.contains(*name) {
                continue;
            }

            // Run migration in a transaction
            let tx = conn.transaction()?;

            // Execute the migration SQL
            // Note: We don't use the INSERT in the SQL file since we track separately
            tx.execute_batch(sql)
                .map_err(|e| MigrationError::Failed((*name).to_string(), e.to_string()))?;

            // Record as applied (if not already recorded by the SQL itself)
            let count: i32 = tx.query_row(
                "SELECT COUNT(*) FROM _migrations WHERE name = ?1",
                [*name],
                |row| row.get(0),
            )?;

            if count == 0 {
                tx.execute(
                    "INSERT INTO _migrations (name, applied_at) VALUES (?1, datetime('now'))",
                    [*name],
                )?;
            }

            tx.commit()?;
            newly_applied.push((*name).to_string());
        }

        Ok(newly_applied)
    }

    /// Gets all migration records from the database.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub fn get_migration_history(
        &self,
        conn: &rusqlite::Connection,
    ) -> Result<Vec<MigrationRecord>, MigrationError> {
        Self::ensure_migrations_table(conn)?;

        let mut stmt = conn.prepare("SELECT id, name, applied_at FROM _migrations ORDER BY id")?;

        let records = stmt
            .query_map([], |row| {
                Ok(MigrationRecord {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    applied_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(records)
    }
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_connection() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch("PRAGMA foreign_keys = ON;").unwrap();
        conn
    }

    #[test]
    fn test_ensure_migrations_table() {
        let conn = create_test_connection();

        let result = MigrationRunner::ensure_migrations_table(&conn);
        assert!(result.is_ok());

        // Verify table exists
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='_migrations'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_run_all_migrations() {
        let mut conn = create_test_connection();
        let runner = MigrationRunner::new();

        let applied = runner.run_all(&mut conn).unwrap();
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0], "001_initial_schema");

        // Verify tables were created
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"servers".to_string()));
        assert!(tables.contains(&"server_tags".to_string()));
        assert!(tables.contains(&"server_groups".to_string()));
        assert!(tables.contains(&"server_group_members".to_string()));
        assert!(tables.contains(&"connection_history".to_string()));
        assert!(tables.contains(&"transfer_history".to_string()));
    }

    #[test]
    fn test_migrations_are_idempotent() {
        let mut conn = create_test_connection();
        let runner = MigrationRunner::new();

        // Run once
        let first = runner.run_all(&mut conn).unwrap();
        assert_eq!(first.len(), 1);

        // Run again - should skip already applied
        let second = runner.run_all(&mut conn).unwrap();
        assert_eq!(second.len(), 0);
    }

    #[test]
    fn test_get_migration_history() {
        let mut conn = create_test_connection();
        let runner = MigrationRunner::new();

        runner.run_all(&mut conn).unwrap();

        let history = runner.get_migration_history(&conn).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].name, "001_initial_schema");
        assert!(!history[0].applied_at.is_empty());
    }

    #[test]
    fn test_all_tables_have_correct_structure() {
        let mut conn = create_test_connection();
        let runner = MigrationRunner::new();
        runner.run_all(&mut conn).unwrap();

        // Test servers table structure
        let server_columns: Vec<String> = conn
            .prepare("PRAGMA table_info(servers)")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(server_columns.contains(&"id".to_string()));
        assert!(server_columns.contains(&"name".to_string()));
        assert!(server_columns.contains(&"hostname".to_string()));
        assert!(server_columns.contains(&"port".to_string()));
        assert!(server_columns.contains(&"username".to_string()));
        assert!(server_columns.contains(&"auth_type".to_string()));
        assert!(server_columns.contains(&"credential_ref".to_string()));
        assert!(server_columns.contains(&"key_path".to_string()));
        assert!(server_columns.contains(&"notes".to_string()));
        assert!(server_columns.contains(&"created_at".to_string()));
        assert!(server_columns.contains(&"updated_at".to_string()));
        assert!(server_columns.contains(&"last_connected".to_string()));
        assert!(server_columns.contains(&"connection_count".to_string()));
    }

    #[test]
    fn test_indexes_created() {
        let mut conn = create_test_connection();
        let runner = MigrationRunner::new();
        runner.run_all(&mut conn).unwrap();

        let indexes: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(indexes.contains(&"idx_servers_hostname".to_string()));
        assert!(indexes.contains(&"idx_servers_last_connected".to_string()));
        assert!(indexes.contains(&"idx_connection_history_server".to_string()));
        assert!(indexes.contains(&"idx_transfer_history_server".to_string()));
    }

    #[test]
    fn test_foreign_key_cascade_delete() {
        let mut conn = create_test_connection();
        let runner = MigrationRunner::new();
        runner.run_all(&mut conn).unwrap();

        // Insert a server
        conn.execute(
            "INSERT INTO servers (id, name, hostname, username, auth_type, created_at, updated_at)
             VALUES ('test-uuid', 'Test', 'test.com', 'user', 'password', datetime('now'), datetime('now'))",
            [],
        )
        .unwrap();

        // Insert a tag
        conn.execute(
            "INSERT INTO server_tags (server_id, tag) VALUES ('test-uuid', 'production')",
            [],
        )
        .unwrap();

        // Delete the server
        conn.execute("DELETE FROM servers WHERE id = 'test-uuid'", [])
            .unwrap();

        // Tag should be cascade deleted
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM server_tags WHERE server_id = 'test-uuid'",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 0);
    }
}
