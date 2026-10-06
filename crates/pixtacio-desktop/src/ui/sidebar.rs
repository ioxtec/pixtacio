use gpui_kit::component::*;
use gpui_kit::*;

const SIDEBAR_WIDTH: f32 = 200.0;

pub struct Sidebar {
    files: Vec<String>,
    selected_index: Option<usize>,
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar {
            files: Vec::new(),
            selected_index: None,
        }
    }
}

impl Render for Sidebar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(px(SIDEBAR_WIDTH))
            .bg(rgb(0xdddddd))
            .children(self.files.iter().map(|file| div().child(file.clone())))
    }
}
