use gpui_kit::component::{Root, h_flex, v_flex};
use gpui_kit::*;

use crate::ui::{
    image_view::ImageView,
    navigation::Screen,
    settings_view::SettingsView,
    sidebar::Sidebar,
    toolbar::{Toolbar, ToolbarEvent},
    video_view::VideoView,
};

/// Coordinates navigation and composes the main window's UI.
pub struct PixtacioApp {
    screen: Screen,
    image_view: Entity<ImageView>,
    video_view: Entity<VideoView>,
    settings_view: Entity<SettingsView>,
    toolbar: Entity<Toolbar>,
    sidebar: Entity<Sidebar>,
    // Dropping a subscription stops its event handler.
    _subscriptions: Vec<Subscription>,
}

impl PixtacioApp {
    /// Starts GPUI and opens the application's window.
    pub fn run() {
        application().with_assets(assets::Assets).run(|cx| {
            // Initialize GPUI Kit before creating its components.
            init(cx);
            let bounds = Bounds::centered(None, size(px(960.0), px(640.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Self::new(window, cx));
                    // Root provides support for dialogs, sheets, and notifications.
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )
            .expect("Failed to open Pixtacio window");
            cx.activate(true);
        });
    }

    /// Creates the views once; rendering does not recreate them.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let screen = Screen::Image;
        let toolbar = cx.new(|_| Toolbar::new(screen));
        let sidebar = cx.new(|_| Sidebar::new());
        let image_view = cx.new(|_| ImageView::new());
        let video_view = cx.new(|_| VideoView::new());
        let settings_view = cx.new(|cx| SettingsView::new(window, cx));
        // The toolbar requests navigation; the app decides how to handle it.
        let navigation = cx.subscribe(&toolbar, |this, _, event, cx| {
            let ToolbarEvent::Navigate(screen) = event;
            this.navigate(*screen, cx);
        });
        Self {
            screen,
            image_view,
            video_view,
            settings_view,
            toolbar,
            sidebar,
            _subscriptions: vec![navigation],
        }
    }

    /// All screen changes go through here; add leave/enter behavior when needed.
    pub fn navigate(&mut self, screen: Screen, cx: &mut Context<Self>) {
        if self.screen == screen {
            return;
        }
        self.screen = screen;
        // Keep the toolbar's selection in sync with the active screen.
        self.toolbar
            .update(cx, |toolbar, cx| toolbar.set_screen(screen, cx));
        // Notify GPUI that the app's rendered content has changed.
        cx.notify();
    }

    // Clone handles, preserving each view's state between screen changes.
    fn active_view(&self) -> AnyElement {
        // AnyElement gives the different view types a common return type.
        match self.screen {
            Screen::Image => self.image_view.clone().into_any_element(),
            Screen::Video => self.video_view.clone().into_any_element(),
            Screen::Settings => self.settings_view.clone().into_any_element(),
        }
    }
}

impl Render for PixtacioApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // The toolbar and sidebar stay in place while the content changes.
        v_flex()
            .size_full()
            .child("Pixtacio Desktop")
            .child(self.toolbar.clone())
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .child(self.sidebar.clone())
                    .child(div().flex_1().min_w_0().h_full().child(self.active_view())),
            )
    }
}
