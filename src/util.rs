//! Utility model for chromium (legacy compat).

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

    pub fn add_tab(&mut self, url: &str) {
        self.tabs.push(Tab { title: String::new(), url: url.into(), active: self.tabs.is_empty() });
    }

    pub fn list_tabs(&self) -> &[Tab] { &self.tabs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_model() {
        let mut m = Model::new();
        m.add_tab("https://example.com");
        assert_eq!(m.tabs.len(), 1);
    }
}
