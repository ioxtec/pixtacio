use super::image_session::ImageSession;
use gpui_kit::*;
use pixtacio_core::image::catalog;
use std::path::PathBuf;

/// GPUI adapter: schedules work and publishes changes to the views.
pub struct ImageController {
    session: ImageSession,
    loading: bool,
    error: Option<String>,
    folder_task: Option<Task<()>>,
}

impl ImageController {
    pub fn new() -> Self {
        Self {
            session: ImageSession::new(),
            loading: false,
            error: None,
            folder_task: None,
        }
    }

    pub fn session(&self) -> &ImageSession {
        &self.session
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }

    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    pub fn open_folder(&mut self, folder: PathBuf, cx: &mut Context<Self>) {
        self.loading = true;
        self.error = None;
        cx.notify();

        // Replacing the task cancels the previous waiter.
        self.folder_task = Some(cx.spawn(async move |this, cx| {
            let result = cx
                .background_executor()
                .spawn(async move {
                    let result = catalog::list_images(&folder);
                    (folder, result)
                })
                .await;

            let _ = this.update(cx, |controller, cx| {
                controller.loading = false;
                match result {
                    (folder, Ok(files)) => controller.session.set_folder(folder, files),
                    (_, Err(error)) => controller.error = Some(error.to_string()),
                }
                cx.notify();
            });
        }));
    }

    pub fn open_image(&mut self, _path: PathBuf, _cx: &mut Context<Self>) {
        todo!("List the parent folder, select the requested file, and load it")
    }

    pub fn load_selected_image(&mut self, _cx: &mut Context<Self>) {
        todo!("Load and decode the selection in the background, then publish the result")
    }
}
