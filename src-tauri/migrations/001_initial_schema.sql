-- Velocity Console Initial Schema
-- Migration: 001_initial_schema
-- Created: 2026-01-11

-- Enable foreign key enforcement
PRAGMA foreign_keys = ON;

-- ============================================================================
-- Migration Tracking
-- ============================================================================

CREATE TABLE IF NOT EXISTS _migrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    applied_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ============================================================================
-- Core Tables
-- ============================================================================

-- Primary server registry (Ledger)
CREATE TABLE IF NOT EXISTS servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    hostname TEXT NOT NULL,
    port INTEGER NOT NULL DEFAULT 22,
    username TEXT NOT NULL,
    auth_type TEXT NOT NULL CHECK (auth_type IN ('password', 'key', 'agent')),
    credential_ref TEXT,
    key_path TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_connected TEXT,
    connection_count INTEGER NOT NULL DEFAULT 0
);

-- Server tags for filtering
CREATE TABLE IF NOT EXISTS server_tags (
    server_id TEXT NOT NULL,
    tag TEXT NOT NULL,
    PRIMARY KEY (server_id, tag),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

-- Server groups for organization
CREATE TABLE IF NOT EXISTS server_groups (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    color TEXT
);

-- Many-to-many junction: servers <-> groups
CREATE TABLE IF NOT EXISTS server_group_members (
    server_id TEXT NOT NULL,
    group_id TEXT NOT NULL,
    PRIMARY KEY (server_id, group_id),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES server_groups(id) ON DELETE CASCADE
);

-- Connection audit log
CREATE TABLE IF NOT EXISTS connection_history (
    id TEXT PRIMARY KEY,
    server_id TEXT NOT NULL,
    connected_at TEXT NOT NULL DEFAULT (datetime('now')),
    disconnected_at TEXT,
    duration_secs INTEGER,
    status TEXT NOT NULL CHECK (status IN ('success', 'failed', 'timeout')),
    error_msg TEXT,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

-- File transfer audit log
CREATE TABLE IF NOT EXISTS transfer_history (
    id TEXT PRIMARY KEY,
    server_id TEXT NOT NULL,
    direction TEXT NOT NULL CHECK (direction IN ('upload', 'download', 's2s')),
    local_path TEXT,
    remote_path TEXT NOT NULL,
    bytes INTEGER NOT NULL,
    started_at TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    status TEXT NOT NULL CHECK (status IN ('completed', 'failed', 'cancelled')),
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE CASCADE
);

-- ============================================================================
-- Performance Indexes
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_servers_hostname ON servers(hostname);
CREATE INDEX IF NOT EXISTS idx_servers_last_connected ON servers(last_connected);
CREATE INDEX IF NOT EXISTS idx_servers_name ON servers(name);
CREATE INDEX IF NOT EXISTS idx_connection_history_server ON connection_history(server_id);
CREATE INDEX IF NOT EXISTS idx_connection_history_connected ON connection_history(connected_at);
CREATE INDEX IF NOT EXISTS idx_transfer_history_server ON transfer_history(server_id);
CREATE INDEX IF NOT EXISTS idx_transfer_history_started ON transfer_history(started_at);

-- ============================================================================
-- Record this migration as applied
-- ============================================================================

INSERT OR IGNORE INTO _migrations (name, applied_at) VALUES ('001_initial_schema', datetime('now'));
