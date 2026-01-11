# Velocity Console - Database Schema

## Overview

Velocity Console uses SQLite for local persistence. The database stores:
- Server inventory (Ledger)
- Connection audit history
- File transfer history

Database location: `~/.local/share/velocity-console/velocity.db`

---

## Entity Relationship Diagram

```
┌─────────────────┐       ┌─────────────────┐
│     servers     │───────│   server_tags   │
│                 │  1:n  │                 │
│  id (PK)        │       │  server_id (FK) │
│  name           │       │  tag            │
│  hostname       │       └─────────────────┘
│  port           │
│  username       │       ┌─────────────────┐
│  auth_type      │       │ server_groups   │
│  credential_ref │       │                 │
│  key_path       │       │  id (PK)        │
│  notes          │       │  name           │
│  created_at     │       │  color          │
│  updated_at     │       └────────┬────────┘
│  last_connected │                │
│  connection_cnt │       ┌────────┴────────┐
└────────┬────────┘       │server_group_    │
         │                │    members      │
         │           n:m  │                 │
         ├────────────────│  server_id (FK) │
         │                │  group_id (FK)  │
         │                └─────────────────┘
         │
    ┌────┴────┐
    │         │
┌───┴───┐ ┌───┴────┐
│connect│ │transfer│
│_history│ │_history│
└───────┘ └────────┘
```

---

## Tables

### servers

Primary table storing all managed server entries.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY | UUID v4 |
| name | TEXT | NOT NULL | Display name |
| hostname | TEXT | NOT NULL | FQDN or IP |
| port | INTEGER | DEFAULT 22 | SSH port |
| username | TEXT | NOT NULL | Login user |
| auth_type | TEXT | NOT NULL | 'password', 'key', 'agent' |
| credential_ref | TEXT | NULLABLE | UUID ref to OS keyring |
| key_path | TEXT | NULLABLE | Path to SSH key file |
| notes | TEXT | NULLABLE | User notes |
| created_at | TEXT | NOT NULL | ISO 8601 timestamp |
| updated_at | TEXT | NOT NULL | ISO 8601 timestamp |
| last_connected | TEXT | NULLABLE | ISO 8601 timestamp |
| connection_count | INTEGER | DEFAULT 0 | Total connections |

**Indexes:**
- `idx_servers_hostname` on `hostname`
- `idx_servers_last_connected` on `last_connected`

---

### server_tags

Tags for filtering and categorization.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| server_id | TEXT | NOT NULL, FK | References servers(id) |
| tag | TEXT | NOT NULL | Tag name |

**Primary Key:** (server_id, tag)
**On Delete:** CASCADE

---

### server_groups

Named groups for organizing servers.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY | UUID v4 |
| name | TEXT | NOT NULL, UNIQUE | Group name |
| color | TEXT | NULLABLE | Hex color code |

---

### server_group_members

Junction table for servers-to-groups many-to-many relationship.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| server_id | TEXT | NOT NULL, FK | References servers(id) |
| group_id | TEXT | NOT NULL, FK | References server_groups(id) |

**Primary Key:** (server_id, group_id)
**On Delete:** CASCADE (both FKs)

---

### connection_history

Audit log of all SSH connection attempts.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY | UUID v4 |
| server_id | TEXT | NOT NULL, FK | References servers(id) |
| connected_at | TEXT | NOT NULL | ISO 8601 timestamp |
| disconnected_at | TEXT | NULLABLE | ISO 8601 timestamp |
| duration_secs | INTEGER | NULLABLE | Session duration |
| status | TEXT | NOT NULL | 'success', 'failed', 'timeout' |
| error_msg | TEXT | NULLABLE | Error details if failed |

**Index:** `idx_connection_history_server` on `server_id`
**On Delete:** CASCADE

---

### transfer_history

Audit log of all file transfers.

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | TEXT | PRIMARY KEY | UUID v4 |
| server_id | TEXT | NOT NULL, FK | References servers(id) |
| direction | TEXT | NOT NULL | 'upload', 'download', 's2s' |
| local_path | TEXT | NULLABLE | Local file path |
| remote_path | TEXT | NOT NULL | Remote file path |
| bytes | INTEGER | NOT NULL | Transfer size |
| started_at | TEXT | NOT NULL | ISO 8601 timestamp |
| completed_at | TEXT | NULLABLE | ISO 8601 timestamp |
| status | TEXT | NOT NULL | 'completed', 'failed', 'cancelled' |

**Index:** `idx_transfer_history_server` on `server_id`
**On Delete:** CASCADE

---

## Constraints

### Data Integrity
- All foreign keys use ON DELETE CASCADE
- Foreign keys are enforced via `PRAGMA foreign_keys = ON`
- UUIDs are validated at application layer

### Authentication Types
- `password`: credential_ref must be set (points to keyring)
- `key`: key_path must be set
- `agent`: neither required (uses SSH agent)

---

## Migration Strategy

Migrations are tracked in a `_migrations` table:

```sql
CREATE TABLE IF NOT EXISTS _migrations (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    applied_at TEXT NOT NULL
);
```

Migration files are numbered: `001_initial_schema.sql`, `002_add_feature.sql`, etc.
