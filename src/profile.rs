//! Browser profile definitions and data directory isolation.
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileKind {
    Standard,
    WebApp,
    Incognito,
    IsolatedEphemeral,
}

#[derive(Debug, Clone)]
pub struct BrowserProfile {
    pub name: String,
    pub kind: ProfileKind,
    pub user_data_dir: PathBuf,
}

impl BrowserProfile {
    pub fn new(name: impl Into<String>, kind: ProfileKind, base_dir: impl AsRef<Path>) -> Self {
        let name_str = name.into();
        let user_data_dir = base_dir.as_ref().join(&name_str);
        Self {
            name: name_str,
            kind,
            user_data_dir,
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            format!("--user-data-dir={}", self.user_data_dir.display()),
        ];
        if self.kind == ProfileKind::Incognito || self.kind == ProfileKind::IsolatedEphemeral {
            args.push("--incognito".to_string());
        }
        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_user_data_dir_args() {
        // Arrange & Act
        let profile = BrowserProfile::new("work", ProfileKind::Standard, "/home/user/.config/sigma-chromium");
        let args = profile.to_args();

        // Assert
        assert_eq!(args.len(), 1);
        assert!(args[0].contains("/home/user/.config/sigma-chromium/work"));
    }

    #[test]
    fn test_incognito_profile_adds_flag() {
        // Arrange & Act
        let profile = BrowserProfile::new("temp", ProfileKind::Incognito, "/tmp");
        let args = profile.to_args();

        // Assert
        assert!(args.contains(&"--incognito".to_string()));
    }
}
