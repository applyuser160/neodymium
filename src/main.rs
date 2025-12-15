use gpui::{
    AppContext, Application, Context, Edges, IntoElement, ParentElement, Render, Styled, Window,
    WindowOptions, div, px,
};
use gpui_component::{
    Root, StyledExt,
    button::{Button, ButtonVariants},
};
use Neodymium::{BrowserModel, EngineRegistry, MockRenderingEngine, MockScriptEngine};

struct BrowserShell {
    model: BrowserModel,
    engines: EngineRegistry,
}

impl BrowserShell {
    fn new() -> Self {
        let mut engines = EngineRegistry::default();
        engines.register_rendering_engine(MockRenderingEngine::default());
        engines.register_script_engine(MockScriptEngine::default());

        let mut model = BrowserModel::new();
        model.add_tab("Design Docs", "https://design.chrome.com");
        model.set_active_status("Ready to composite page layers");

        Self { model, engines }
    }
}

impl Render for BrowserShell {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let tab_strip = div()
            .h_flex()
            .gap_2()
            .items_center()
            .child(div().child("＋").child(" New Tab"));

        let tabs = self
            .model
            .tabs
            .iter()
            .enumerate()
            .fold(tab_strip, |container, (idx, tab)| {
                let is_active = idx == self.model.active_tab;
                let pill = div()
                    .h_flex()
                    .gap_2()
                    .paddings(Edges {
                        top: px(6.),
                        right: px(8.),
                        bottom: px(6.),
                        left: px(8.),
                    })
                    .items_center()
                    .justify_between()
                    .child(div().child(if is_active { "●" } else { "○" }))
                    .child(div().child(tab.title.clone()))
                    .child(div().child("✕"));

                container.child(pill)
            });

        let toolbar = div()
            .h_flex()
            .gap_2()
            .items_center()
            .child(Button::new("back").ghost().label("←"))
            .child(Button::new("forward").ghost().label("→"))
            .child(Button::new("refresh").ghost().label("⟳"))
            .child(
                div()
                    .h_flex()
                    .gap_2()
                    .flex_1()
                    .paddings(Edges::all(px(8.)))
                    .child(div().child("🔒"))
                    .child(div().child("https://neodymium.dev/home"))
                    .child(div().child("☆")),
            )
            .child(Button::new("profile").primary().label("•••"));

        let engine_panel = div()
            .v_flex()
            .gap_2()
            .child(div().child("Rendering Pipeline"))
            .child(div().child(format!(
                "Renderer: {}",
                self
                    .engines
                    .active_rendering_engine()
                    .map(|r| r.name().to_string())
                    .unwrap_or_else(|| "None".into())
            )))
            .child(div().child(format!(
                "Script Engine: {}",
                self
                    .engines
                    .active_script_engine()
                    .map(|r| r.name().to_string())
                    .unwrap_or_else(|| "None".into())
            )))
            .child(div().child(
                "Pluggable engines can be swapped without touching UI plumbing.",
            ));

        div()
            .v_flex()
            .gap_2()
            .size_full()
            .child(tabs)
            .child(toolbar)
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .flex_1()
                    .child(div().child("Google Chrome inspired shell"))
                    .child(div().child(
                        self.model
                            .render_status
                            .clone()
                            .unwrap_or_else(|| "Waiting for page load".into()),
                    ))
                    .child(engine_panel),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| BrowserShell::new());
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
