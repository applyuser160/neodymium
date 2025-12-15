use std::collections::HashMap;
use std::sync::Arc;

/// A rendering engine that can draw pages into the UI shell.
pub trait RenderingEngine: Send + Sync {
    /// Human readable name to display in the UI.
    fn name(&self) -> &str;

    /// Render a URL and provide a small status summary.
    fn render(&self, url: &str) -> RenderOutput;
}

/// A script engine that can evaluate inline JavaScript for the currently loaded page.
pub trait ScriptEngine: Send + Sync {
    /// Human readable name to display in the UI.
    fn name(&self) -> &str;

    /// Execute a script and return the result description.
    fn execute(&self, script: &str) -> ScriptResult;
}

/// A tiny placeholder for information a renderer would normally stream back to the compositor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderOutput {
    pub summary: String,
}

/// Describes the JavaScript work that was performed for a page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptResult {
    pub description: String,
}

/// Registry that keeps a set of pluggable engines and tracks which ones are active.
#[derive(Default)]
pub struct EngineRegistry {
    rendering_engines: HashMap<String, Arc<dyn RenderingEngine>>, 
    script_engines: HashMap<String, Arc<dyn ScriptEngine>>, 
    active_rendering: Option<String>,
    active_script: Option<String>,
}

impl EngineRegistry {
    pub fn register_rendering_engine<E: RenderingEngine + 'static>(
        &mut self,
        engine: E,
    ) {
        let name = engine.name().to_owned();
        self.rendering_engines.insert(name.clone(), Arc::new(engine));
        self.active_rendering.get_or_insert(name);
    }

    pub fn register_script_engine<E: ScriptEngine + 'static>(
        &mut self,
        engine: E,
    ) {
        let name = engine.name().to_owned();
        self.script_engines.insert(name.clone(), Arc::new(engine));
        self.active_script.get_or_insert(name);
    }

    pub fn available_rendering_engines(&self) -> Vec<String> {
        self.rendering_engines.keys().cloned().collect()
    }

    pub fn available_script_engines(&self) -> Vec<String> {
        self.script_engines.keys().cloned().collect()
    }

    pub fn set_rendering_engine(&mut self, name: &str) -> bool {
        if self.rendering_engines.contains_key(name) {
            self.active_rendering = Some(name.to_owned());
            true
        } else {
            false
        }
    }

    pub fn set_script_engine(&mut self, name: &str) -> bool {
        if self.script_engines.contains_key(name) {
            self.active_script = Some(name.to_owned());
            true
        } else {
            false
        }
    }

    pub fn active_rendering_engine(&self) -> Option<Arc<dyn RenderingEngine>> {
        self.active_rendering
            .as_ref()
            .and_then(|name| self.rendering_engines.get(name))
            .cloned()
    }

    pub fn active_script_engine(&self) -> Option<Arc<dyn ScriptEngine>> {
        self.active_script
            .as_ref()
            .and_then(|name| self.script_engines.get(name))
            .cloned()
    }

    pub fn render_demo(&self, url: &str) -> Option<(RenderOutput, ScriptResult)> {
        let renderer = self.active_rendering_engine()?;
        let script = self.active_script_engine()?;

        let render_output = renderer.render(url);
        let script_output = script.execute("console.log('page ready')");

        Some((render_output, script_output))
    }
}

/// A lightweight rendering engine used for demo purposes.
#[derive(Default)]
pub struct MockRenderingEngine;

impl RenderingEngine for MockRenderingEngine {
    fn name(&self) -> &str {
        "Vector"
    }

    fn render(&self, url: &str) -> RenderOutput {
        RenderOutput {
            summary: format!("Rendering {url} via GPU surfaces"),
        }
    }
}

/// A simple script engine that pretends to execute JavaScript.
#[derive(Default)]
pub struct MockScriptEngine;

impl ScriptEngine for MockScriptEngine {
    fn name(&self) -> &str {
        "Mercury JS"
    }

    fn execute(&self, script: &str) -> ScriptResult {
        ScriptResult {
            description: format!("Evaluated script: {script}"),
        }
    }
}

/// High level UI state shared across the UI and tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserTab {
    pub title: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserModel {
    pub tabs: Vec<BrowserTab>,
    pub active_tab: usize,
    pub render_status: Option<String>,
}

impl BrowserModel {
    pub fn new() -> Self {
        Self {
            tabs: vec![BrowserTab {
                title: "New Tab".into(),
                url: "neodymium://start".into(),
            }],
            active_tab: 0,
            render_status: None,
        }
    }

    pub fn set_active_status(&mut self, status: impl Into<String>) {
        self.render_status = Some(status.into());
    }

    pub fn add_tab(&mut self, title: impl Into<String>, url: impl Into<String>) {
        self.tabs.push(BrowserTab {
            title: title.into(),
            url: url.into(),
        });
        self.active_tab = self.tabs.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct UppercaseRenderer;
    impl RenderingEngine for UppercaseRenderer {
        fn name(&self) -> &str {
            "Upper"
        }

        fn render(&self, url: &str) -> RenderOutput {
            RenderOutput {
                summary: url.to_uppercase(),
            }
        }
    }

    struct LowerScript;
    impl ScriptEngine for LowerScript {
        fn name(&self) -> &str {
            "Lower"
        }

        fn execute(&self, script: &str) -> ScriptResult {
            ScriptResult {
                description: script.to_lowercase(),
            }
        }
    }

    #[test]
    fn tracks_active_engines_and_executes_them() {
        let mut registry = EngineRegistry::default();
        registry.register_rendering_engine(UppercaseRenderer);
        registry.register_script_engine(LowerScript);

        let (render, script) = registry.render_demo("https://example.com").unwrap();
        assert_eq!(render.summary, "HTTPS://EXAMPLE.COM");
        assert_eq!(script.description, "console.log('page ready')");
    }

    #[test]
    fn switches_engines_by_name() {
        let mut registry = EngineRegistry::default();
        registry.register_rendering_engine(UppercaseRenderer);
        registry.register_rendering_engine(MockRenderingEngine::default());
        registry.register_script_engine(LowerScript);
        registry.register_script_engine(MockScriptEngine::default());

        assert!(registry.set_rendering_engine("Vector"));
        assert!(registry.set_script_engine("Mercury JS"));

        let (render, script) = registry.render_demo("neodymium://status").unwrap();
        assert_eq!(render.summary, "Rendering neodymium://status via GPU surfaces");
        assert_eq!(script.description, "Evaluated script: console.log('page ready')");
    }
}
