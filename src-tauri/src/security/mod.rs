//! Security infrastructure for Velocity Console
//!
//! This module provides security primitives including:
//! - `SecretString`: A wrapper type that zeroizes memory on drop and prevents accidental logging
//! - Input validation functions for hostnames, paths, and ports
//! - Logging sanitization layer for redacting sensitive data

pub mod secret;
pub mod validation;

pub use secret::SecretString;
pub use validation::{validate_hostname, validate_port, validate_remote_path, ValidationError};
