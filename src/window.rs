//! Window definitions for chromium.

/// Window state
#[derive(Debug, Clone, PartialEq)]
pub enum WindowState {
    /// Normal window
    Normal,
    /// Minimized window
    Minimized,
    /// Maximized window
    Maximized,
    /// Fullscreen window
    Fullscreen,
}

/// Window kind
#[derive(Debug, Clone, PartialEq)]
pub enum WindowKind {
    /// Normal window
    Normal,
    /// Popup window
    Popup,
    /// DevTools window
    DevTools,
}

/// Window info
#[derive(Debug, Clone, Default)]
pub struct WindowInfo {
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// X position
    pub x: i32,
    /// Y position
    pub y: i32,
    /// Always on top
    pub always_on_top: bool,
}

/// Browser window
#[derive(Debug, Clone)]
pub struct BrowserWindow {
    /// Window ID
    pub id: String,
    /// Window state
    pub state: WindowState,
    /// Window kind
    pub kind: WindowKind,
    /// Window info
    pub info: WindowInfo,
    /// Tabs
    pub tabs: Vec<crate::tab::Tab>,
    /// Active tab index
    pub active_tab: usize,
    /// Created timestamp
    pub created_at: u64,
}

impl BrowserWindow {
    /// Create new browser window
    pub fn new() -> Self {
        Self {
            id: format!("window-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            state: WindowState::Normal,
            kind: WindowKind::Normal,
            info: WindowInfo::default(),
            tabs: vec![],
            active_tab: 0,
            created_at: Self::now(),
        }
    }
    
    /// Set size
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.info.width = width;
        self.info.height = height;
    }
    
    /// Set position
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.info.x = x;
        self.info.y = y;
    }
    
    /// Set always on top
    pub fn set_always_on_top(&mut self, on_top: bool) {
        self.info.always_on_top = on_top;
    }
    
    /// Add tab
    pub fn add_tab(&mut self, tab: crate::tab::Tab) -> usize {
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
        self.tabs.len() - 1
    }
    
    /// Remove tab
    pub fn remove_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len().saturating_sub(1);
            }
            return true;
        }
        false
    }
    
    /// Get tab
    pub fn get_tab(&self, index: usize) -> Option<&crate::tab::Tab> {
        self.tabs.get(index)
    }
    
    /// Get tab mutably
    pub fn get_tab_mut(&mut self, index: usize) -> Option<&mut crate::tab::Tab> {
        self.tabs.get_mut(index)
    }
    
    /// Get active tab
    pub fn get_active_tab(&self) -> Option<&crate::tab::Tab> {
        self.tabs.get(self.active_tab)
    }
    
    /// Switch tab
    pub fn switch_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_tab = index;
            return true;
        }
        false
    }
    
    /// Next tab
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }
    
    /// Previous tab
    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
        }
    }
    
    /// Minimize window
    pub fn minimize(&mut self) {
        self.state = WindowState::Minimized;
    }
    
    /// Maximize window
    pub fn maximize(&mut self) {
        self.state = WindowState::Maximized;
    }
    
    /// Fullscreen window
    pub fn fullscreen(&mut self) {
        self.state = WindowState::Fullscreen;
    }
    
    /// Restore window
    pub fn restore(&mut self) {
        self.state = WindowState::Normal;
    }
    
    /// Close window
    pub fn close(&mut self) {
        self.tabs.clear();
        self.state = WindowState::Normal;
    }
    
    /// Get tab count
    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
    
    /// Get window age in seconds
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

impl Default for BrowserWindow {
    fn default() -> Self {
        Self::new()
    }
}
