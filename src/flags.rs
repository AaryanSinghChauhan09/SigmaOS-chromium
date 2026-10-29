//! Chromium security hardening and feature flags builder.
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct SecurityFlags {
    flags: BTreeSet<String>,
}

impl SecurityFlags {
    pub fn hardened() -> Self {
        let mut flags = BTreeSet::new();
        flags.insert("--no-default-browser-check".to_string());
        flags.insert("--no-first-run".to_string());
        flags.insert("--disable-sync".to_string());
        flags.insert("--disable-background-networking".to_string());
        flags.insert("--disable-default-apps".to_string());
        flags.insert("--disable-breakpad".to_string());
        flags.insert("--disable-component-update".to_string());
        flags.insert("--disable-domain-reliability".to_string());
        flags.insert("--enforce-webrtc-ip-permission-check".to_string());
        flags.insert("--force-webrtc-ip-handling-policy=disable_non_proxied_udp".to_string());
        flags.insert("--enable-features=NetworkServiceInProcess".to_string());
        Self { flags }
    }

    pub fn to_args(&self) -> Vec<String> {
        self.flags.iter().cloned().collect()
    }

    pub fn add_flag(&mut self, flag: impl Into<String>) {
        self.flags.insert(flag.into());
    }

    pub fn contains(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }

    pub fn count(&self) -> usize {
        self.flags.len()
    }
}

#[derive(Debug, Clone)]
pub struct FeatureFlags {
    enabled_features: Vec<String>,
    disabled_features: Vec<String>,
}

impl FeatureFlags {
    pub fn new() -> Self {
        Self {
            enabled_features: Vec::new(),
            disabled_features: Vec::new(),
        }
    }

    pub fn enable(&mut self, feature: impl Into<String>) {
        self.enabled_features.push(feature.into());
    }

    pub fn disable(&mut self, feature: impl Into<String>) {
        self.disabled_features.push(feature.into());
    }

    pub fn to_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if !self.enabled_features.is_empty() {
            args.push(format!("--enable-features={}", self.enabled_features.join(",")));
        }
        if !self.disabled_features.is_empty() {
            args.push(format!("--disable-features={}", self.disabled_features.join(",")));
        }
        args
    }
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardened_flags_contain_privacy_options() {
        // Arrange & Act
        let sec = SecurityFlags::hardened();

        // Assert
        assert!(sec.contains("--no-first-run"));
        assert!(sec.contains("--disable-sync"));
        assert!(sec.contains("--disable-breakpad"));
        assert!(sec.count() >= 10);
    }

    #[test]
    fn test_feature_flags_formatting() {
        // Arrange
        let mut ff = FeatureFlags::new();

        // Act
        ff.enable("VaapiVideoDecoder");
        ff.disable("TelemetryExtension");
        let args = ff.to_args();

        // Assert
        assert_eq!(args.len(), 2);
        assert!(args[0].contains("VaapiVideoDecoder"));
        assert!(args[1].contains("TelemetryExtension"));
    }
}
