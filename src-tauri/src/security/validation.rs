//! Input validation functions for security-sensitive operations.
//!
//! # Security Invariant
//! All user-provided hostnames, paths, and ports MUST be validated before use
//! to prevent command injection, path traversal, and other attacks.

use thiserror::Error;

/// Errors that can occur during input validation.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    /// The hostname contains invalid or dangerous characters
    #[error("Invalid hostname: {0}")]
    InvalidHostname(String),

    /// The path contains traversal sequences (e.g., "..")
    #[error("Path traversal attempt detected: {0}")]
    PathTraversal(String),

    /// The port number is invalid (0 or out of range)
    #[error("Invalid port number: {0}")]
    InvalidPort(u16),

    /// The input contains null bytes
    #[error("Input contains null bytes")]
    NullByte,

    /// The input is empty when it shouldn't be
    #[error("Input cannot be empty")]
    EmptyInput,
}

/// Characters that could enable shell command injection.
const SHELL_METACHARACTERS: &[char] = &[
    ';', '|', '&', '$', '`', '(', ')', '{', '}', '[', ']', '<', '>', '!', '\\', '\n', '\r', '\0',
];

/// Validate a hostname for SSH connection.
///
/// # Security Rules
/// - No shell metacharacters (prevents command injection)
/// - No path separators (prevents path traversal)
/// - No whitespace except internal spaces in quoted strings
/// - Must not be empty
///
/// # Errors
/// Returns `ValidationError` if:
/// - The hostname is empty (`EmptyInput`)
/// - Contains null bytes (`NullByte`)
/// - Contains path separators (`PathTraversal`)
/// - Contains shell metacharacters (`InvalidHostname`)
/// - Has leading/trailing whitespace (`InvalidHostname`)
///
/// # Examples
/// ```
/// use velocity_console_lib::security::validate_hostname;
///
/// assert!(validate_hostname("example.com").is_ok());
/// assert!(validate_hostname("192.168.1.1").is_ok());
/// assert!(validate_hostname("server; rm -rf /").is_err());
/// ```
pub fn validate_hostname(host: &str) -> Result<(), ValidationError> {
    // Check for empty input
    if host.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    // Check for null bytes
    if host.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    // Check for path separators (prevents treating hostname as path)
    if host.contains('/') || host.contains('\\') {
        return Err(ValidationError::PathTraversal(host.to_string()));
    }

    // Check for shell metacharacters (prevents command injection)
    for c in SHELL_METACHARACTERS {
        if host.contains(*c) {
            return Err(ValidationError::InvalidHostname(format!(
                "contains dangerous character '{}'",
                c.escape_default()
            )));
        }
    }

    // Check for leading/trailing whitespace
    if host != host.trim() {
        return Err(ValidationError::InvalidHostname(
            "contains leading or trailing whitespace".to_string(),
        ));
    }

    Ok(())
}

/// Validate a remote path for SFTP operations.
///
/// # Security Rules
/// - No path traversal sequences (..)
/// - No null bytes
/// - Must be non-empty
///
/// # Errors
/// Returns `ValidationError` if:
/// - The path is empty (`EmptyInput`)
/// - Contains null bytes (`NullByte`)
/// - Contains path traversal sequences like `..` (`PathTraversal`)
///
/// # Examples
/// ```
/// use velocity_console_lib::security::validate_remote_path;
///
/// assert!(validate_remote_path("/home/user/file.txt").is_ok());
/// assert!(validate_remote_path("../../../etc/passwd").is_err());
/// ```
pub fn validate_remote_path(path: &str) -> Result<(), ValidationError> {
    // Check for empty input
    if path.is_empty() {
        return Err(ValidationError::EmptyInput);
    }

    // Check for null bytes
    if path.contains('\0') {
        return Err(ValidationError::NullByte);
    }

    // Check for path traversal
    // Must handle various forms: "..", "/../", "\..\", etc.
    let normalized = path.replace('\\', "/");
    let segments: Vec<&str> = normalized.split('/').collect();

    for segment in segments {
        if segment == ".." {
            return Err(ValidationError::PathTraversal(path.to_string()));
        }
    }

    Ok(())
}

