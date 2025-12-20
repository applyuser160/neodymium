use std::collections::HashMap;
use std::sync::Arc;

/// A pluggable rendering engine that can be swapped at runtime.
pub trait RenderingEngine: Send + Sync {
    fn name(&self) -> &str;

    #[allow(dead_code)]
    fn render(&self, url: &str) -> String;
}


pub struct HttpEngine {
    client: reqwest::blocking::Client,
}

impl HttpEngine {
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        Self { client }
    }
}

impl Default for HttpEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderingEngine for HttpEngine {
    fn name(&self) -> &str {
        "HTTP Engine"
    }

    fn render(&self, url: &str) -> String {
        match self.client.get(url).send() {
            Ok(response) => match response.text() {
                Ok(text) => text,
                Err(e) => format!("Error reading body: {}", e),
            },
            Err(e) => format!("Error connecting: {}", e),
        }
    }
}

/// A pluggable JavaScript engine that can be swapped at runtime.
pub trait ScriptEngine: Send + Sync {
    fn name(&self) -> &str;

    #[allow(dead_code)]
    fn execute(&self, script: &str) -> String;
}

/// Tracks available rendering and script engines to keep the UI decoupled from the backend.
#[derive(Default)]
pub struct EngineRegistry {
    rendering_engines: HashMap<String, Arc<dyn RenderingEngine>>,
    script_engines: HashMap<String, Arc<dyn ScriptEngine>>,
    active_rendering_engine: Option<String>,
    active_script_engine: Option<String>,
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_rendering_engine<E>(&mut self, engine: E)
    where
        E: RenderingEngine + 'static,
    {
        let name = engine.name().to_string();
        self.rendering_engines
            .insert(name.clone(), Arc::new(engine));
        self.active_rendering_engine.get_or_insert(name);
    }

    pub fn register_script_engine<E>(&mut self, engine: E)
    where
        E: ScriptEngine + 'static,
    {
        let name = engine.name().to_string();
        self.script_engines.insert(name.clone(), Arc::new(engine));
        self.active_script_engine.get_or_insert(name);
    }

    #[allow(dead_code)]
    pub fn set_active_rendering_engine(&mut self, name: &str) -> bool {
        if self.rendering_engines.contains_key(name) {
            self.active_rendering_engine = Some(name.to_string());
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn set_active_script_engine(&mut self, name: &str) -> bool {
        if self.script_engines.contains_key(name) {
            self.active_script_engine = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn active_rendering_engine(&self) -> Option<&str> {
        self.active_rendering_engine.as_deref()
    }

    pub fn active_script_engine(&self) -> Option<&str> {
        self.active_script_engine.as_deref()
    }

    #[allow(dead_code)]
    pub fn rendering_engines(&self) -> impl Iterator<Item = &str> {
        self.rendering_engines.keys().map(|name| name.as_str())
    }

    #[allow(dead_code)]
    pub fn script_engines(&self) -> impl Iterator<Item = &str> {
        self.script_engines.keys().map(|name| name.as_str())
    }
}
