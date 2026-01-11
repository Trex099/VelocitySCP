//! Secure secret wrapper type that zeroizes memory on drop.
//!
//! # Security Invariant
//! - Secret values are NEVER exposed in Debug or Display output
//! - Memory is securely zeroed when the value is dropped
//! - Only explicit `expose_secret()` reveals the inner value

use zeroize::{Zeroize, ZeroizeOnDrop};

/// A wrapper type for secret strings that:
/// 1. Zeroes memory on drop (prevents memory scraping attacks)
/// 2. Redacts content in Debug/Display (prevents accidental logging)
/// 3. Requires explicit action to access the inner value
///
/// # Example
/// ```
/// use velocity_console_lib::security::SecretString;
///
/// let secret = SecretString::new("my-password".to_string());
/// println!("{:?}", secret); // Prints: SecretString([REDACTED])
///
/// // Only explicit access reveals the value
/// let password = secret.expose_secret();
/// ```
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecretString(String);

impl SecretString {
    /// Create a new `SecretString` from a String value.
    /// The value will be securely zeroed when this wrapper is dropped.
    #[must_use]
    pub fn new(value: String) -> Self {
        Self(value)
    }

    /// Explicitly expose the secret value.
    ///
    /// # Security Note
    /// Use this sparingly and ensure the returned reference is not logged or persisted.
    #[must_use]
    pub fn expose_secret(&self) -> &str {
        &self.0
    }

    /// Returns the length of the secret string.
    /// This is safe to expose as it doesn't reveal the secret content.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the secret string is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

// Prevent accidental logging of secrets
impl std::fmt::Debug for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecretString([REDACTED])")
    }
}

impl std::fmt::Display for SecretString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

// Implement Clone manually to ensure copies are also protected
impl Clone for SecretString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_string_redacts_debug() {
        let secret = SecretString::new("super-secret-password".to_string());
        let debug_output = format!("{secret:?}");

        assert!(debug_output.contains("[REDACTED]"));
        assert!(!debug_output.contains("super-secret-password"));
    }

    #[test]
    fn test_secret_string_redacts_display() {
        let secret = SecretString::new("super-secret-password".to_string());
        let display_output = format!("{secret}");

        assert_eq!(display_output, "[REDACTED]");
        assert!(!display_output.contains("super-secret-password"));
    }

    #[test]
    fn test_secret_string_exposes_on_request() {
        let secret = SecretString::new("my-password".to_string());

        assert_eq!(secret.expose_secret(), "my-password");
    }

    #[test]
    fn test_secret_string_len() {
        let secret = SecretString::new("12345".to_string());

        assert_eq!(secret.len(), 5);
        assert!(!secret.is_empty());
    }

    #[test]
    fn test_secret_string_empty() {
        let secret = SecretString::new(String::new());

        assert!(secret.is_empty());
        assert_eq!(secret.len(), 0);
    }

    #[test]
    fn test_secret_string_clone() {
        let secret = SecretString::new("cloneable".to_string());
        let cloned = secret.clone();

        assert_eq!(cloned.expose_secret(), "cloneable");
    }
}
