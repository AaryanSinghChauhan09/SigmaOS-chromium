//! SigmaOS-chromium — web browser for SigmaOS.
//!
//! Models chromium's browser functionality: tabs, windows, history,
//! bookmarks, downloads, extensions, and settings.

pub mod tab;
pub mod window;
pub mod history;
pub mod bookmark;
pub mod download;
pub mod extension;
pub mod manager;
pub mod options;
pub mod util;

pub use tab::{Tab, TabState, TabKind, TabInfo};
pub use window::{BrowserWindow, WindowState, WindowKind, WindowInfo};
pub use history::{HistoryEntry, HistoryKind, HistoryState};
pub use bookmark::{Bookmark, BookmarkState, BookmarkFolder};
pub use download::{Download, DownloadState, DownloadKind, DownloadInfo};
pub use extension::{Extension, ExtensionState, ExtensionKind, ExtensionInfo};
pub use manager::{ChromiumManager, ManagerOptions, ManagerStatistics};
pub use options::{Mode, Options};
pub use util::{Tab as UtilTab, Model};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tab_state() {
        let states = vec![TabState::Loading, TabState::Loaded, TabState::Error, TabState::Crashed];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_tab_kind() {
        let kinds = vec![TabKind::Normal, TabKind::Pinned, TabKind::Incognito, TabKind::App];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_window_state() {
        let states = vec![WindowState::Normal, WindowState::Minimized, WindowState::Maximized, WindowState::Fullscreen];
        assert_eq!(states.len(), 4);
    }
    
    #[test]
    fn test_history_kind() {
        let kinds = vec![HistoryKind::Visit, HistoryKind::Download, HistoryKind::Typed];
        assert_eq!(kinds.len(), 3);
    }
    
    #[test]
    fn test_bookmark_state() {
        let states = vec![BookmarkState::Active, BookmarkState::Deleted];
        assert_eq!(states.len(), 2);
    }
    
    #[test]
    fn test_download_state() {
        let states = vec![DownloadState::Pending, DownloadState::InProgress, DownloadState::Paused, DownloadState::Completed, DownloadState::Failed, DownloadState::Cancelled];
        assert_eq!(states.len(), 6);
    }
    
    #[test]
    fn test_download_kind() {
        let kinds = vec![DownloadKind::File, DownloadKind::Page, DownloadKind::Image];
        assert_eq!(kinds.len(), 3);
    }
    
    #[test]
    fn test_extension_state() {
        let states = vec![ExtensionState::Enabled, ExtensionState::Disabled, ExtensionState::Error];
        assert_eq!(states.len(), 3);
    }
    
    #[test]
    fn test_extension_kind() {
        let kinds = vec![ExtensionKind::Extension, ExtensionKind::App, ExtensionKind::Theme, ExtensionKind::Userscript];
        assert_eq!(kinds.len(), 4);
    }
    
    #[test]
    fn test_tab_creation() {
        let tab = Tab::new("https://example.com".into());
        assert_eq!(tab.url, "https://example.com".to_string());
        assert_eq!(tab.state, TabState::Loading);
        assert_eq!(tab.kind, TabKind::Normal);
    }
    
    #[test]
    fn test_tab_load() {
        let mut tab = Tab::new("https://test.com".into());
        tab.set_title("Test Page".into());
        tab.finish_loading();
        assert_eq!(tab.state, TabState::Loaded);
        assert_eq!(tab.title, "Test Page".to_string());
    }
    
    #[test]
    fn test_tab_error() {
        let mut tab = Tab::new("https://bad.com".into());
        tab.fail("DNS resolution failed".into());
        assert_eq!(tab.state, TabState::Error);
        assert_eq!(tab.error, Some("DNS resolution failed".to_string()));
    }
    
    #[test]
    fn test_tab_pin() {
        let mut tab = Tab::new("https://app.com".into());
        tab.pin();
        assert_eq!(tab.kind, TabKind::Pinned);
        tab.unpin();
        assert_eq!(tab.kind, TabKind::Normal);
    }
    
    #[test]
    fn test_window_creation() {
        let window = BrowserWindow::new();
        assert_eq!(window.state, WindowState::Normal);
        assert!(window.tabs.is_empty());
    }
    
    #[test]
    fn test_window_add_tab() {
        let mut window = BrowserWindow::new();
        window.add_tab(Tab::new("https://a.com".into()));
        window.add_tab(Tab::new("https://b.com".into()));
        
        assert_eq!(window.tabs.len(), 2);
        assert_eq!(window.active_tab, 1);
    }
    
    #[test]
    fn test_window_switch_tab() {
        let mut window = BrowserWindow::new();
        window.add_tab(Tab::new("https://a.com".into()));
        window.add_tab(Tab::new("https://b.com".into()));
        
        window.switch_tab(0);
        assert_eq!(window.active_tab, 0);
        window.next_tab();
        assert_eq!(window.active_tab, 1);
    }
    
    #[test]
    fn test_window_state_changes() {
        let mut window = BrowserWindow::new();
        window.minimize();
        assert_eq!(window.state, WindowState::Minimized);
        window.maximize();
        assert_eq!(window.state, WindowState::Maximized);
        window.fullscreen();
        assert_eq!(window.state, WindowState::Fullscreen);
        window.restore();
        assert_eq!(window.state, WindowState::Normal);
    }
    
    #[test]
    fn test_history_creation() {
        let entry = HistoryEntry::new("https://example.com".into(), HistoryKind::Visit);
        assert_eq!(entry.url, "https://example.com".to_string());
        assert_eq!(entry.kind, HistoryKind::Visit);
        assert_eq!(entry.state, HistoryState::Active);
    }
    
    #[test]
    fn test_history_title() {
        let mut entry = HistoryEntry::new("https://test.com".into(), HistoryKind::Visit);
        entry.set_title("Test".into());
        entry.set_visit_count(5);
        
        assert_eq!(entry.title, "Test".to_string());
        assert_eq!(entry.info.visit_count, 5);
    }
    
    #[test]
    fn test_bookmark_creation() {
        let bookmark = Bookmark::new("Example".into(), "https://example.com".into());
        assert_eq!(bookmark.name, "Example".to_string());
        assert_eq!(bookmark.url, "https://example.com".to_string());
        assert_eq!(bookmark.state, BookmarkState::Active);
    }
    
    #[test]
    fn test_bookmark_folder() {
        let mut folder = BookmarkFolder::new("Work".into());
        folder.add_bookmark(Bookmark::new("Site1".into(), "https://s1.com".into()));
        folder.add_bookmark(Bookmark::new("Site2".into(), "https://s2.com".into()));
        
        assert_eq!(folder.bookmarks.len(), 2);
        assert_eq!(folder.name, "Work".to_string());
    }
    
    #[test]
    fn test_download_creation() {
        let download = Download::new("file.zip".into(), "https://example.com/file.zip".into(), 1024 * 1024);
        assert_eq!(download.filename, "file.zip".to_string());
        assert_eq!(download.url, "https://example.com/file.zip".to_string());
        assert_eq!(download.state, DownloadState::Pending);
        assert_eq!(download.info.total_size, 1024 * 1024);
    }
    
    #[test]
    fn test_download_progress() {
        let mut download = Download::new("test.bin".into(), "https://test.com".into(), 1000);
        download.start();
        assert_eq!(download.state, DownloadState::InProgress);
        download.set_progress(500);
        assert_eq!(download.info.downloaded_size, 500);
        assert_eq!(download.get_progress_percent(), 50.0);
    }
    
    #[test]
    fn test_download_complete() {
        let mut download = Download::new("file.txt".into(), "https://f.com".into(), 100);
        download.start();
        download.complete("/tmp/file.txt".into());
        assert_eq!(download.state, DownloadState::Completed);
        assert_eq!(download.info.saved_path, Some("/tmp/file.txt".to_string()));
    }
    
    #[test]
    fn test_download_fail() {
        let mut download = Download::new("file".into(), "https://f.com".into(), 100);
        download.start();
        download.fail("Network error".into());
        assert_eq!(download.state, DownloadState::Failed);
        assert_eq!(download.error, Some("Network error".to_string()));
    }
    
    #[test]
    fn test_extension_creation() {
        let ext = Extension::new("AdBlock".into(), "1.0.0".into(), ExtensionKind::Extension);
        assert_eq!(ext.name, "AdBlock".to_string());
        assert_eq!(ext.version, "1.0.0".to_string());
        assert_eq!(ext.state, ExtensionState::Enabled);
    }
    
    #[test]
    fn test_extension_toggle() {
        let mut ext = Extension::new("DarkMode".into(), "2.0".into(), ExtensionKind::Extension);
        ext.disable();
        assert_eq!(ext.state, ExtensionState::Disabled);
        ext.enable();
        assert_eq!(ext.state, ExtensionState::Enabled);
    }
    
    #[test]
    fn test_extension_permissions() {
        let mut ext = Extension::new("Tool".into(), "1.0".into(), ExtensionKind::Extension);
        ext.add_permission("tabs");
        ext.add_permission("storage");
        ext.add_permission("tabs"); // duplicate
        
        assert_eq!(ext.info.permissions.len(), 2);
        assert!(ext.has_permission("tabs"));
        assert!(!ext.has_permission("cookies"));
    }
    
    #[test]
    fn test_manager_creation() {
        let mut manager = ChromiumManager::new();
        assert!(manager.list_windows().is_empty());
        assert!(manager.list_history().is_empty());
        assert!(manager.list_bookmarks().is_empty());
    }
    
    #[test]
    fn test_new_window() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let id = manager.new_window();
        assert!(manager.get_window(&id).is_some());
    }
    
    #[test]
    fn test_new_tab() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let win_id = manager.new_window();
        let tab_id = manager.new_tab(&win_id, "https://example.com".into());
        assert!(manager.get_tab(&tab_id).is_some());
    }
    
    #[test]
    fn test_close_tab() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let win_id = manager.new_window();
        let tab_id = manager.new_tab(&win_id, "https://a.com".into());
        
        assert!(manager.close_tab(&tab_id));
    }
    
    #[test]
    fn test_add_history() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let id = manager.add_history("https://visited.com".into(), HistoryKind::Visit);
        assert!(manager.get_history(&id).is_some());
    }
    
    #[test]
    fn test_add_bookmark() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let id = manager.add_bookmark("Fav".into(), "https://fav.com".into());
        assert!(manager.get_bookmark(&id).is_some());
    }
    
    #[test]
    fn test_download_file() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let id = manager.download_file("file.zip".into(), "https://dl.com/f.zip".into(), 1024);
        assert!(manager.get_download(&id).is_some());
    }
    
    #[test]
    fn test_install_extension() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        let id = manager.install_extension("uBlock".into(), "1.0".into(), ExtensionKind::Extension);
        assert!(manager.get_extension(&id).is_some());
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        manager.new_window();
        manager.add_bookmark("B".into(), "https://b.com".into());
        manager.add_history("https://h.com".into(), HistoryKind::Visit);
        
        let export = manager.export_data();
        let mut manager2 = ChromiumManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_bookmarks().len(), 1);
        assert_eq!(manager2.list_history().len(), 1);
    }
    
    #[test]
    fn test_manager_statistics() {
        let mut manager = ChromiumManager::new();
        manager.initialize();
        manager.new_window();
        manager.new_window();
        manager.add_bookmark("b".into(), "https://b.com".into());
        manager.add_history("https://h.com".into(), HistoryKind::Visit);
        manager.install_extension("e".into(), "1.0".into(), ExtensionKind::Extension);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_windows, 2);
        assert_eq!(stats.total_bookmarks, 1);
        assert_eq!(stats.total_extensions, 1);
    }
}
