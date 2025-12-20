/// Represents a single browser tab.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tab {
    pub title: String,
    history: Vec<String>,
    current_index: usize,
}

impl Tab {
    pub fn new<T: Into<String>, U: Into<String>>(title: T, url: U) -> Self {
        Self {
            title: title.into(),
            history: vec![url.into()],
            current_index: 0,
        }
    }

    pub fn url(&self) -> &str {
        &self.history[self.current_index]
    }

    pub fn navigate<U: Into<String>>(&mut self, url: U) {
        // Truncate history if we are not at the end
        if self.current_index < self.history.len() - 1 {
            self.history.truncate(self.current_index + 1);
        }
        self.history.push(url.into());
        self.current_index = self.history.len() - 1;
    }

    pub fn go_back(&mut self) -> bool {
        if self.current_index > 0 {
            self.current_index -= 1;
            true
        } else {
            false
        }
    }

    pub fn go_forward(&mut self) -> bool {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            true
        } else {
            false
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.current_index < self.history.len() - 1
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

    pub fn close_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }
        self.tabs.remove(index);

        if self.tabs.is_empty() {
            self.active_index = 0;
            return;
        }

        if self.active_index > index {
            self.active_index -= 1;
        } else if self.active_index == index && self.active_index >= self.tabs.len() {
            self.active_index = self.tabs.len() - 1;
        }
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

    pub fn active_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_index)
    }

    pub fn navigate(&mut self, url: impl Into<String>) {
        if let Some(tab) = self.tabs.get_mut(self.active_index) {
            tab.url = url.into();
        }
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_history() {
        let mut tab = Tab::new("Test", "url1");
        assert_eq!(tab.url(), "url1");
        assert!(!tab.can_go_back());
        assert!(!tab.can_go_forward());

        tab.navigate("url2");
        assert_eq!(tab.url(), "url2");
        assert!(tab.can_go_back());
        assert!(!tab.can_go_forward());

        tab.navigate("url3");
        assert_eq!(tab.url(), "url3");

        // Go back
        assert!(tab.go_back());
        assert_eq!(tab.url(), "url2");
        assert!(tab.can_go_forward());

        // Go back again
        assert!(tab.go_back());
        assert_eq!(tab.url(), "url1");
        assert!(!tab.can_go_back());

        // Go forward
        assert!(tab.go_forward());
        assert_eq!(tab.url(), "url2");

        // Navigate from middle (should truncate "url3")
        tab.navigate("url4");
        assert_eq!(tab.url(), "url4");
        assert!(!tab.can_go_forward());

        // Check history structure implicitly by going back
        assert!(tab.go_back());
        assert_eq!(tab.url(), "url2");
    }

    #[test]
    fn test_close_tab() {
        let mut manager = TabManager::default();
        // Default has 1 tab at index 0.

        // Add 2 more tabs
        manager.open_tab("Tab 1", "url1");
        manager.open_tab("Tab 2", "url2");
        // State: [Default(0), Tab 1(1), Tab 2(2)], active = 2

        assert_eq!(manager.tabs().len(), 3);
        assert_eq!(manager.active_index, 2);

        // Close the active tab (index 2)
        manager.close_tab(2);
        // State: [Default(0), Tab 1(1)], active should be 1
        assert_eq!(manager.tabs().len(), 2);
        assert_eq!(manager.active_index, 1);
        assert_eq!(manager.tabs()[1].title, "Tab 1");

        // Close a tab before active (index 0)
        manager.close_tab(0);
        // State: [Tab 1(0)], active was 1, now should be 0
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_index, 0);
        assert_eq!(manager.tabs()[0].title, "Tab 1");

        // Close the last remaining tab
        manager.close_tab(0);
        assert_eq!(manager.tabs().len(), 0);
        assert_eq!(manager.active_index, 0); // Safety check
    }

    #[test]
    fn test_navigate() {
        let mut manager = TabManager::default();
        manager.navigate("https://gpui.rs");
        assert_eq!(manager.active().unwrap().url, "https://gpui.rs");

        manager.open_tab("New Tab", "about:blank");
        manager.navigate("https://google.com");
        assert_eq!(manager.active().unwrap().url, "https://google.com");

        // Verify first tab wasn't changed
        manager.switch_to(0);
        assert_eq!(manager.active().unwrap().url, "https://gpui.rs");
    }
}
