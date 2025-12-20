mod engines;
mod tabs;
mod ui;

use gpui::{AppContext, Application, WindowOptions};
use gpui_component::Root;
use ui::BrowserView;

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);
        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| BrowserView::new(window, cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
