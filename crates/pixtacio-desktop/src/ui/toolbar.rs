use super::navigation::Screen;
use gpui_kit::base::Selectable as _;
use gpui_kit::component::{Sizable as _, button::Button, h_flex};
use gpui_kit::*;

pub struct Toolbar {
    // Mirrors the app's selection to highlight the active button.
    screen: Screen,
}

/// Requests sent to the app without holding a reference to it.
pub enum ToolbarEvent {
    Navigate(Screen),
}

impl EventEmitter<ToolbarEvent> for Toolbar {}

impl Toolbar {
    pub fn new(screen: Screen) -> Self {
        Self { screen }
    }

    /// Updates the selection after the app accepts a navigation request.
    pub fn set_screen(&mut self, screen: Screen, cx: &mut Context<Self>) {
        self.screen = screen;
        cx.notify();
    }
}

impl Render for Toolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        h_flex()
            .h(px(80.0))
            .flex_shrink_0()
            .gap_2()
            .bg(rgb(0x333333))
            .children(
                [
                    ("show-image", "Image", Screen::Image),
                    ("show-video", "Video", Screen::Video),
                    ("show-settings", "Settings", Screen::Settings),
                ]
                .into_iter()
                .map(|(id, label, screen)| {
                    Button::new(id)
                        .small()
                        .label(label)
                        .selected(self.screen == screen)
                        // Capture this button's destination in its click handler.
                        .on_click(cx.listener(move |_, _, _, cx| {
                            cx.emit(ToolbarEvent::Navigate(screen));
                        }))
                }),
            )
    }
}
