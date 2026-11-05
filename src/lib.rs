#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChromiumFlags {
    pub enable_sandboxing: bool,
    pub disable_telemetry: bool,
    pub wayland_ozone_enabled: bool,
}

impl ChromiumFlags {
    pub fn sovereign_hardened() -> Self {
        Self {
            enable_sandboxing: true,
            disable_telemetry: true,
            wayland_ozone_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chromium_flags_aaa() {
        // Arrange & Act
        let flags = ChromiumFlags::sovereign_hardened();
        // Assert
        assert!(flags.enable_sandboxing);
        assert!(flags.disable_telemetry);
        assert!(flags.wayland_ozone_enabled);
    }
}
