use super::navigation::Screen;
use gpui_kit::component::IconName;
use gpui_kit::component::button::Button;
use gpui_kit::component::separator::Separator;
use gpui_kit::component::toolbar::Toolbar;
use gpui_kit::*;

pub struct AppToolbar {
    // Mirrors the app's selection to highlight the active button.
    screen: Screen,
}

/// Requests sent to the app without holding a reference to it.
pub enum OpenViewEvent {
    Navigate(Screen),
}

pub enum MenuItemEvent {
    OpenFile,
    OpenFolder,
}

impl EventEmitter<OpenViewEvent> for AppToolbar {}
impl EventEmitter<MenuItemEvent> for AppToolbar {}

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
                Button::new("open-folder")
                    .icon(IconName::File)
                    .label("Open folder")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(MenuItemEvent::OpenFolder);
                    })),
            )
            .child(
                Button::new("images")
                    .icon(IconName::File)
                    .label("Images")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(OpenViewEvent::Navigate(Screen::Image));
                    })),
            )
            .content(Separator::vertical().h_5())
            .child(
                Button::new("videos")
                    .icon(IconName::File)
                    .label("Videos")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(OpenViewEvent::Navigate(Screen::Video));
                    })),
            )
            .content(div().flex_1())
            .child(
                Button::new("settings")
                    .icon(IconName::Settings)
                    .label("Settings")
                    .on_click(cx.listener(|_toolbar, _event, _window, cx| {
                        cx.emit(OpenViewEvent::Navigate(Screen::Settings));
                    })),
            )
    }
}
