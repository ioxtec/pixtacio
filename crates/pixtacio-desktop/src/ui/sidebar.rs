use gpui_kit::component::IconName;
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_kit::*;

/// Application navigation; image previews belong to ThumbnailStrip.
pub struct AppSidebar;

impl AppSidebar {
    pub fn new() -> Self {
        Self
    }
}

impl Render for AppSidebar {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("app-sidebar")
            .header(SidebarHeader::new().child("My Application"))
            .child(
                SidebarGroup::new("Navigation").child(
                    SidebarMenu::new()
                        .child(
                            SidebarMenuItem::new("Dashboard")
                                .icon(IconName::LayoutDashboard)
                                .on_click(|_, _, _| println!("Dashboard clicked")),
                        )
                        .child(
                            SidebarMenuItem::new("Settings")
                                .icon(IconName::Settings)
                                .on_click(|_, _, _| println!("Settings clicked")),
                        ),
                ),
            )
            .footer(SidebarFooter::new().child("User Profile"))
    }
}
