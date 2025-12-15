use Neodymium::{
    BrowserModel, EngineRegistry, MockRenderingEngine, MockScriptEngine, RenderingEngine, ScriptEngine,
};

struct EchoRenderer;
impl RenderingEngine for EchoRenderer {
    fn name(&self) -> &str {
        "Echo"
    }

    fn render(&self, url: &str) -> Neodymium::RenderOutput {
        Neodymium::RenderOutput {
            summary: format!("echo:{url}"),
        }
    }
}

struct CountingScript;
impl ScriptEngine for CountingScript {
    fn name(&self) -> &str {
        "Counter"
    }

    fn execute(&self, script: &str) -> Neodymium::ScriptResult {
        Neodymium::ScriptResult {
            description: format!("{} chars", script.len()),
        }
    }
}

#[test]
fn can_register_multiple_engines() {
    let mut registry = EngineRegistry::default();
    registry.register_rendering_engine(EchoRenderer);
    registry.register_rendering_engine(MockRenderingEngine::default());
    registry.register_script_engine(CountingScript);
    registry.register_script_engine(MockScriptEngine::default());

    let renderers = registry.available_rendering_engines();
    assert!(renderers.contains(&"Echo".to_string()));
    assert!(renderers.contains(&"Vector".to_string()));

    let scripts = registry.available_script_engines();
    assert!(scripts.contains(&"Counter".to_string()));
    assert!(scripts.contains(&"Mercury JS".to_string()));
}

#[test]
fn browser_model_tracks_tabs() {
    let mut model = BrowserModel::new();
    model.add_tab("New Tab", "https://chromium.org");
    model.set_active_status("GPU surface ready");

    assert_eq!(model.tabs.len(), 2);
    assert_eq!(model.active_tab, 1);
    assert_eq!(model.render_status.as_deref(), Some("GPU surface ready"));
}
