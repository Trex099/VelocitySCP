//! Platform abstraction layer for cross-distro compatibility.
//!
//! Velocity Console supports multiple Linux distributions:
//! - Fedora / RHEL
//! - Ubuntu / Debian
//! - Arch Linux
//!
//! This module abstracts distro-specific paths and tools.

pub mod paths;
pub mod terminal;

pub use paths::{app_cache_dir, app_config_dir, app_data_dir};
pub use terminal::{detect_terminal, TerminalEmulator};
