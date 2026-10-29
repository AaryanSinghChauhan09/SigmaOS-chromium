//! Chromium launcher and process invocation builder.
use crate::flags::{FeatureFlags, SecurityFlags};
use crate::profile::BrowserProfile;

#[derive(Debug, Clone)]
pub struct LaunchConfig {
    pub binary_path: String,
    pub target_url: Option<String>,
    pub app_mode: bool,
}

impl Default for LaunchConfig {
    fn default() -> Self {
        Self {
            binary_path: "chromium".to_string(),
            target_url: None,
            app_mode: false,
        }
    }
}

pub struct ChromiumLauncher {
    config: LaunchConfig,
    security_flags: SecurityFlags,
    feature_flags: FeatureFlags,
    profile: BrowserProfile,
}

impl ChromiumLauncher {
    pub fn new(config: LaunchConfig, profile: BrowserProfile) -> Self {
        Self {
            config,
            security_flags: SecurityFlags::hardened(),
            feature_flags: FeatureFlags::new(),
            profile,
        }
    }

    // ── Queries (Command-Query Separation) ──────────────────────────────────
    pub fn build_command_line(&self) -> Vec<String> {
        let mut cmd = vec![self.config.binary_path.clone()];
        cmd.extend(self.profile.to_args());
        cmd.extend(self.security_flags.to_args());
        cmd.extend(self.feature_flags.to_args());

        if let Some(ref url) = self.config.target_url {
            if self.config.app_mode {
                cmd.push(format!("--app={}", url));
            } else {
                cmd.push(url.clone());
            }
        }
        cmd
    }

    pub fn target_url(&self) -> Option<&str> {
        self.config.target_url.as_deref()
    }

    // ── Commands (Command-Query Separation) ──────────────────────────────────
    pub fn set_target_url(&mut self, url: impl Into<String>) {
        self.config.target_url = Some(url.into());
    }

    pub fn set_app_mode(&mut self, app_mode: bool) {
        self.config.app_mode = app_mode;
    }

    pub fn security_flags_mut(&mut self) -> &mut SecurityFlags {
        &mut self.security_flags
    }

    pub fn feature_flags_mut(&mut self) -> &mut FeatureFlags {
        &mut self.feature_flags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::ProfileKind;

    #[test]
    fn test_launcher_builds_full_command_line() {
        // Arrange
        let profile = BrowserProfile::new("default", ProfileKind::Standard, "/tmp/profiles");
        let mut launcher = ChromiumLauncher::new(LaunchConfig::default(), profile);

        // Act
        launcher.set_target_url("https://sigmaos.org");
        let cmd = launcher.build_command_line();

        // Assert
        assert_eq!(cmd[0], "chromium");
        assert!(cmd.iter().any(|arg| arg.contains("--user-data-dir=")));
        assert!(cmd.iter().any(|arg| arg == "--no-first-run"));
        assert_eq!(cmd.last().unwrap(), "https://sigmaos.org");
    }

    #[test]
    fn test_webapp_mode_uses_app_flag() {
        // Arrange
        let profile = BrowserProfile::new("slack", ProfileKind::WebApp, "/tmp/webapps");
        let mut launcher = ChromiumLauncher::new(LaunchConfig::default(), profile);

        // Act
        launcher.set_target_url("https://app.slack.com");
        launcher.set_app_mode(true);
        let cmd = launcher.build_command_line();

        // Assert
        assert!(cmd.iter().any(|arg| arg == "--app=https://app.slack.com"));
    }
}
