//! Command-line options for chromium.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Incognito mode
    Incognito,
    /// App mode
    App(String),
    /// Kiosk mode
    Kiosk(String),
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// URL to open
    pub url: String,
    /// Incognito mode
    pub incognito: bool,
    /// Kiosk mode
    pub kiosk: bool,
    /// Start URL
    pub start_url: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            url: String::new(),
            incognito: false,
            kiosk: false,
            start_url: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}
