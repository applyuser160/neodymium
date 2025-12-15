use neodymium::engine::{EngineRegistry, MockJavaScriptEngine, MockRenderingEngine};
use neodymium::state::BrowserState;

#[test]
fn tab_management_mimics_browser_flow() {
    let mut state = BrowserState::new("https://google.com");
    assert_eq!(state.active_tab().url, "https://google.com");

    let new_tab_id = state
        .open_tab("Docs", "https://developer.chrome.com")
        .id;
    assert_eq!(state.active_tab().id, new_tab_id);

    state.switch_tab(1);
    assert_eq!(state.active_tab().id, 1);

    state.navigate("https://chromium.org");
    assert_eq!(state.active_tab().url, "https://chromium.org");
}

#[test]
fn engines_are_pluggable() {
    let mut registry = EngineRegistry::new(
        Box::new(MockRenderingEngine::new("Blink-compatible")),
        Box::new(MockJavaScriptEngine::new("V8-compatible")),
    );

    assert!(
        registry
            .render_page("https://example.com")
            .contains("Blink-compatible")
    );
    assert!(
        registry
            .evaluate_script("alert('hi')")
            .contains("V8-compatible")
    );

    registry.set_rendering_engine(Box::new(MockRenderingEngine::new("WebKit")));
    registry.set_javascript_engine(Box::new(MockJavaScriptEngine::new("SpiderMonkey")));

    assert_eq!(registry.rendering_engine_name(), "WebKit");
    assert_eq!(registry.javascript_engine_name(), "SpiderMonkey");
}
