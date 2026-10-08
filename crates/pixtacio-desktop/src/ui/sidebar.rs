use gpui_kit::component::IconName;
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
    SidebarToggleButton,
};
use gpui_kit::*;

pub struct AppSidebar {
    files: Vec<String>,
}

impl AppSidebar {
    pub fn new() -> Self {
        AppSidebar {
            files: vec!["cat.jpg".into(), "dog.png".into(), "mountains.jpg".into()],
        }
    }

    pub fn update_files(&mut self, file_list: Vec<String>, cx: &mut Context<Self>) {
        self.files = file_list;
        cx.notify();
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
