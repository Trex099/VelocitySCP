//! Error handling for Velocity Console.
//!
//! Provides a unified error type hierarchy for all operations.

use thiserror::Error;

/// Top-level application errors.
#[derive(Error, Debug)]
pub enum VelocityError {
    /// SSH connection related errors
    #[error("SSH error: {0}")]
    Ssh(#[from] SshError),

    /// SFTP operation errors
    #[error("SFTP error: {0}")]
    Sftp(#[from] SftpError),

    /// Credential storage errors
    #[error("Credential error: {0}")]
    Credential(#[from] CredentialError),

    /// Database errors
    #[error("Database error: {0}")]
    Database(#[from] DatabaseError),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(#[from] crate::security::ValidationError),

    /// Platform/system errors
    #[error("Platform error: {0}")]
    Platform(String),

    /// Generic I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// SSH-specific errors.
#[derive(Error, Debug)]
pub enum SshError {
    /// Failed to establish connection
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Host key verification failed
    #[error("Host key verification failed for {host}")]
    HostKeyVerificationFailed { host: String },

    /// Connection timeout
    #[error("Connection timed out after {seconds}s")]
    Timeout { seconds: u64 },

    /// Channel operation failed
    #[error("Channel error: {0}")]
    ChannelError(String),

    /// Session already closed
    #[error("Session closed")]
    SessionClosed,
}

/// SFTP-specific errors.
#[derive(Error, Debug)]
pub enum SftpError {
    /// File not found
    #[error("File not found: {path}")]
    NotFound { path: String },

    /// Permission denied
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },

    /// Transfer failed
    #[error("Transfer failed: {0}")]
    TransferFailed(String),

    /// Invalid path
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Disk full
    #[error("Disk full on remote server")]
    DiskFull,
}

/// Credential storage errors.
#[derive(Error, Debug)]
pub enum CredentialError {
    /// Keyring not available
    #[error("Keyring service not available")]
    KeyringUnavailable,

    /// Credential not found
    #[error("Credential not found for {server_id}")]
    NotFound { server_id: String },

    /// Failed to store credential
    #[error("Failed to store credential: {0}")]
    StoreFailed(String),

    /// Failed to retrieve credential
    #[error("Failed to retrieve credential: {0}")]
    RetrieveFailed(String),

    /// Failed to delete credential
    #[error("Failed to delete credential: {0}")]
    DeleteFailed(String),
}

/// Database errors.
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// Connection failed
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    /// Query failed
    #[error("Query failed: {0}")]
    QueryFailed(String),

    /// Migration failed
    #[error("Migration failed: {0}")]
    MigrationFailed(String),

    /// Record not found
    #[error("Record not found: {0}")]
    NotFound(String),

    /// Constraint violation
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
}

/// Result type alias for Velocity operations.
pub type Result<T> = std::result::Result<T, VelocityError>;

/// Result type alias for SSH operations.
pub type SshResult<T> = std::result::Result<T, SshError>;

/// Result type alias for SFTP operations.
pub type SftpResult<T> = std::result::Result<T, SftpError>;

/// Result type alias for credential operations.
pub type CredentialResult<T> = std::result::Result<T, CredentialError>;

/// Result type alias for database operations.
pub type DatabaseResult<T> = std::result::Result<T, DatabaseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SshError::ConnectionFailed("network unreachable".to_string());
        assert!(err.to_string().contains("Connection failed"));
        assert!(err.to_string().contains("network unreachable"));
    }

    #[test]
    fn test_error_conversion() {
        let ssh_err = SshError::Timeout { seconds: 30 };
        let velocity_err: VelocityError = ssh_err.into();
        assert!(velocity_err.to_string().contains("SSH error"));
    }

    #[test]
    fn test_sftp_error_with_path() {
        let err = SftpError::NotFound {
            path: "/home/user/missing.txt".to_string(),
        };
        assert!(err.to_string().contains("/home/user/missing.txt"));
    }

    #[test]
    fn test_credential_error_with_server_id() {
        let err = CredentialError::NotFound {
            server_id: "server-123".to_string(),
        };
        assert!(err.to_string().contains("server-123"));
    }
}
