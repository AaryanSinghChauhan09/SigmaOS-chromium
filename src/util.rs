//! chromium model — web browser.

#[derive(Debug, Clone)]
pub struct Tab {
    pub title: String,
    pub url: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub tabs: Vec<Tab>,
}

impl Default for Model {
    fn default() -> Self { Self { tabs: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn open(&mut self, title: &str, url: &str) {
        self.tabs.push(Tab { title: title.into(), url: url.into(), active: true });
    }

    pub fn list_tabs(&self) -> &[Tab] { &self.tabs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_tab() {
        let mut c = Model::new();
        c.open("Example", "https://example.com");
        assert_eq!(c.list_tabs().len(), 1);
    }
}
