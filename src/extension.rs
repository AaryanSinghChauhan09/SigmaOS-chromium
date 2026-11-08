//! Extension definitions for chromium.

/// Extension state
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionState {
    /// Enabled extension
    Enabled,
    /// Disabled extension
    Disabled,
    /// Error extension
    Error,
}

/// Extension kind
#[derive(Debug, Clone, PartialEq)]
pub enum ExtensionKind {
    /// Extension
    Extension,
    /// App
    App,
    /// Theme
    Theme,
    /// Userscript
    Userscript,
}

/// Extension info
#[derive(Debug, Clone, Default)]
pub struct ExtensionInfo {
    /// Description
    pub description: String,
    /// Author
    pub author: String,
    /// Permissions
    pub permissions: Vec<String>,
    /// Homepage URL
    pub homepage_url: String,
    /// Update URL
    pub update_url: String,
    /// Installed timestamp
    pub installed_at: u64,
}

/// Extension
#[derive(Debug, Clone)]
pub struct Extension {
    /// Extension ID
    pub id: String,
    /// Extension name
    pub name: String,
    /// Extension version
    pub version: String,
    /// Extension state
    pub state: ExtensionState,
    /// Extension kind
    pub kind: ExtensionKind,
    /// Extension info
    pub info: ExtensionInfo,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl Extension {
    /// Create new extension
    pub fn new(name: String, version: String, kind: ExtensionKind) -> Self {
        Self {
            id: format!("ext-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            version,
            state: ExtensionState::Enabled,
            kind,
            info: ExtensionInfo::default(),
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Set info
    pub fn set_info(&mut self, info: ExtensionInfo) {
        self.info = info;
        self.last_modified = Self::now();
    }
    
    /// Enable extension
    pub fn enable(&mut self) {
        self.state = ExtensionState::Enabled;
        self.last_modified = Self::now();
    }
    
    /// Disable extension
    pub fn disable(&mut self) {
        self.state = ExtensionState::Disabled;
        self.last_modified = Self::now();
    }
    
    /// Set error
    pub fn set_error(&mut self) {
        self.state = ExtensionState::Error;
        self.last_modified = Self::now();
    }
    
    /// Add permission
    pub fn add_permission(&mut self, permission: &str) -> bool {
        if !self.info.permissions.contains(&permission.to_string()) {
            self.info.permissions.push(permission.to_string());
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Remove permission
    pub fn remove_permission(&mut self, permission: &str) -> bool {
        if let Some(pos) = self.info.permissions.iter().position(|p| p == permission) {
            self.info.permissions.remove(pos);
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Check if has permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.info.permissions.contains(&permission.to_string())
    }
    
    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.state == ExtensionState::Enabled
    }
    
    /// Get extension age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get time since last modified
    pub fn get_idle_seconds(&self) -> u64 {
        Self::now() - self.last_modified
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
