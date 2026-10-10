use crate::image::image_controller::ImageController;
use gpui_kit::*;

/// Future horizontal filmstrip; not mounted until rendering is implemented.
pub struct ThumbnailStrip {
    controller: Entity<ImageController>,
}

impl ThumbnailStrip {
    pub fn new(controller: Entity<ImageController>) -> Self {
        Self { controller }
    }

    pub fn request_visible_thumbnails(&mut self, _cx: &mut Context<Self>) {
        todo!("Generate visible thumbnails in background tasks with a bounded cache")
    }
}

impl Render for ThumbnailStrip {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // Keep the unfinished strip safe to render while learning.
        div().child("Thumbnails TODO")
    }
}
