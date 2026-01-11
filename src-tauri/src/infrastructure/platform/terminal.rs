//! Terminal emulator detection and launching.
//!
//! Detects the user's preferred terminal emulator and provides
//! a unified interface for launching SSH sessions.

use std::process::Command;
use which::which;

/// Supported terminal emulators in order of preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalEmulator {
    /// Alacritty - GPU-accelerated terminal
    Alacritty,
    /// Kitty - GPU-accelerated terminal
    Kitty,
    /// Ghostty - Modern terminal
    Ghostty,
    /// `WezTerm` - GPU-accelerated terminal
    WezTerm,
    /// GNOME Terminal
    GnomeTerminal,
    /// Konsole (KDE)
    Konsole,
    /// xterm (fallback)
    Xterm,
}

impl TerminalEmulator {
    /// Get the executable name for this terminal.
    #[must_use]
    pub const fn executable(&self) -> &'static str {
        match self {
            Self::Alacritty => "alacritty",
            Self::Kitty => "kitty",
            Self::Ghostty => "ghostty",
            Self::WezTerm => "wezterm",
            Self::GnomeTerminal => "gnome-terminal",
            Self::Konsole => "konsole",
            Self::Xterm => "xterm",
        }
    }

    /// Get the argument style for executing a command.
    #[must_use]
    pub const fn exec_args(&self) -> &'static [&'static str] {
        match self {
            Self::Alacritty | Self::Ghostty | Self::Konsole | Self::Xterm => &["-e"],
            Self::Kitty | Self::GnomeTerminal => &["--"],
            Self::WezTerm => &["start", "--"],
        }
    }

    /// Check if this terminal is available on the system.
    #[must_use]
    pub fn is_available(&self) -> bool {
        which(self.executable()).is_ok()
    }
}

/// Detect the best available terminal emulator.
///
/// Returns the first available terminal from the preference list,
/// or None if no supported terminal is found.
#[must_use]
pub fn detect_terminal() -> Option<TerminalEmulator> {
    const PREFERENCE_ORDER: &[TerminalEmulator] = &[
        TerminalEmulator::Alacritty,
        TerminalEmulator::Kitty,
        TerminalEmulator::Ghostty,
        TerminalEmulator::WezTerm,
        TerminalEmulator::GnomeTerminal,
        TerminalEmulator::Konsole,
        TerminalEmulator::Xterm,
    ];

    PREFERENCE_ORDER.iter().find(|t| t.is_available()).copied()
}

/// Launch an SSH session in the detected terminal.
///
/// # Arguments
/// * `username` - SSH username
/// * `host` - Remote host
/// * `port` - SSH port (usually 22)
/// * `remote_dir` - Optional directory to cd into after connecting
///
/// # Errors
/// Returns an error if no terminal is available or the process fails to spawn.
pub fn launch_ssh_terminal(
    username: &str,
    host: &str,
    port: u16,
    remote_dir: Option<&str>,
) -> Result<(), std::io::Error> {
    let terminal = detect_terminal().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No supported terminal emulator found",
        )
    })?;

    let ssh_command = if let Some(dir) = remote_dir {
        format!("ssh -p {port} {username}@{host} -t 'cd {dir} && $SHELL'")
    } else {
        format!("ssh -p {port} {username}@{host}")
    };

    let mut cmd = Command::new(terminal.executable());
    for arg in terminal.exec_args() {
        cmd.arg(arg);
    }
    cmd.arg("sh").arg("-c").arg(&ssh_command);

    cmd.spawn()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_executable_names() {
        assert_eq!(TerminalEmulator::Alacritty.executable(), "alacritty");
        assert_eq!(TerminalEmulator::Kitty.executable(), "kitty");
        assert_eq!(
            TerminalEmulator::GnomeTerminal.executable(),
            "gnome-terminal"
        );
    }

    #[test]
    fn test_exec_args_not_empty() {
        // All terminals should have at least one exec arg
        assert!(!TerminalEmulator::Alacritty.exec_args().is_empty());
        assert!(!TerminalEmulator::Kitty.exec_args().is_empty());
    }

    #[test]
    fn test_detect_terminal_returns_available() {
        // This test may pass or fail depending on the system
        // but it should not panic
        let _ = detect_terminal();
    }
}
