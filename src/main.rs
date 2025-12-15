use gpui::{
    AppContext, Application, Context, IntoElement, ParentElement, Render, Styled, Window,
    WindowOptions, div,
};
use gpui_component::{
    Root, StyledExt,
    button::{Button, ButtonVariants},
};
use neodymium::engine::{EngineRegistry, MockJavaScriptEngine, MockRenderingEngine};
use neodymium::state::{BrowserState, Tab};

pub struct BrowserView {
    state: BrowserState,
    engines: EngineRegistry,
}

impl BrowserView {
    fn new() -> Self {
        let engines = EngineRegistry::new(
            Box::new(MockRenderingEngine::new("Blink-compatible")),
            Box::new(MockJavaScriptEngine::new("V8-compatible")),
        );
        Self {
            state: BrowserState::new("https://example.com"),
            engines,
        }
    }

    fn build_tab_bar(&self) -> impl IntoElement {
        self.state
            .tabs()
            .iter()
            .fold(div().h_flex().gap_2(), |container, tab| {
                container.child(self.render_tab(tab))
            })
    }

    fn render_tab(&self, tab: &Tab) -> impl IntoElement {
        let _active = tab.id == self.state.active_tab().id;
        let label = format!("{} · {}", tab.title, tab.url);
        div()
            .child(label)
            .paddings(6.0)
            .rounded_md()
            .border_1()
    }

    fn build_toolbar(&self) -> impl IntoElement {
        let engine_badge = format!(
            "Rendering: {} | JS: {}",
            self.engines.rendering_engine_name(),
            self.engines.javascript_engine_name()
        );
        div()
            .h_flex()
            .gap_2()
            .items_center()
            .child(Button::new("back").ghost().label("←"))
            .child(Button::new("forward").ghost().label("→"))
            .child(Button::new("refresh").ghost().label("⟳"))
            .child(
                div()
                    .flex_1()
                    .paddings(6.0)
                    .rounded_md()
                    .border_1()
                    .child(format!("🔒 {}", self.state.active_tab().url)),
            )
            .child(Button::new("tabs").primary().label("+ 新しいタブ"))
            .child(div().child(engine_badge))
    }
}

impl Render for BrowserView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .paddings(12.0)
            .child(
                div()
                    .h_flex()
                    .justify_between()
                    .items_center()
                    .child(div().child("Neo Browser"))
                    .child(div().child("•••")),
            )
            .child(self.build_tab_bar())
            .child(self.build_toolbar())
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .paddings(12.0)
                    .border_1()
                    .rounded_md()
                    .child(format!("現在表示中: {}", self.state.active_tab().url))
                    .child("Google Chrome風のツールバーとタブバーを持つサンドボックスです。")
                    .child(format!(
                        "プラガブルなエンジン: {}, {}",
                        self.engines.rendering_engine_name(),
                        self.engines.javascript_engine_name()
                    ))
                    .child(
                        div()
                            .child(self.engines.render_page(&self.state.active_tab().url)),
                    )
                    .child(div().child(self.engines.evaluate_script("console.log('ready')"))),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| BrowserView::new());
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
