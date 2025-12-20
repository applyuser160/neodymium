use crate::engines::EngineRegistry;
use crate::tabs::TabManager;
use gpui::{
    AppContext, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, px, rgb,
};
use gpui_component::StyledExt;
use gpui_component::Disableable;
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputEvent, InputState};

/// The top-level view for the Chrome-inspired browser UI.
pub struct BrowserView {
    tabs: TabManager,
    engines: EngineRegistry,
    address_bar: Entity<InputState>,
}

impl BrowserView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
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

        let address_bar = cx.new(|cx| {
            InputState::new(window, cx)
                .default_value("https://example.com")
                .placeholder("Search or enter address")
        });

        cx.subscribe_in(
            &address_bar,
            window,
            |view, state, event, _window, cx| {
                if let InputEvent::PressEnter { .. } = event {
                   let url = state.read(cx).value();
                   if let Some(tab) = view.tabs.active_mut() {
                       tab.navigate(url);
                       cx.notify();
                   }
                }
            },
        )
        .detach();

        Self {
            tabs: TabManager::default(),
            engines,
            address_bar,
        }
    }
}

impl Render for BrowserView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x181a1b))
            .text_color(rgb(0xf0f0f0))
            .v_flex()
            .child(self.render_tab_strip(cx))
            .child(self.render_toolbar(cx))
            .child(self.render_content())
    }
}

impl BrowserView {
    fn render_tab_strip(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let mut strip = div()
            .bg(rgb(0x292b2c))
            .h_flex()
            .items_center()
            .gap_2()
            .px(px(12.))
            .py(px(8.));

        for (index, tab) in self.tabs.tabs().iter().enumerate() {
            let is_active = Some(tab) == self.tabs.active();

            // Close button
            let close_btn = Button::new(("close-tab", index))
                .label("×")
                .ghost()
                .rounded_full()
                .px(px(6.))
                .py(px(0.))
                .on_click(cx.listener(move |view: &mut BrowserView, _, _window, cx| {
                    view.tabs.close_tab(index);
                    cx.notify();
                }));

            // Tab container styling
            let mut tab_container = div()
                .id(("tab", index))
                .h_flex()
                .items_center()
                .gap_2()
                .px(px(12.))
                .py(px(6.))
                .rounded_md()
                .cursor_pointer()
                .on_click(cx.listener(move |view: &mut BrowserView, _, _, cx| {
                    view.tabs.switch_to(index);
                    cx.notify();
                }));

            if is_active {
                tab_container = tab_container.bg(rgb(0x3a3d3e)).text_color(rgb(0xffffff));
            } else {
                tab_container = tab_container
                    .text_color(rgb(0xaaaaaa))
                    .hover(|s| s.bg(rgb(0x333536)));
            }

            tab_container = tab_container.child(tab.title.clone()).child(close_btn);

            strip = strip.child(tab_container);
        }

        // Add new tab button
        strip = strip.child(
            Button::new("new-tab")
                .label("+")
                .ghost()
                .rounded_full()
                .px(px(8.))
                .py(px(4.))
                .on_click(cx.listener(|view: &mut BrowserView, _, _, cx| {
                    view.tabs.open_tab("New Tab", "about:blank");
                    cx.notify();
                })),
        );

        strip
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let active_tab = self.tabs.active();
        let can_go_back = active_tab.map_or(false, |t| t.can_go_back());
        let can_go_forward = active_tab.map_or(false, |t| t.can_go_forward());

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
                    .py(px(6.))
                    .disabled(!can_go_back)
                    .on_click(cx.listener(|view: &mut BrowserView, _event, window, cx| {
                        if let Some(tab) = view.tabs.active_mut() {
                            if tab.go_back() {
                                let new_url = tab.url().to_string();
                                view.address_bar.update(cx, |state, cx| {
                                    state.set_value(new_url, window, cx);
                                });
                                cx.notify();
                            }
                        }
                    })),
            )
            .child(
                Button::new("forward")
                    .label("→")
                    .ghost()
                    .rounded_sm()
                    .px(px(10.))
                    .py(px(6.))
                    .disabled(!can_go_forward)
                    .on_click(cx.listener(|view: &mut BrowserView, _event, window, cx| {
                         if let Some(tab) = view.tabs.active_mut() {
                             if tab.go_forward() {
                                 let new_url = tab.url().to_string();
                                 view.address_bar.update(cx, |state, cx| {
                                     state.set_value(new_url, window, cx);
                                 });
                                 cx.notify();
                             }
                         }
                    })),
            )
            .child(
                Button::new("refresh")
                    .label("⟳")
                    .ghost()
                    .rounded_sm()
                    .px(px(10.))
                    .py(px(6.)),
            )
            .child(Input::new(&self.address_bar).rounded_full())
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
