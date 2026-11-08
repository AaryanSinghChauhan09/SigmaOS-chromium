//! Chromium manager.

use std::collections::HashMap;
use crate::tab::{Tab, TabState, TabKind};
use crate::window::{BrowserWindow, WindowState, WindowKind};
use crate::history::{HistoryEntry, HistoryKind, HistoryState};
use crate::bookmark::{Bookmark, BookmarkState, BookmarkFolder};
use crate::download::{Download, DownloadState, DownloadKind};
use crate::extension::{Extension, ExtensionState, ExtensionKind};

/// Manager options
#[derive(Debug, Clone)]
pub struct ManagerOptions {
    /// Max windows
    pub max_windows: usize,
    /// Max tabs per window
    pub max_tabs_per_window: usize,
    /// Max history entries
    pub max_history_entries: usize,
    /// Max bookmarks
    pub max_bookmarks: usize,
    /// Max downloads
    pub max_downloads: usize,
    /// Max extensions
    pub max_extensions: usize,
}

impl Default for ManagerOptions {
    fn default() -> Self {
        Self {
            max_windows: 20,
            max_tabs_per_window: 100,
            max_history_entries: 10000,
            max_bookmarks: 1000,
            max_downloads: 50,
            max_extensions: 100,
        }
    }
}

/// Manager statistics
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Total windows
    pub total_windows: usize,
    /// Total tabs
    pub total_tabs: usize,
    /// Loaded tabs
    pub loaded_tabs: usize,
    /// Error tabs
    pub error_tabs: usize,
    /// Total history entries
    pub total_history: usize,
    /// Total bookmarks
    pub total_bookmarks: usize,
    /// Total downloads
    pub total_downloads: usize,
    /// Active downloads
    pub active_downloads: usize,
    /// Total extensions
    pub total_extensions: usize,
    /// Enabled extensions
    pub enabled_extensions: usize,
    /// Manager uptime in seconds
    pub uptime: u64,
}

/// Chromium manager
#[derive(Debug, Clone)]
pub struct ChromiumManager {
    /// Windows
    windows: HashMap<String, BrowserWindow>,
    /// Window -> tabs mapping
    window_tabs: HashMap<String, Vec<String>>,
    /// Tab lookup
    tab_lookup: HashMap<String, (String, usize)>,
    /// History entries
    history: HashMap<String, HistoryEntry>,
    /// Bookmarks
    bookmarks: HashMap<String, Bookmark>,
    /// Bookmark folders
    bookmark_folders: HashMap<String, BookmarkFolder>,
    /// Downloads
    downloads: HashMap<String, Download>,
    /// Extensions
    extensions: HashMap<String, Extension>,
    /// Options
    options: ManagerOptions,
    /// Statistics
    statistics: ManagerStatistics,
    /// Manager created timestamp
    pub created_at: u64,
}

