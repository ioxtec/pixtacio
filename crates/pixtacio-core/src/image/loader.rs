use super::{image_file::ImageFile, metadata::ImageMetadata};
use std::path::Path;

/// Reads one file. Run this blocking operation on a background executor.
pub fn load_file(path: &Path) -> image::ImageResult<ImageFile> {
    let name = path
        .file_name()
        .ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path has no file name")
        })?
        .to_string_lossy()
        .into_owned();
    let file_metadata = std::fs::metadata(path)?;
    let data = std::fs::read(path)?;
    let metadata = ImageMetadata::from_bytes(&data)?;
    Ok(ImageFile {
        name,
        path: path.to_path_buf(),
        size: file_metadata.len(),
        data,
        extension: path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_owned(),
        metadata,
        created: file_metadata.created()?,
        modified: file_metadata.modified()?,
    })
}
