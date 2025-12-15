use neodymium::engines::{EngineRegistry, RenderingEngine, ScriptEngine};
use neodymium::tabs::{Tab, TabManager};

struct MockRenderEngine(&'static str);
struct MockScriptEngine(&'static str);

impl RenderingEngine for MockRenderEngine {
    fn name(&self) -> &str {
        self.0
    }

    fn render(&self, url: &str) -> String {
        format!("{} rendering {}", self.0, url)
    }
}

impl ScriptEngine for MockScriptEngine {
    fn name(&self) -> &str {
        self.0
    }

    fn execute(&self, script: &str) -> String {
        format!("{} executed {}", self.0, script)
    }
}

#[test]
fn engine_registry_supports_pluggable_engines() {
    let mut registry = EngineRegistry::new();
    registry.register_rendering_engine(MockRenderEngine("Blink"));
    registry.register_rendering_engine(MockRenderEngine("WebKit"));
    registry.register_script_engine(MockScriptEngine("V8"));
    registry.register_script_engine(MockScriptEngine("SpiderMonkey"));

    assert_eq!(registry.active_rendering_engine(), Some("Blink"));
    assert_eq!(registry.active_script_engine(), Some("V8"));

    assert!(registry.set_active_rendering_engine("WebKit"));
    assert!(registry.set_active_script_engine("SpiderMonkey"));
    assert_eq!(registry.active_rendering_engine(), Some("WebKit"));
    assert_eq!(registry.active_script_engine(), Some("SpiderMonkey"));

    let render_engines: Vec<_> = registry.rendering_engines().collect();
    assert_eq!(render_engines, vec!["Blink", "WebKit"]);
}

#[test]
fn tab_manager_tracks_active_tabs() {
    let mut tabs = TabManager::new();
    assert_eq!(tabs.active(), Some(&Tab::new("New Tab", "about:blank")));

    let docs_index = tabs.open_tab("Docs", "https://developer.chrome.com");
    assert_eq!(docs_index, 1);
    assert_eq!(tabs.active().map(|tab| &tab.title), Some(&"Docs".to_string()));

    assert!(tabs.switch_to(0));
    assert_eq!(tabs.active(), Some(&Tab::new("New Tab", "about:blank")));
    assert!(!tabs.switch_to(42));
}
