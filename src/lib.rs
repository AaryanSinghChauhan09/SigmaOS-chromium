#![forbid(unsafe_code)]
//! SigmaOS Chromium integration: browser profile and policy management.
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error)]
pub enum ChromiumError {
    #[error("profile not found: {0}")] ProfileNotFound(String),
    #[error("invalid policy: {0}")] InvalidPolicy(String),
    #[error("duplicate profile: {0}")] DuplicateProfile(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromiumProfile {
    pub name: String, pub path: String, pub is_default: bool, pub extensions: Vec<String>,
}
impl ChromiumProfile {
    pub fn new(name: &str, path: &str) -> Self {
        Self { name: name.into(), path: path.into(), is_default: false, extensions: vec![] }
    }
    pub fn add_extension(&mut self, id: &str) { self.extensions.push(id.into()); }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromiumPolicy { pub key: String, pub value: String }
impl ChromiumPolicy {
    pub fn new(key: &str, value: &str) -> Result<Self, ChromiumError> {
        if key.is_empty() { return Err(ChromiumError::InvalidPolicy("key cannot be empty".into())); }
        Ok(Self { key: key.into(), value: value.into() })
    }
}

pub struct ChromiumManager { profiles: Vec<ChromiumProfile>, policies: Vec<ChromiumPolicy> }
impl ChromiumManager {
    pub fn new() -> Self { Self { profiles: vec![], policies: vec![] } }
    pub fn add_profile(&mut self, p: ChromiumProfile) -> Result<(), ChromiumError> {
        if self.profiles.iter().any(|x| x.name == p.name) {
            return Err(ChromiumError::DuplicateProfile(p.name));
        }
        self.profiles.push(p);
        Ok(())
    }
    pub fn get_profile(&self, name: &str) -> Result<&ChromiumProfile, ChromiumError> {
        self.profiles.iter().find(|p| p.name == name).ok_or_else(|| ChromiumError::ProfileNotFound(name.into()))
    }
    pub fn add_policy(&mut self, p: ChromiumPolicy) { self.policies.push(p); }
    pub fn list_profiles(&self) -> &[ChromiumProfile] { &self.profiles }
}
impl Default for ChromiumManager { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_profile_aaa() {
        // Arrange
        let mut mgr = ChromiumManager::new();
        let p = ChromiumProfile::new("work", "/home/.config/chromium/work");
        // Act
        mgr.add_profile(p).unwrap();
        // Assert
        assert_eq!(mgr.get_profile("work").unwrap().name, "work");
    }
    #[test]
    fn test_duplicate_profile_aaa() {
        // Arrange
        let mut mgr = ChromiumManager::new();
        mgr.add_profile(ChromiumProfile::new("default", "/config/default")).unwrap();
        // Act
        let result = mgr.add_profile(ChromiumProfile::new("default", "/config/default2"));
        // Assert
        assert!(matches!(result, Err(ChromiumError::DuplicateProfile(_))));
    }
    #[test]
    fn test_extension_management_aaa() {
        // Arrange
        let mut profile = ChromiumProfile::new("dev", "/config/dev");
        // Act
        profile.add_extension("uBlock0");
        profile.add_extension("vimium");
        // Assert
        assert_eq!(profile.extensions.len(), 2);
    }
    #[test]
    fn test_invalid_policy_aaa() {
        // Arrange / Act
        let result = ChromiumPolicy::new("", "value");
        // Assert
        assert!(matches!(result, Err(ChromiumError::InvalidPolicy(_))));
    }
}
