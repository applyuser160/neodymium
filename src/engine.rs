pub trait RenderingEngine: Send + Sync {
    fn name(&self) -> &str;
    fn render(&self, url: &str) -> String;
}

pub trait JavaScriptEngine: Send + Sync {
    fn name(&self) -> &str;
    fn evaluate(&self, script: &str) -> String;
}

pub struct EngineRegistry {
    rendering_engine: Box<dyn RenderingEngine>,
    javascript_engine: Box<dyn JavaScriptEngine>,
}

impl EngineRegistry {
    pub fn new(
        rendering_engine: Box<dyn RenderingEngine>,
        javascript_engine: Box<dyn JavaScriptEngine>,
    ) -> Self {
        Self {
            rendering_engine,
            javascript_engine,
        }
    }

    pub fn rendering_engine_name(&self) -> &str {
        self.rendering_engine.name()
    }

    pub fn javascript_engine_name(&self) -> &str {
        self.javascript_engine.name()
    }

    pub fn set_rendering_engine(&mut self, rendering_engine: Box<dyn RenderingEngine>) {
        self.rendering_engine = rendering_engine;
    }

    pub fn set_javascript_engine(&mut self, javascript_engine: Box<dyn JavaScriptEngine>) {
        self.javascript_engine = javascript_engine;
    }

    pub fn render_page(&self, url: &str) -> String {
        self.rendering_engine.render(url)
    }

    pub fn evaluate_script(&self, script: &str) -> String {
        self.javascript_engine.evaluate(script)
    }
}

#[derive(Clone)]
pub struct MockRenderingEngine {
    name: String,
}

impl MockRenderingEngine {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl RenderingEngine for MockRenderingEngine {
    fn name(&self) -> &str {
        &self.name
    }

    fn render(&self, url: &str) -> String {
        format!("[{}] rendering {}", self.name, url)
    }
}

#[derive(Clone)]
pub struct MockJavaScriptEngine {
    name: String,
}

impl MockJavaScriptEngine {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl JavaScriptEngine for MockJavaScriptEngine {
    fn name(&self) -> &str {
        &self.name
    }

    fn evaluate(&self, script: &str) -> String {
        format!("[{}] eval {}", self.name, script)
    }
}
