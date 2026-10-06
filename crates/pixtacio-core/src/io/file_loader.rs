
use std::path::Path;

use tokio::fs::{File, read_dir};
use super::image::ImageFile;

pub async fn load_file(path: &Path) -> Result<tokio::fs::File, std::io::Error> {
    File::open(path).await
}

pub async fn get_files_list_from_folder(folder_path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut files = Vec::new();
    let mut dir = read_dir(folder_path).await?;

    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            if let Some(path_str) = path.to_str() {
                files.push(path_str.to_string());
            }
        }
    }

    Ok(files)
}

pub async fn get_images(folder_path: &str) -> Vec<ImageFile> {
    let mut images = Vec::new();
    if let Ok(files) = get_files_list_from_folder(folder_path).await {
        for file_path in files {
            let path = Path::new(&file_path);
            if let Ok(image_file) = ImageFile::new(path) {
                images.push(image_file);
            }
        }
    }
    images
}