use super::navigation::Screen;
use gpui_kit::base::Selectable as _;
use gpui_kit::component::IconName;
use gpui_kit::component::separator::Separator;
use gpui_kit::component::toolbar::Toolbar;
use gpui_kit::component::{Sizable as _, button::Button, h_flex};
use gpui_kit::*;

pub struct AppToolbar {
    // Mirrors the app's selection to highlight the active button.
    screen: Screen,
}

/// Requests sent to the app without holding a reference to it.
pub enum AppToolbarEvent {
    Navigate(Screen),
}

impl EventEmitter<AppToolbarEvent> for AppToolbar {}

impl AppToolbar {
    pub fn new(screen: Screen) -> Self {
        Self { screen }
    }

    /// Updates the selection after the app accepts a navigation request.
    pub fn set_screen(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        cx.notify();
    }
}

impl Render for AppToolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Toolbar::new("toolbar")
            .child(
                Button::new("images")
                    .icon(IconName::File)
                    .label("Images")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(AppToolbarEvent::Navigate(Screen::Image));
                    })),
            )
            .content(Separator::vertical().h_5())
            .child(
                Button::new("videos")
                    .icon(IconName::File)
                    .tooltip("Videos")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(AppToolbarEvent::Navigate(Screen::Video));
                    })),
            )
            .content(div().flex_1())
            .child(
                Button::new("settings")
                    .icon(IconName::Settings)
                    .tooltip("More options")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(AppToolbarEvent::Navigate(Screen::Settings));
                    })),
            )
    }
}
//cx.emit(AppToolbarEvent::Navigate(screen));
