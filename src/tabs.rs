/// Represents a single browser tab.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tab {
    pub title: String,
    pub url: String,
}

impl Tab {
    pub fn new<T: Into<String>, U: Into<String>>(title: T, url: U) -> Self {
        Self {
            title: title.into(),
            url: url.into(),
        }
    }
}

/// Tracks tab order and active selection without depending on a backend.
#[derive(Clone, Debug)]
pub struct TabManager {
    tabs: Vec<Tab>,
    active_index: usize,
}

impl Default for TabManager {
    fn default() -> Self {
        Self {
            tabs: vec![Tab::new("New Tab", "about:blank")],
            active_index: 0,
        }
    }
}

impl TabManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn open_tab<T: Into<String>, U: Into<String>>(&mut self, title: T, url: U) -> usize {
        let tab = Tab::new(title, url);
        self.tabs.push(tab);
        self.active_index = self.tabs.len() - 1;
        self.active_index
    }

    #[allow(dead_code)]
    pub fn switch_to(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_index = index;
            true
        } else {
            false
        }
    }

    pub fn active(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }
}
