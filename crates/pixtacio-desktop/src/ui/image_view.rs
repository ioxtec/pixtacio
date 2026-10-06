use gpui_kit::*;
use pixtacio_core::io::file_loader::get_images;

const  IMAGE_FOLEDER_PATH: &str = "C:\\Users\\mryzk\\Pictures\\Wallpapers";
pub struct ImageView {}

impl ImageView {
    pub fn new() -> Self {
        init();
        ImageView {}
    }
}

impl Render for ImageView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(rgb(0x99ccff)).child("Image View")
    }
}

fn init() {
    get_list_of_images();
}

fn get_list_of_images(){
    let images = get_images(IMAGE_FOLEDER_PATH);
}