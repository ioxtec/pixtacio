use crate::dialogs::file_dialogs;
use crate::image::image_controller::ImageController;
use crate::ui::{
    navigation::Screen,
    sidebar::AppSidebar,
    toolbar::{AppToolbar, MenuItemEvent, OpenViewEvent},
};

use crate::ui::views::{image_view::ImageView, settings_view::SettingsView, video_view::VideoView};
use gpui_kit::component::{Root, h_flex, v_flex};
use gpui_kit::*;

/// Coordinates navigation and composes the main window's UI.
pub struct PixtacioApp {
    screen: Screen,
    image_view: Entity<ImageView>,
    video_view: Entity<VideoView>,
    settings_view: Entity<SettingsView>,
    toolbar: Entity<AppToolbar>,
    image_controller: Entity<ImageController>,
    sidebar: Entity<AppSidebar>,
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
        let toolbar = cx.new(|_| AppToolbar::new(screen));
        let sidebar = cx.new(|_| AppSidebar::new());
        let image_controller = cx.new(|_| ImageController::new());
        let image_view = cx.new(|cx| ImageView::new(image_controller.clone(), cx));
        let video_view = cx.new(|_| VideoView::new());
        let settings_view = cx.new(|cx| SettingsView::new(window, cx));
        // The toolbar requests navigation; the app decides how to handle it.
        let menu_event_sub = cx.subscribe(&toolbar, Self::open_menu_event_handler);
        let view_event_sub = cx.subscribe(&toolbar, Self::open_view_event_handler);

        Self {
            screen,
            image_view,
            video_view,
            settings_view,
            toolbar,
            image_controller,
            sidebar,
            _subscriptions: vec![menu_event_sub, view_event_sub],
        }
    }

    fn open_menu_event_handler(
        &mut self,
        _source: Entity<AppToolbar>,
        event: &MenuItemEvent,
        cx: &mut Context<Self>,
    ) {
        match event {
            MenuItemEvent::OpenFile => {
                println!("Open file");
            }
            MenuItemEvent::OpenFolder => {
                println!("Open folder");
                cx.spawn(async move |this, cx| {
                    let Some(folder) = file_dialogs::pick_folder().await else {
                        return;
                    };

                    let _ = this.update(cx, |app, cx| {
                        // Show folder information even when another view was active.
                        app.screen = Screen::Image;
                        app.toolbar.update(cx, |toolbar, cx| {
                            toolbar.set_screen(Screen::Image, cx);
                        });
                        app.image_controller.update(cx, |controller, cx| {
                            controller.open_folder(folder, cx);
                        });
                        cx.notify();
                    });
                })
                .detach();
            }
        }
    }

    fn open_view_event_handler(
        &mut self,
        _source: Entity<AppToolbar>,
        event: &OpenViewEvent,
        cx: &mut Context<Self>,
    ) {
        let OpenViewEvent::Navigate(screen) = event;
        let screen = *screen;
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
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
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
