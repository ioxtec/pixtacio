use rfd::AsyncFileDialog;
use std::path::PathBuf;

/// Only selects a path; it does not load files or change the session.
pub async fn pick_folder() -> Option<PathBuf> {
    AsyncFileDialog::new()
        .set_title("Select image folder")
        .pick_folder()
        .await
        .map(|folder| folder.path().to_path_buf())
}

pub async fn pick_image() -> Option<PathBuf> {
    todo!("Show an image file picker")
}
