# Velocity Console

A high-performance SSH/SFTP client built with Rust and Tauri, designed for DevOps engineers managing multiple servers.

## Features

- **Ledger-First Architecture** - Your server inventory is home base, not a temporary connection
- **Zero-Copy File Streaming** - Server-to-server transfers without touching your disk
- **Security-First Design** - OS keyring integration, no plaintext passwords
- **Dual-Pane Interface** - WinSCP-style file browsing
- **Edit-on-Save** - Open remote files in your editor, auto-sync on save
- **Terminal Integration** - Launch SSH sessions in your preferred terminal

## Tech Stack

| Layer | Technology |
|-------|------------|
| Backend | Rust, Tokio, russh, russh-sftp |
| Frontend | React, TypeScript, TailwindCSS |
| Framework | Tauri v2 |
| State | TanStack Query, Zustand |
| Database | SQLite (via tauri-plugin-sql) |
| Credentials | OS Keyring (Secret Service on Linux) |

## Development

### Prerequisites

- Rust 1.75+
- Node.js 20+
- System dependencies (Ubuntu/Debian):
  ```bash
  sudo apt install libwebkit2gtk-4.1-dev libssl-dev libdbus-1-dev librsvg2-dev
  ```

### Quick Start

```bash
npm install
npm run tauri dev
```

### Build

```bash
npm run tauri build
```

### Testing

```bash
# Rust
cd src-tauri && cargo test --lib

# Frontend
npm run lint
npm run format:check
```

## Project Structure

```
velocity-console/
├── src/                    # React frontend
├── src-tauri/              # Rust backend
│   ├── src/
│   │   ├── security/       # SecretString, validation
│   │   ├── error.rs        # Error types
│   │   ├── logging.rs      # Structured logging
│   │   └── infrastructure/ # Platform abstractions
│   └── tests/
└── docs/
```

## License

MIT OR Apache-2.0
