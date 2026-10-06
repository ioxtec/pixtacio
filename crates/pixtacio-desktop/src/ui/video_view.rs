use gpui_kit::*;

pub struct VideoView;

impl  VideoView{
    pub fn new() -> Self {
        VideoView {}
    }
}

impl Render for VideoView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child("Video View")
    }
}