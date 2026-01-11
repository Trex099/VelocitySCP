//! Velocity Console - Ledger-First System Management Console
//!
//! A high-performance SSH/SFTP client with session persistence,
//! server-to-server streaming, and WinSCP-style dual-pane interface.
//!
//! # Modules
//! - [`security`]: Secure string handling and input validation
//! - [`error`]: Error types for all operations
//! - [`logging`]: Structured logging with PII redaction
//! - [`infrastructure`]: Platform abstractions and external integrations
//!
//! # Feature Flags
//! - `tauri-app`: Full desktop application with Tauri GUI (default)
//! - `headless`: Core library only, for CI testing without GUI deps

pub mod error;
pub mod infrastructure;
pub mod logging;
pub mod security;

use tracing::info;

/// Initialize the application infrastructure.
///
/// Sets up logging, platform detection, and other foundational services.
/// This should be called before the Tauri builder.
pub fn init_infrastructure() {
    // Initialize logging - warn on failure but don't crash
    if let Err(e) = logging::init_logging() {
        eprintln!("Warning: Failed to initialize logging: {e}");
        eprintln!("Continuing without structured logging...");
    }

    // Log platform information
    if let Some(terminal) = infrastructure::platform::detect_terminal() {
        info!(
            target: "velocity::platform",
            terminal = ?terminal,
            "Detected terminal emulator"
        );
    } else {
        info!(
            target: "velocity::platform",
            "No supported terminal emulator detected"
        );
    }

    info!(
        target: "velocity::app",
        version = env!("CARGO_PKG_VERSION"),
        "Velocity Console starting"
    );
}

// ============================================================================
// Tauri Application (only compiled when tauri-app feature is enabled)
// ============================================================================

#[cfg(feature = "tauri-app")]
mod tauri_app {
    use tracing::{error, info};

    /// Greet a user by name.
    ///
    /// # Arguments
    /// * `name` - The name to greet
    ///
    /// # Returns
    /// A greeting string
    #[tauri::command]
    pub fn greet(name: &str) -> String {
        info!(target: "velocity::commands", name = %name, "Greet command invoked");
        format!("Hello, {name}! You've been greeted from Rust!")
    }

    /// Run the Tauri application.
    ///
    /// # Panics
    /// Panics if the Tauri application fails to initialize or run.
    #[cfg_attr(mobile, tauri::mobile_entry_point)]
    pub fn run() {
        // Initialize infrastructure before Tauri
        super::init_infrastructure();

        let result = tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![greet])
            .run(tauri::generate_context!());

        match result {
            Ok(()) => info!(target: "velocity::app", "Application exited normally"),
            Err(e) => {
                error!(target: "velocity::app", error = %e, "Application failed to run");
                panic!("error while running tauri application: {e}");
            }
        }
    }
}

#[cfg(feature = "tauri-app")]
pub use tauri_app::run;
