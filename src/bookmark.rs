//! Bookmark definitions for chromium.

/// Bookmark state
#[derive(Debug, Clone, PartialEq)]
pub enum BookmarkState {
    /// Active bookmark
    Active,
    /// Deleted bookmark
    Deleted,
}

/// Bookmark info
#[derive(Debug, Clone, Default)]
pub struct BookmarkInfo {
    /// Description
    pub description: String,
    /// Tags
    pub tags: Vec<String>,
    /// Folder ID
    pub folder_id: Option<String>,
    /// Created by
    pub created_by: String,
}

/// Bookmark
#[derive(Debug, Clone)]
pub struct Bookmark {
    /// Bookmark ID
    pub id: String,
    /// Bookmark name
    pub name: String,
    /// Bookmark URL
    pub url: String,
    /// Bookmark state
    pub state: BookmarkState,
    /// Bookmark info
    pub info: BookmarkInfo,
    /// Created timestamp
    pub created_at: u64,
    /// Last modified timestamp
    pub last_modified: u64,
}

impl Bookmark {
    /// Create new bookmark
    pub fn new(name: String, url: String) -> Self {
        Self {
            id: format!("bookmark-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            url,
            state: BookmarkState::Active,
            info: BookmarkInfo::default(),
            created_at: Self::now(),
            last_modified: Self::now(),
        }
    }
    
    /// Set description
    pub fn set_description(&mut self, description: String) {
        self.info.description = description;
        self.last_modified = Self::now();
    }
    
    /// Add tag
    pub fn add_tag(&mut self, tag: String) -> bool {
        if !self.info.tags.contains(&tag) {
            self.info.tags.push(tag);
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Remove tag
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        if let Some(pos) = self.info.tags.iter().position(|t| t == tag) {
            self.info.tags.remove(pos);
            self.last_modified = Self::now();
            return true;
        }
        false
    }
    
    /// Check if has tag
    pub fn has_tag(&self, tag: &str) -> bool {
        self.info.tags.contains(&tag.to_string())
    }
    
    /// Set folder
    pub fn set_folder(&mut self, folder_id: String) {
        self.info.folder_id = Some(folder_id);
        self.last_modified = Self::now();
    }
    
    /// Delete bookmark
    pub fn delete(&mut self) {
        self.state = BookmarkState::Deleted;
        self.last_modified = Self::now();
    }
    
    /// Check if active
    pub fn is_active(&self) -> bool {
        self.state == BookmarkState::Active
    }
    
    /// Get bookmark age in seconds
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

/// Bookmark folder
#[derive(Debug, Clone)]
pub struct BookmarkFolder {
    /// Folder ID
    pub id: String,
    /// Folder name
    pub name: String,
    /// Parent folder ID
    pub parent_id: Option<String>,
    /// Bookmarks
    pub bookmarks: Vec<Bookmark>,
    /// Subfolders
    pub subfolders: Vec<BookmarkFolder>,
    /// Created timestamp
    pub created_at: u64,
}

impl BookmarkFolder {
    /// Create new bookmark folder
    pub fn new(name: String) -> Self {
        Self {
            id: format!("folder-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            name,
            parent_id: None,
            bookmarks: vec![],
            subfolders: vec![],
            created_at: Self::now(),
        }
    }
    
    /// Add bookmark
    pub fn add_bookmark(&mut self, bookmark: Bookmark) {
        self.bookmarks.push(bookmark);
    }
    
    /// Remove bookmark
    pub fn remove_bookmark(&mut self, bookmark_id: &str) -> bool {
        if let Some(pos) = self.bookmarks.iter().position(|b| b.id == bookmark_id) {
            self.bookmarks.remove(pos);
            return true;
        }
        false
    }
    
    /// Get bookmark count
    pub fn get_bookmark_count(&self) -> usize {
        self.bookmarks.len() + self.subfolders.iter()
            .map(|f| f.get_bookmark_count())
            .sum::<usize>()
    }
    
    /// Get all bookmarks recursively
    pub fn get_all_bookmarks(&self) -> Vec<&Bookmark> {
        let mut result = self.bookmarks.iter().collect::<Vec<_>>();
        for subfolder in &self.subfolders {
            result.extend(subfolder.get_all_bookmarks());
        }
        result
    }
    
    /// Add subfolder
    pub fn add_subfolder(&mut self, folder: BookmarkFolder) {
        self.subfolders.push(folder);
    }
    
    /// Get folder age in seconds
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
