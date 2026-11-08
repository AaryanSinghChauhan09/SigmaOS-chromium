//! Download definitions for chromium.

/// Download state
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadState {
    /// Pending download
    Pending,
    /// In progress download
    InProgress,
    /// Paused download
    Paused,
    /// Completed download
    Completed,
    /// Failed download
    Failed,
    /// Cancelled download
    Cancelled,
}

/// Download kind
#[derive(Debug, Clone, PartialEq)]
pub enum DownloadKind {
    /// File download
    File,
    /// Page download
    Page,
    /// Image download
    Image,
}

/// Download info
#[derive(Debug, Clone, Default)]
pub struct DownloadInfo {
    /// Total size in bytes
    pub total_size: u64,
    /// Downloaded size in bytes
    pub downloaded_size: u64,
    /// Speed in bytes/sec
    pub speed_bps: u64,
    /// Saved path
    pub saved_path: Option<String>,
    /// MIME type
    pub mime_type: String,
}

/// Download
#[derive(Debug, Clone)]
pub struct Download {
    /// Download ID
    pub id: String,
    /// Filename
    pub filename: String,
    /// URL
    pub url: String,
    /// Download state
    pub state: DownloadState,
    /// Download kind
    pub kind: DownloadKind,
    /// Download info
    pub info: DownloadInfo,
    /// Error message
    pub error: Option<String>,
    /// Created timestamp
    pub created_at: u64,
    /// Started timestamp
    pub started_at: Option<u64>,
    /// Finished timestamp
    pub finished_at: Option<u64>,
}

impl Download {
    /// Create new download
    pub fn new(filename: String, url: String, total_size: u64) -> Self {
        Self {
            id: format!("download-{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()),
            filename,
            url,
            state: DownloadState::Pending,
            kind: DownloadKind::File,
            info: DownloadInfo {
                total_size,
                ..Default::default()
            },
            error: None,
            created_at: Self::now(),
            started_at: None,
            finished_at: None,
        }
    }
    
    /// Start download
    pub fn start(&mut self) {
        self.state = DownloadState::InProgress;
        self.started_at = Some(Self::now());
    }
    
    /// Set progress
    pub fn set_progress(&mut self, downloaded_size: u64) {
        self.info.downloaded_size = downloaded_size.min(self.info.total_size);
    }
    
    /// Set speed
    pub fn set_speed(&mut self, speed_bps: u64) {
        self.info.speed_bps = speed_bps;
    }
    
    /// Pause download
    pub fn pause(&mut self) {
        if self.state == DownloadState::InProgress {
            self.state = DownloadState::Paused;
        }
    }
    
    /// Resume download
    pub fn resume(&mut self) {
        if self.state == DownloadState::Paused {
            self.state = DownloadState::InProgress;
        }
    }
    
    /// Complete download
    pub fn complete(&mut self, saved_path: String) {
        self.state = DownloadState::Completed;
        self.info.saved_path = Some(saved_path);
        self.info.downloaded_size = self.info.total_size;
        self.finished_at = Some(Self::now());
    }
    
    /// Fail download
    pub fn fail(&mut self, error: String) {
        self.state = DownloadState::Failed;
        self.error = Some(error);
        self.finished_at = Some(Self::now());
    }
    
    /// Cancel download
    pub fn cancel(&mut self) {
        self.state = DownloadState::Cancelled;
        self.finished_at = Some(Self::now());
    }
    
    /// Get progress percent
    pub fn get_progress_percent(&self) -> f64 {
        if self.info.total_size == 0 {
            return 100.0;
        }
        (self.info.downloaded_size as f64 / self.info.total_size as f64) * 100.0
    }
    
    /// Get download duration in seconds
    pub fn get_duration_seconds(&self) -> u64 {
        if let (Some(start), Some(end)) = (self.started_at, self.finished_at) {
            end - start
        } else {
            0
        }
    }
    
    /// Get download age in seconds
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
