//! Cross-platform path utilities.
//!
//! Uses XDG base directory specification on Linux.

use std::path::PathBuf;

/// Get the application data directory.
///
/// - Linux: `$XDG_DATA_HOME/velocity-console` or `~/.local/share/velocity-console`
///
/// # Panics
/// Panics if the home directory cannot be determined.
#[must_use]
pub fn app_data_dir() -> PathBuf {
    dirs::data_dir()
        .expect("Could not determine data directory")
        .join("velocity-console")
}

/// Get the application config directory.
///
/// - Linux: `$XDG_CONFIG_HOME/velocity-console` or `~/.config/velocity-console`
///
/// # Panics
/// Panics if the home directory cannot be determined.
#[must_use]
pub fn app_config_dir() -> PathBuf {
    dirs::config_dir()
        .expect("Could not determine config directory")
        .join("velocity-console")
}

/// Get the application cache directory.
///
/// - Linux: `$XDG_CACHE_HOME/velocity-console` or `~/.cache/velocity-console`
///
/// # Panics
/// Panics if the home directory cannot be determined.
#[must_use]
pub fn app_cache_dir() -> PathBuf {
    dirs::cache_dir()
        .expect("Could not determine cache directory")
        .join("velocity-console")
}

/// Get the application log directory.
///
/// Logs are stored in the cache directory under `logs/`.
#[must_use]
pub fn app_log_dir() -> PathBuf {
    app_cache_dir().join("logs")
}

/// Get the path for the `SQLite` database.
#[must_use]
pub fn database_path() -> PathBuf {
    app_data_dir().join("ledger.db")
}

/// Get the path for temporary file downloads during edit-on-save.
#[must_use]
pub fn temp_edit_dir() -> PathBuf {
    app_cache_dir().join("edit-temp")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_data_dir_contains_app_name() {
        let path = app_data_dir();
        assert!(path.ends_with("velocity-console"));
    }

    #[test]
    fn test_app_config_dir_contains_app_name() {
        let path = app_config_dir();
        assert!(path.ends_with("velocity-console"));
    }

    #[test]
    fn test_database_path_is_in_data_dir() {
        let db_path = database_path();
        let data_dir = app_data_dir();
        assert!(db_path.starts_with(data_dir));
        assert!(db_path.ends_with("ledger.db"));
    }
}
