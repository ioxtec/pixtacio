use std::path::{Path, PathBuf};

/// Browsing state only: no GPUI, dialogs, or background tasks.
#[derive(Default)]
pub struct ImageSession {
    folder: Option<PathBuf>,
    files: Vec<PathBuf>,
    selected_index: Option<usize>,
}

impl ImageSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_folder(&mut self, folder: PathBuf, files: Vec<PathBuf>) {
        self.folder = Some(folder);
        self.selected_index = if files.is_empty() { None } else { Some(0) };
        self.files = files;
    }

    pub fn folder(&self) -> Option<&Path> {
        self.folder.as_deref()
    }

    pub fn files(&self) -> &[PathBuf] {
        &self.files
    }

    pub fn selected_path(&self) -> Option<&Path> {
        self.files.get(self.selected_index?).map(PathBuf::as_path)
    }

    pub fn select_image(&mut self, _index: usize) -> bool {
        todo!("Select a valid index; return false for an invalid index")
    }

    /// Wrap around to the first image after the last one.
    pub fn next_image(&mut self) {
        if self.files.is_empty() {
            return;
        }
        self.selected_index = Some(match self.selected_index {
            Some(index) => (index + 1) % self.files.len(),
            None => 0,
        });
    }

    pub fn previous_image(&mut self) {
        todo!("Move to the previous image, wrapping at the beginning")
    }

    pub fn clear(&mut self) {
        todo!("Reset the folder, list, and selection")
    }
}
