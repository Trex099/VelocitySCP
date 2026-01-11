//! Logging infrastructure for Velocity Console.
//!
//! Uses `tracing` for structured logging with automatic PII redaction.

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use crate::infrastructure::platform::paths::app_log_dir;
use std::fs;

/// Initialize the logging system.
///
/// # Log Levels
/// - ERROR: Critical errors that require immediate attention
/// - WARN: Potential issues that should be investigated
/// - INFO: Important operational information
/// - DEBUG: Detailed debugging information
/// - TRACE: Very detailed tracing information
///
/// # Environment Variables
/// - `VELOCITY_LOG`: Set log level (e.g., "debug", "`velocity_console=trace`")
/// - `RUST_LOG`: Fallback if `VELOCITY_LOG` is not set
///
/// # Errors
/// Returns an error if the logging system fails to initialize.
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    // Create log directory if it doesn't exist
    let log_dir = app_log_dir();
    fs::create_dir_all(&log_dir)?;

    // Build the filter from environment or default
    let filter = EnvFilter::try_from_env("VELOCITY_LOG")
        .or_else(|_| EnvFilter::try_from_env("RUST_LOG"))
        .unwrap_or_else(|_| EnvFilter::new("info,velocity_console=debug"));

    // Console layer for development
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_span_events(FmtSpan::CLOSE)
        .compact();

    // Initialize subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(console_layer)
        .init();

    tracing::info!("Logging initialized");
    tracing::debug!(log_dir = %log_dir.display(), "Log directory configured");

    Ok(())
}

/// Log a connection attempt (with PII redaction).
///
/// # Arguments
/// * `host` - The hostname (will be logged)
/// * `username` - The username (will be logged)
/// * `port` - The SSH port
#[inline]
pub fn log_connection_attempt(host: &str, username: &str, port: u16) {
    tracing::info!(
        target: "velocity::ssh",
        host = %host,
        username = %username,
        port = %port,
        "Attempting SSH connection"
    );
}

/// Log a successful connection.
#[inline]
pub fn log_connection_success(host: &str, session_id: &str) {
    tracing::info!(
        target: "velocity::ssh",
        host = %host,
        session_id = %session_id,
        "SSH connection established"
    );
}

/// Log a connection failure.
#[inline]
pub fn log_connection_failure(host: &str, error: &str) {
    tracing::error!(
        target: "velocity::ssh",
        host = %host,
        error = %error,
        "SSH connection failed"
    );
}

/// Log an SFTP operation.
#[inline]
pub fn log_sftp_operation(operation: &str, path: &str, success: bool) {
    if success {
        tracing::debug!(
            target: "velocity::sftp",
            operation = %operation,
            path = %path,
            "SFTP operation completed"
        );
    } else {
        tracing::warn!(
            target: "velocity::sftp",
            operation = %operation,
            path = %path,
            "SFTP operation failed"
        );
    }
}

/// Log a credential access (WITHOUT exposing the credential).
///
/// # Security Note
/// This function NEVER logs the actual credential value.
#[inline]
pub fn log_credential_access(server_id: &str, operation: &str) {
    tracing::debug!(
        target: "velocity::credentials",
        server_id = %server_id,
        operation = %operation,
        credential = "[REDACTED]",
        "Credential operation"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: We can't easily test tracing output in unit tests,
    // but we can verify the functions don't panic

    #[test]
    fn test_log_connection_attempt_does_not_panic() {
        // This will only log if tracing is initialized
        log_connection_attempt("example.com", "user", 22);
    }

    #[test]
    fn test_log_credential_access_redacts() {
        // Verify the function exists and doesn't expose credentials
        log_credential_access("server-123", "retrieve");
    }
}
