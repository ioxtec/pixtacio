use gpui_kit::*;
use pixtacio_core::io::file_loader::get_images;
use pixtacio_core::io::image::ImageFile;

const IMAGE_FOLEDER_PATH: &str = "C:\\Users\\mryzk\\Pictures\\Wallpapers";
pub struct ImageView {
    images: Vec<ImageFile>,
    current_image: Option<pixtacio_core::io::image::ImageFile>,
}

impl ImageView {
    pub fn new() -> Self {
        ImageView {
            images: Vec::new(),
            current_image: None,
        }
    }

    async fn load_images_from_folder(&mut self, folder_path: &str) {
        self.images = get_images(folder_path).await;
    }
}

impl Render for ImageView {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0x99ccff)).child(
            if let Some(current_image) = &self.current_image {
                div().child(format!("Current Image: {}", current_image.name))
            } else {
                div().child("No image loaded")
            },
        )
    }
}
