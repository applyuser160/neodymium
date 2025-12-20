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

    pub fn close_tab(&mut self, index: usize) {
        if index >= self.tabs.len() {
            return;
        }
        self.tabs.remove(index);

        if self.tabs.is_empty() {
            self.active_index = 0;
            // Ensure there is always at least one tab
            self.open_tab("New Tab", "about:blank");
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

    pub fn close_active_tab(&mut self) {
        self.close_tab(self.active_index);
    }

    pub fn active(&self) -> Option<&Tab> {
        self.tabs.get(self.active_index)
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        // Should create a new empty tab
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_index, 0);
        assert_eq!(manager.tabs()[0].title, "New Tab");
        assert_eq!(manager.tabs()[0].url, "about:blank");
    }

    #[test]
    fn test_close_active_tab() {
        let mut manager = TabManager::default();
        manager.open_tab("Tab 1", "url1");
        manager.switch_to(1);

        assert_eq!(manager.active_index, 1);
        assert_eq!(manager.tabs().len(), 2);

        manager.close_active_tab();
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_index, 0);
        assert_eq!(manager.tabs()[0].title, "New Tab");

        // Test closing the last remaining tab
        manager.close_active_tab();
        assert_eq!(manager.tabs().len(), 1);
        assert_eq!(manager.active_index, 0);
        assert_eq!(manager.tabs()[0].title, "New Tab");
    }
}