impl Default for ChromiumManager {
    fn default() -> Self {
        Self {
            windows: HashMap::new(),
            window_tabs: HashMap::new(),
            tab_lookup: HashMap::new(),
            history: HashMap::new(),
            bookmarks: HashMap::new(),
            bookmark_folders: HashMap::new(),
            downloads: HashMap::new(),
            extensions: HashMap::new(),
            options: ManagerOptions::default(),
            statistics: ManagerStatistics {
                total_windows: 0,
                total_tabs: 0,
                loaded_tabs: 0,
                error_tabs: 0,
                total_history: 0,
                total_bookmarks: 0,
                total_downloads: 0,
                active_downloads: 0,
                total_extensions: 0,
                enabled_extensions: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
}

impl ChromiumManager {
    /// Create new chromium manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create manager with options
    pub fn with_options(options: ManagerOptions) -> Self {
        Self {
            windows: HashMap::new(),
            window_tabs: HashMap::new(),
            tab_lookup: HashMap::new(),
            history: HashMap::new(),
            bookmarks: HashMap::new(),
            bookmark_folders: HashMap::new(),
            downloads: HashMap::new(),
            extensions: HashMap::new(),
            options,
            statistics: ManagerStatistics {
                total_windows: 0,
                total_tabs: 0,
                loaded_tabs: 0,
                error_tabs: 0,
                total_history: 0,
                total_bookmarks: 0,
                total_downloads: 0,
                active_downloads: 0,
                total_extensions: 0,
                enabled_extensions: 0,
                uptime: 0,
            },
            created_at: Self::now(),
        }
    }
    
    /// Initialize manager
    pub fn initialize(&mut self) {
        // Create default bookmark folders
        let mut bookmarks_folder = BookmarkFolder::new("Bookmarks".into());
        bookmarks_folder.add_subfolder(BookmarkFolder::new("Other".into()));
        self.bookmark_folders.insert("bookmarks".into(), bookmarks_folder);
        
        self.update_statistics();
    }
    
    // Window methods
    
    /// New window
    pub fn new_window(&mut self) -> String {
        let window = BrowserWindow::new();
        let id = window.id.clone();
        
        self.windows.insert(id.clone(), window);
        self.window_tabs.insert(id.clone(), vec![]);
        self.update_statistics();
        id
    }
    
    /// Get window
    pub fn get_window(&self, win_id: &str) -> Option<&BrowserWindow> {
        self.windows.get(win_id)
    }
    
    /// Get window mutably
    pub fn get_window_mut(&mut self, win_id: &str) -> Option<&mut BrowserWindow> {
        self.windows.get_mut(win_id)
    }
    
    /// Get all windows
    pub fn list_windows(&self) -> Vec<&BrowserWindow> {
        self.windows.values().collect()
    }
    
    /// Remove window
    pub fn remove_window(&mut self, win_id: &str) -> bool {
        if let Some(window) = self.windows.remove(win_id) {
            // Remove all tabs
            for tab in window.tabs {
                self.tab_lookup.remove(&tab.id);
            }
            self.window_tabs.remove(win_id);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get windows by state
    pub fn get_windows_by_state(&self, state: &WindowState) -> Vec<&BrowserWindow> {
        self.windows.values()
            .filter(|w| w.state == *state)
            .collect()
    }
    
    // Tab methods
    
    /// New tab
    pub fn new_tab(&mut self, win_id: &str, url: String) -> String {
        let tab = Tab::new(url);
        let tab_id = tab.id.clone();
        
        if let Some(window) = self.windows.get_mut(win_id) {
            window.add_tab(tab);
            
            let tab_ids = self.window_tabs.entry(win_id.to_string())
                .or_insert_with(Vec::new);
            tab_ids.push(tab_id.clone());
            
            self.tab_lookup.insert(tab_id.clone(), (win_id.to_string(), window.tabs.len() - 1));
            self.update_statistics();
        }
        
        tab_id
    }
    
    /// Get tab
    pub fn get_tab(&self, tab_id: &str) -> Option<&Tab> {
        if let Some((win_id, idx)) = self.tab_lookup.get(tab_id) {
            return self.windows.get(win_id)
                .and_then(|w| w.tabs.get(*idx));
        }
        None
    }
    
    /// Get tab mutably
    pub fn get_tab_mut(&mut self, tab_id: &str) -> Option<&mut Tab> {
        if let Some((win_id, idx)) = self.tab_lookup.get(tab_id) {
            let win_id = win_id.clone();
            let idx = *idx;
            return self.windows.get_mut(&win_id)
                .and_then(|w| w.tabs.get_mut(idx));
        }
        None
    }
    
    /// Get all tabs
    pub fn list_tabs(&self) -> Vec<&Tab> {
        self.windows.values()
            .flat_map(|w| w.tabs.iter())
            .collect()
    }
    
    /// Get tabs for window
    pub fn get_window_tabs(&self, win_id: &str) -> Vec<&Tab> {
        self.windows.get(win_id)
            .map(|w| w.tabs.iter().collect())
            .unwrap_or_default()
    }
    
    /// Close tab
    pub fn close_tab(&mut self, tab_id: &str) -> bool {
        if let Some((win_id, idx)) = self.tab_lookup.remove(tab_id) {
            if let Some(window) = self.windows.get_mut(&win_id) {
                if window.remove_tab(idx) {
                    // Update indices
                    self.rebuild_tab_index(&win_id);
                    self.update_statistics();
                    return true;
                }
            }
        }
        false
    }
    
    /// Switch tab in window
    pub fn switch_tab(&mut self, win_id: &str, tab_index: usize) -> bool {
        if let Some(window) = self.windows.get_mut(win_id) {
            return window.switch_tab(tab_index);
        }
        false
    }
    
    /// Next tab in window
    pub fn next_tab(&mut self, win_id: &str) -> bool {
        if let Some(window) = self.windows.get_mut(win_id) {
            window.next_tab();
            return true;
        }
        false
    }
    
    /// Previous tab in window
    pub fn prev_tab(&mut self, win_id: &str) -> bool {
        if let Some(window) = self.windows.get_mut(win_id) {
            window.prev_tab();
            return true;
        }
        false
    }
    
    /// Load tab
    pub fn load_tab(&mut self, tab_id: &str, url: String) -> bool {
        if let Some(tab) = self.get_tab_mut(tab_id) {
            tab.set_url(url);
            tab.finish_loading();
            return true;
        }
        false
    }
    
    /// Fail tab
    pub fn fail_tab(&mut self, tab_id: &str, error: String) -> bool {
        if let Some(tab) = self.get_tab_mut(tab_id) {
            tab.fail(error);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Pin tab
    pub fn pin_tab(&mut self, tab_id: &str) -> bool {
        if let Some(tab) = self.get_tab_mut(tab_id) {
            tab.pin();
            return true;
        }
        false
    }
    
    /// Search tabs
    pub fn search_tabs(&self, query: &str) -> Vec<&Tab> {
        let query_lower = query.to_lowercase();
        self.list_tabs()
            .into_iter()
            .filter(|t| t.url.to_lowercase().contains(&query_lower) ||
                       t.title.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    fn rebuild_tab_index(&mut self, win_id: &str) {
        self.tab_lookup.retain(|_, (wid, _)| wid != win_id);
        if let Some(window) = self.windows.get(win_id) {
            for (idx, tab) in window.tabs.iter().enumerate() {
                self.tab_lookup.insert(tab.id.clone(), (win_id.to_string(), idx));
            }
        }
    }
    
    // History methods
    
    /// Add history entry
    pub fn add_history(&mut self, url: String, kind: HistoryKind) -> String {
        let entry = HistoryEntry::new(url.clone(), kind);
        let id = url.clone();
        
        self.history.insert(id.clone(), entry);
        self.update_statistics();
        id
    }
    
    /// Get history entry
    pub fn get_history(&self, history_id: &str) -> Option<&HistoryEntry> {
        self.history.get(history_id)
    }
    
    /// Get history mutably
    pub fn get_history_mut(&mut self, history_id: &str) -> Option<&mut HistoryEntry> {
        self.history.get_mut(history_id)
    }
    
    /// Get all history
    pub fn list_history(&self) -> Vec<&HistoryEntry> {
        self.history.values().collect()
    }
    
    /// Remove history entry
    pub fn remove_history(&mut self, history_id: &str) -> bool {
        if self.history.remove(history_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Visit history entry
    pub fn visit_history(&mut self, history_id: &str) -> bool {
        if let Some(entry) = self.history.get_mut(history_id) {
            entry.increment_visit();
            return true;
        }
        false
    }
    
    /// Search history
    pub fn search_history(&self, query: &str) -> Vec<&HistoryEntry> {
        let query_lower = query.to_lowercase();
        self.history.values()
            .filter(|h| h.url.to_lowercase().contains(&query_lower) ||
                       h.title.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Bookmark methods
    
    /// Add bookmark
    pub fn add_bookmark(&mut self, name: String, url: String) -> String {
        let bookmark = Bookmark::new(name.clone(), url);
        let id = name.clone();
        
        self.bookmarks.insert(id.clone(), bookmark);
        self.update_statistics();
        id
    }
    
    /// Get bookmark
    pub fn get_bookmark(&self, bookmark_id: &str) -> Option<&Bookmark> {
        self.bookmarks.get(bookmark_id)
    }
    
    /// Get bookmark mutably
    pub fn get_bookmark_mut(&mut self, bookmark_id: &str) -> Option<&mut Bookmark> {
        self.bookmarks.get_mut(bookmark_id)
    }
    
    /// Get all bookmarks
    pub fn list_bookmarks(&self) -> Vec<&Bookmark> {
        self.bookmarks.values().collect()
    }
    
    /// Remove bookmark
    pub fn remove_bookmark(&mut self, bookmark_id: &str) -> bool {
        if self.bookmarks.remove(bookmark_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Search bookmarks
    pub fn search_bookmarks(&self, query: &str) -> Vec<&Bookmark> {
        let query_lower = query.to_lowercase();
        self.bookmarks.values()
            .filter(|b| b.name.to_lowercase().contains(&query_lower) ||
                       b.url.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    /// Add bookmark folder
    pub fn add_bookmark_folder(&mut self, name: String) -> String {
        let folder = BookmarkFolder::new(name.clone());
        let id = name.clone();
        self.bookmark_folders.insert(id.clone(), folder);
        id
    }
    
    /// Get bookmark folder
    pub fn get_bookmark_folder(&self, folder_id: &str) -> Option<&BookmarkFolder> {
        self.bookmark_folders.get(folder_id)
    }
    
    /// Get all bookmark folders
    pub fn list_bookmark_folders(&self) -> Vec<&BookmarkFolder> {
        self.bookmark_folders.values().collect()
    }
    
    // Download methods
    
    /// Download file
    pub fn download_file(&mut self, filename: String, url: String, size: u64) -> String {
        let download = Download::new(filename.clone(), url, size);
        let id = filename.clone();
        
        self.downloads.insert(id.clone(), download);
        self.update_statistics();
        id
    }
    
    /// Get download
    pub fn get_download(&self, download_id: &str) -> Option<&Download> {
        self.downloads.get(download_id)
    }
    
    /// Get download mutably
    pub fn get_download_mut(&mut self, download_id: &str) -> Option<&mut Download> {
        self.downloads.get_mut(download_id)
    }
    
    /// Get all downloads
    pub fn list_downloads(&self) -> Vec<&Download> {
        self.downloads.values().collect()
    }
    
    /// Remove download
    pub fn remove_download(&mut self, download_id: &str) -> bool {
        if self.downloads.remove(download_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Start download
    pub fn start_download(&mut self, download_id: &str) -> bool {
        if let Some(download) = self.downloads.get_mut(download_id) {
            download.start();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Complete download
    pub fn complete_download(&mut self, download_id: &str, path: String) -> bool {
        if let Some(download) = self.downloads.get_mut(download_id) {
            download.complete(path);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Fail download
    pub fn fail_download(&mut self, download_id: &str, error: String) -> bool {
        if let Some(download) = self.downloads.get_mut(download_id) {
            download.fail(error);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get downloads by state
    pub fn get_downloads_by_state(&self, state: &DownloadState) -> Vec<&Download> {
        self.downloads.values()
            .filter(|d| d.state == *state)
            .collect()
    }
    
    /// Get active downloads
    pub fn get_active_downloads(&self) -> Vec<&Download> {
        self.get_downloads_by_state(&DownloadState::InProgress)
    }
    
    /// Search downloads
    pub fn search_downloads(&self, query: &str) -> Vec<&Download> {
        let query_lower = query.to_lowercase();
        self.downloads.values()
            .filter(|d| d.filename.to_lowercase().contains(&query_lower) ||
                       d.url.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Extension methods
    
    /// Install extension
    pub fn install_extension(&mut self, name: String, version: String, kind: ExtensionKind) -> String {
        let ext = Extension::new(name.clone(), version, kind);
        let id = name.clone();
        
        self.extensions.insert(id.clone(), ext);
        self.update_statistics();
        id
    }
    
    /// Get extension
    pub fn get_extension(&self, ext_id: &str) -> Option<&Extension> {
        self.extensions.get(ext_id)
    }
    
    /// Get extension mutably
    pub fn get_extension_mut(&mut self, ext_id: &str) -> Option<&mut Extension> {
        self.extensions.get_mut(ext_id)
    }
    
    /// Get all extensions
    pub fn list_extensions(&self) -> Vec<&Extension> {
        self.extensions.values().collect()
    }
    
    /// Remove extension
    pub fn remove_extension(&mut self, ext_id: &str) -> bool {
        if self.extensions.remove(ext_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Enable extension
    pub fn enable_extension(&mut self, ext_id: &str) -> bool {
        if let Some(ext) = self.extensions.get_mut(ext_id) {
            ext.enable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Disable extension
    pub fn disable_extension(&mut self, ext_id: &str) -> bool {
        if let Some(ext) = self.extensions.get_mut(ext_id) {
            ext.disable();
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Get extensions by state
    pub fn get_extensions_by_state(&self, state: &ExtensionState) -> Vec<&Extension> {
        self.extensions.values()
            .filter(|e| e.state == *state)
            .collect()
    }
    
    /// Get enabled extensions
    pub fn get_enabled_extensions(&self) -> Vec<&Extension> {
        self.get_extensions_by_state(&ExtensionState::Enabled)
    }
    
    /// Search extensions
    pub fn search_extensions(&self, query: &str) -> Vec<&Extension> {
        let query_lower = query.to_lowercase();
        self.extensions.values()
            .filter(|e| e.name.to_lowercase().contains(&query_lower))
            .collect()
    }
    
    // Statistics methods
    
    /// Get manager uptime
    pub fn get_uptime(&self) -> u64 {
        Self::now() - self.created_at
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_windows = self.windows.len();
        self.statistics.total_tabs = self.list_tabs().len();
        self.statistics.loaded_tabs = self.list_tabs()
            .into_iter()
            .filter(|t| t.state == TabState::Loaded)
            .count();
        self.statistics.error_tabs = self.list_tabs()
            .into_iter()
            .filter(|t| t.state == TabState::Error)
            .count();
        self.statistics.total_history = self.history.len();
        self.statistics.total_bookmarks = self.bookmarks.len();
        self.statistics.total_downloads = self.downloads.len();
        self.statistics.active_downloads = self.downloads.values()
            .filter(|d| d.state == DownloadState::InProgress)
            .count();
        self.statistics.total_extensions = self.extensions.len();
        self.statistics.enabled_extensions = self.extensions.values()
            .filter(|e| e.state == ExtensionState::Enabled)
            .count();
        self.statistics.uptime = self.get_uptime();
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }
    
    /// Export manager data
    pub fn export_data(&self) -> ManagerExport {
        ManagerExport {
            windows: self.windows.values().cloned().collect(),
            history: self.history.values().cloned().collect(),
            bookmarks: self.bookmarks.values().cloned().collect(),
            extensions: self.extensions.values().cloned().collect(),
        }
    }
    
    /// Import manager data
    pub fn import_data(&mut self, export: ManagerExport) {
        self.windows.clear();
        self.window_tabs.clear();
        self.tab_lookup.clear();
        for window in export.windows {
            self.windows.insert(window.id.clone(), window);
        }
        
        self.history.clear();
        for entry in export.history {
            self.history.insert(entry.url.clone(), entry);
        }
        
        self.bookmarks.clear();
        for bookmark in export.bookmarks {
            self.bookmarks.insert(bookmark.name.clone(), bookmark);
        }
        
        self.extensions.clear();
        for ext in export.extensions {
            self.extensions.insert(ext.name.clone(), ext);
        }
        
        self.update_statistics();
    }
    
    fn now() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

/// Manager export data
#[derive(Debug, Clone)]
pub struct ManagerExport {
    /// Windows
    pub windows: Vec<BrowserWindow>,
    /// History entries
    pub history: Vec<HistoryEntry>,
    /// Bookmarks
    pub bookmarks: Vec<Bookmark>,
    /// Extensions
    pub extensions: Vec<Extension>,
}