/// Validate a port number for SSH connection.
///
/// # Security Rules
/// - Port 0 is reserved and invalid
/// - Must be in valid TCP range (1-65535)
///
/// # Errors
/// Returns `ValidationError::InvalidPort` if the port is 0.
///
/// # Examples
/// ```
/// use velocity_console_lib::security::validate_port;
///
/// assert!(validate_port(22).is_ok());
/// assert!(validate_port(0).is_err());
/// ```
pub fn validate_port(port: u16) -> Result<(), ValidationError> {
    if port == 0 {
        return Err(ValidationError::InvalidPort(port));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Hostname Validation Tests ====================

    #[test]
    fn test_valid_hostnames() {
        assert!(validate_hostname("example.com").is_ok());
        assert!(validate_hostname("sub.example.com").is_ok());
        assert!(validate_hostname("192.168.1.1").is_ok());
        assert!(validate_hostname("10.0.0.1").is_ok());
        assert!(validate_hostname("localhost").is_ok());
        assert!(validate_hostname("my-server-01").is_ok());
        assert!(validate_hostname("server_name").is_ok());
    }

    #[test]
    fn test_hostname_rejects_empty() {
        assert_eq!(validate_hostname(""), Err(ValidationError::EmptyInput));
    }

    #[test]
    fn test_hostname_rejects_null_bytes() {
        assert_eq!(
            validate_hostname("example\0.com"),
            Err(ValidationError::NullByte)
        );
    }

    #[test]
    fn test_hostname_rejects_path_separators() {
        assert!(matches!(
            validate_hostname("example.com/path"),
            Err(ValidationError::PathTraversal(_))
        ));
        assert!(matches!(
            validate_hostname("example.com\\path"),
            Err(ValidationError::PathTraversal(_))
        ));
    }

    #[test]
    fn test_hostname_rejects_command_injection() {
        // Semicolon injection (no path separators to avoid PathTraversal)
        assert!(matches!(
            validate_hostname("example.com;rm"),
            Err(ValidationError::InvalidHostname(_))
        ));

        // Pipe injection
        assert!(matches!(
            validate_hostname("host|cat"),
            Err(ValidationError::InvalidHostname(_))
        ));

        // Backtick injection
        assert!(matches!(
            validate_hostname("host`whoami`"),
            Err(ValidationError::InvalidHostname(_))
        ));

        // Dollar substitution (parens)
        assert!(matches!(
            validate_hostname("host$(whoami)"),
            Err(ValidationError::InvalidHostname(_))
        ));

        // Ampersand
        assert!(matches!(
            validate_hostname("host&&malicious"),
            Err(ValidationError::InvalidHostname(_))
        ));

        // Newline injection
        assert!(matches!(
            validate_hostname("host\nmalicious"),
            Err(ValidationError::InvalidHostname(_))
        ));
    }

    #[test]
    fn test_hostname_rejects_whitespace_padding() {
        assert!(matches!(
            validate_hostname(" example.com"),
            Err(ValidationError::InvalidHostname(_))
        ));
        assert!(matches!(
            validate_hostname("example.com "),
            Err(ValidationError::InvalidHostname(_))
        ));
    }

    // ==================== Path Validation Tests ====================

    #[test]
    fn test_valid_paths() {
        assert!(validate_remote_path("/home/user/file.txt").is_ok());
        assert!(validate_remote_path("/var/log/syslog").is_ok());
        assert!(validate_remote_path("relative/path/file.txt").is_ok());
        assert!(validate_remote_path("/").is_ok());
        assert!(validate_remote_path("./current/dir").is_ok());
    }

    #[test]
    fn test_path_rejects_empty() {
        assert_eq!(validate_remote_path(""), Err(ValidationError::EmptyInput));
    }

    #[test]
    fn test_path_rejects_null_bytes() {
        assert_eq!(
            validate_remote_path("/home\0/user"),
            Err(ValidationError::NullByte)
        );
    }

    #[test]
    fn test_path_rejects_traversal() {
        assert!(matches!(
            validate_remote_path("../../../etc/passwd"),
            Err(ValidationError::PathTraversal(_))
        ));
        assert!(matches!(
            validate_remote_path("/home/user/../../../etc/passwd"),
            Err(ValidationError::PathTraversal(_))
        ));
        assert!(matches!(
            validate_remote_path(".."),
            Err(ValidationError::PathTraversal(_))
        ));
        // Windows-style
        assert!(matches!(
            validate_remote_path("..\\..\\etc\\passwd"),
            Err(ValidationError::PathTraversal(_))
        ));
    }

    // ==================== Port Validation Tests ====================

    #[test]
    fn test_valid_ports() {
        assert!(validate_port(22).is_ok());
        assert!(validate_port(1).is_ok());
        assert!(validate_port(443).is_ok());
        assert!(validate_port(8080).is_ok());
        assert!(validate_port(65535).is_ok());
    }

    #[test]
    fn test_port_rejects_zero() {
        assert_eq!(validate_port(0), Err(ValidationError::InvalidPort(0)));
    }
}
