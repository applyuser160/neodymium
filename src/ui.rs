use crate::engines::EngineRegistry;
use crate::tabs::TabManager;
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb};
use gpui_component::StyledExt;
use gpui_component::button::{Button, ButtonVariants};

/// The top-level view for the Chrome-inspired browser UI.
pub struct BrowserView {
    tabs: TabManager,
    engines: EngineRegistry,
    address_bar: String,
}

impl Default for BrowserView {
    fn default() -> Self {
        let mut engines = EngineRegistry::new();
        engines.register_rendering_engine(BuiltinEngine {
            name: "Blink".to_string(),
            kind: EngineKind::Rendering,
        });
        engines.register_rendering_engine(BuiltinEngine {
            name: "WebKit".to_string(),
            kind: EngineKind::Rendering,
        });
        engines.register_script_engine(BuiltinEngine {
            name: "V8".to_string(),
            kind: EngineKind::Script,
        });
        engines.register_script_engine(BuiltinEngine {
            name: "SpiderMonkey".to_string(),
            kind: EngineKind::Script,
        });

        Self {
            tabs: TabManager::default(),
            engines,
            address_bar: "https://example.com".to_string(),
        }
    }
}

impl Render for BrowserView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x181a1b))
            .text_color(rgb(0xf0f0f0))
            .v_flex()
            .child(self.render_tab_strip())
            .child(self.render_toolbar())
            .child(self.render_content())
    }
}

impl BrowserView {
    fn render_tab_strip(&self) -> impl IntoElement {
        let mut strip = div()
            .bg(rgb(0x292b2c))
            .h_flex()
            .items_center()
            .gap_2()
            .px(px(12.))
            .py(px(8.));

        for (index, tab) in self.tabs.tabs().iter().enumerate() {
            let is_active = Some(tab) == self.tabs.active();
            let tab_button = Button::new(("tab", index as u32)).label(tab.title.clone());
            let tab_button = if is_active {
                tab_button.primary()
            } else {
                tab_button.ghost()
            }
            .px(px(12.))
            .py(px(6.));

            strip = strip.child(tab_button);

            if index == self.tabs.tabs().len() - 1 {
                strip = strip.child(
                    Button::new("new-tab")
                        .label("+")
                        .ghost()
                        .rounded_full()
                        .px(px(8.))
                        .py(px(4.)),
                );
            }
        }

        strip
    }

    fn render_toolbar(&self) -> impl IntoElement {
        div()
            .bg(rgb(0x1f2122))
            .h_flex()
            .items_center()
            .gap_2()
            .px(px(12.))
            .py(px(10.))
            .child(
                Button::new("back")
                    .label("←")
                    .ghost()
                    .rounded_sm()
                    .px(px(10.))
                    .py(px(6.)),
            )
            .child(
                Button::new("forward")
                    .label("→")
                    .ghost()
                    .rounded_sm()
                    .px(px(10.))
                    .py(px(6.)),
            )
            .child(
                Button::new("refresh")
                    .label("⟳")
                    .ghost()
                    .rounded_sm()
                    .px(px(10.))
                    .py(px(6.)),
            )
            .child(
                div()
                    .bg(rgb(0xffffff))
                    .text_color(rgb(0x222222))
                    .rounded_full()
                    .px(px(14.))
                    .py(px(8.))
                    .flex_grow()
                    .child(self.address_bar.clone()),
            )
            .child(
                Button::new("menu")
                    .label("⋮")
                    .ghost()
                    .rounded_sm()
                    .px(px(10.))
                    .py(px(6.)),
            )
    }

    fn render_content(&self) -> impl IntoElement {
        let active_render = self
            .engines
            .active_rendering_engine()
            .unwrap_or("No rendering engine");
        let active_script = self
            .engines
            .active_script_engine()
            .unwrap_or("No script engine");

        div()
            .v_flex()
            .gap_2()
            .p_6()
            .child(
                div()
                    .text_lg()
                    .child(format!("Active rendering engine: {}", active_render)),
            )
            .child(
                div()
                    .text_lg()
                    .child(format!("Active JavaScript engine: {}", active_script)),
            )
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0xc8c8c8))
                    .child("Content would be rendered here once a backend is connected."),
            )
    }
}

#[derive(Clone)]
enum EngineKind {
    Rendering,
    Script,
}

#[derive(Clone)]
struct BuiltinEngine {
    name: String,
    #[allow(dead_code)]
    kind: EngineKind,
}

impl crate::engines::RenderingEngine for BuiltinEngine {
    fn name(&self) -> &str {
        &self.name
    }

    fn render(&self, url: &str) -> String {
        format!("[{}] rendering {}", self.name, url)
    }
}

impl crate::engines::ScriptEngine for BuiltinEngine {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&self, script: &str) -> String {
        match self.kind {
            EngineKind::Rendering => format!("{} cannot execute scripts", self.name),
            EngineKind::Script => format!("[{}] executed {}", self.name, script),
        }
    }
}
