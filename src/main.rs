/*
NOTE i should i should add hot reloading, i don't think zed uses it, but i know for sure it will reduce
time to finish the project
*/

use gpui::{
    App, AppContext, Bounds, IntoElement, ParentElement, Render, SharedString, Styled,
    TitlebarOptions, WindowBounds, WindowOptions, blue, div, green, px, size, yellow,
};

#[derive(Default)]
struct Todo {
    title: String,
    completed: bool,
}

#[derive(Default)]
struct TodoApp {
    todos: Vec<Todo>,
}

// you build elements based on style
impl Render for TodoApp {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::prelude::Context<Self>,
    ) -> impl IntoElement {
        const TODO_TITLE: &str = "Todo";

        // NOTE maybe you can understand what you did with tags ?(try)
        div().flex().items_center().justify_center().child(
            div()
                .w_2_3()
                .h_2_4()
                .h(px(400.))
                .bg(blue())
                .border_1()
                .rounded_md()
                .border_color(yellow())
                .child(div().child(TODO_TITLE).text_center().text_color(green())),
        )
    }
}

fn main() {
    gpui_platform::application().run(|cx: &mut App| {
        let bound = Bounds::centered(None, size(px(500.), px(500.)), cx);
        let title_bar = Some(TitlebarOptions {
            title: Some(SharedString::new("Todo")),
            ..Default::default()
        });
        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bound)),
            titlebar: title_bar,
            ..Default::default()
        };

        // NOTE want to ?, but didn't work
        cx.open_window(window_options, |_, cx| cx.new(|_| TodoApp::default()))
            .unwrap();
    });
}
