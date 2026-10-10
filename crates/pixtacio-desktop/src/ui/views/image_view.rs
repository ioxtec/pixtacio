use crate::image::image_controller::ImageController;
use gpui_kit::*;

/// Presentation only; the controller owns the browsing state.
pub struct ImageView {
    controller: Entity<ImageController>,
    _subscription: Subscription,
}

impl ImageView {
    pub fn new(controller: Entity<ImageController>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.observe(&controller, |_, _, cx| cx.notify());
        Self {
            controller,
            _subscription: subscription,
        }
    }

    pub fn zoom_in(&mut self, _cx: &mut Context<Self>) {
        todo!("Increase the view zoom")
    }

    pub fn zoom_out(&mut self, _cx: &mut Context<Self>) {
        todo!("Decrease the view zoom")
    }

    pub fn reset_zoom(&mut self, _cx: &mut Context<Self>) {
        todo!("Reset zoom and pan")
    }
}

impl Render for ImageView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let controller = self.controller.read(cx);
        let label = if controller.is_loading() {
            "Loading folder…".to_owned()
        } else if let Some(error) = controller.error() {
            format!("Could not open folder: {error}")
        } else if let Some(folder) = controller.session().folder() {
            let selected = controller
                .session()
                .selected_path()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|| "No images in this folder".to_owned());
            format!(
                "Folder: {}\nImages: {}\nSelected: {}",
                folder.display(),
                controller.session().files().len(),
                selected
            )
        } else {
            "Use Open folder to choose an image folder".to_owned()
        };
        div()
            .size_full()
            .p_4()
            .bg(rgb(0x99ccff))
            .children(label.lines().map(|line| div().child(line.to_owned())))
    }
}
