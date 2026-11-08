//! Tab definitions for chromium.

/// Tab state
#[derive(Debug, Clone, PartialEq)]
pub enum TabState {
    /// Loading tab
    Loading,
    /// Loaded tab
    Loaded,
    /// Error tab
    Error,
    /// Crashed tab
    Crashed,
}

/// Tab kind
#[derive(Debug, Clone, PartialEq)]
pub enum TabKind {
    /// Normal tab
    Normal,
    /// Pinned tab
    Pinned,
    /// Incognito tab
    Incognito,
    /// App tab
    App,
}

/// Tab info
#[derive(Debug, Clone, Default)]
pub struct TabInfo {
    /// Favicon URL
    pub favicon_url: String,
    /// Load progress (0-100)
    pub load_progress: u8,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// CPU usage percent
    pub cpu_percent: f32,
    /// Last activity timestamp
    pub last_activity: u64,
}

/// Browser tab
#[derive(Debug, Clone)]
pub struct Tab {
    /// Tab ID
    pub id: String,
    /// Tab URL
    pub url: String,
    /// Tab title
    pub title: String,
    /// Tab state
    pub state: TabState,
    /// Tab kind
    pub kind: TabKind,
    /// Tab info
    pub info: TabInfo,
    /// Error message
    pub error: Option<String>,
    /// Created timestamp
    pub created_at: u64,
    /// Loaded timestamp
    pub loaded_at: Option<u64>,
}

impl Tab {
    /// Create new tab
    pub fn new(url: String) -> Self {
        Self {
            id: format!("tab-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            url,
            title: String::new(),
            state: TabState::Loading,
            kind: TabKind::Normal,
            info: TabInfo::default(),
            error: None,
            created_at: Self::now(),
            loaded_at: None,
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    
    /// Set URL
    pub fn set_url(&mut self, url: String) {
        self.url = url;
        self.state = TabState::Loading;
    }
    
    /// Set favicon
    pub fn set_favicon(&mut self, favicon_url: String) {
        self.info.favicon_url = favicon_url;
    }
    
    /// Set load progress
    pub fn set_load_progress(&mut self, progress: u8) {
        self.info.load_progress = progress.min(100);
    }
    
    /// Set memory usage
    pub fn set_memory(&mut self, bytes: u64) {
        self.info.memory_bytes = bytes;
    }
    
    /// Set CPU usage
    pub fn set_cpu(&mut self, percent: f32) {
        self.info.cpu_percent = percent;
    }
    
    /// Finish loading
    pub fn finish_loading(&mut self) {
        self.state = TabState::Loaded;
        self.loaded_at = Some(Self::now());
        self.info.load_progress = 100;
    }
    
    /// Fail loading
    pub fn fail(&mut self, error: String) {
        self.state = TabState::Error;
        self.error = Some(error);
    }
    
    /// Crash tab
    pub fn crash(&mut self) {
        self.state = TabState::Crashed;
    }
    
    /// Reload tab
    pub fn reload(&mut self) {
        self.state = TabState::Loading;
        self.error = None;
    }
    
    /// Pin tab
    pub fn pin(&mut self) {
        self.kind = TabKind::Pinned;
    }
    
    /// Unpin tab
    pub fn unpin(&mut self) {
        self.kind = TabKind::Normal;
    }
    
    /// Make incognito
    pub fn make_incognito(&mut self) {
        self.kind = TabKind::Incognito;
    }
    
    /// Check if loaded
    pub fn is_loaded(&self) -> bool {
        self.state == TabState::Loaded
    }
    
    /// Check if error
    pub fn is_error(&self) -> bool {
        self.state == TabState::Error
    }
    
    /// Get tab age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Get load duration in seconds
    pub fn get_load_duration(&self) -> u64 {
        if let Some(loaded) = self.loaded_at {
            loaded - self.created_at
        } else {
            0
        }
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
