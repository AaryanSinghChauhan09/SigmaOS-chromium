//! History definitions for chromium.

/// History kind
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryKind {
    /// Visit history
    Visit,
    /// Download history
    Download,
    /// Typed history
    Typed,
}

/// History state
#[derive(Debug, Clone, PartialEq)]
pub enum HistoryState {
    /// Active history
    Active,
    /// Deleted history
    Deleted,
}

/// History info
#[derive(Debug, Clone, Default)]
pub struct HistoryInfo {
    /// Visit count
    pub visit_count: u32,
    /// Last visit timestamp
    pub last_visit: u64,
    /// Typed count
    pub typed_count: u32,
}

/// History entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// Entry ID
    pub id: String,
    /// Entry URL
    pub url: String,
    /// Entry title
    pub title: String,
    /// Entry kind
    pub kind: HistoryKind,
    /// Entry state
    pub state: HistoryState,
    /// Entry info
    pub info: HistoryInfo,
    /// Created timestamp
    pub created_at: u64,
}

impl HistoryEntry {
    /// Create new history entry
    pub fn new(url: String, kind: HistoryKind) -> Self {
        Self {
            id: format!("history-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            url,
            title: String::new(),
            kind,
            state: HistoryState::Active,
            info: HistoryInfo::default(),
            created_at: Self::now(),
        }
    }
    
    /// Set title
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    
    /// Set visit count
    pub fn set_visit_count(&mut self, count: u32) {
        self.info.visit_count = count;
    }
    
    /// Increment visit count
    pub fn increment_visit(&mut self) {
        self.info.visit_count += 1;
        self.info.last_visit = Self::now();
    }
    
    /// Set last visit
    pub fn set_last_visit(&mut self, timestamp: u64) {
        self.info.last_visit = timestamp;
    }
    
    /// Delete entry
    pub fn delete(&mut self) {
        self.state = HistoryState::Deleted;
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == HistoryState::Active
    }
    
    /// Get entry age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
