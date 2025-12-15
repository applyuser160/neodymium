#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub url: String,
}

pub struct BrowserState {
    tabs: Vec<Tab>,
    active_tab_index: usize,
}

impl BrowserState {
    pub fn new(home_url: impl Into<String>) -> Self {
        let initial_tab = Tab {
            id: 1,
            title: "New Tab".to_string(),
            url: home_url.into(),
        };
        Self {
            tabs: vec![initial_tab],
            active_tab_index: 0,
        }
    }

    pub fn tabs(&self) -> &[Tab] {
        &self.tabs
    }

    pub fn active_tab(&self) -> &Tab {
        &self.tabs[self.active_tab_index]
    }

    pub fn open_tab(&mut self, title: impl Into<String>, url: impl Into<String>) -> &Tab {
        let id = self.tabs.last().map(|tab| tab.id + 1).unwrap_or(1);
        let tab = Tab {
            id,
            title: title.into(),
            url: url.into(),
        };
        self.tabs.push(tab);
        self.active_tab_index = self.tabs.len() - 1;
        self.active_tab()
    }

    pub fn switch_tab(&mut self, tab_id: usize) {
        if let Some((index, _)) = self
            .tabs
            .iter()
            .enumerate()
            .find(|(_, tab)| tab.id == tab_id)
        {
            self.active_tab_index = index;
        }
    }

    pub fn navigate(&mut self, url: impl Into<String>) {
        if let Some(active) = self.tabs.get_mut(self.active_tab_index) {
            active.url = url.into();
        }
    }
}
