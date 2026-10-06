use std::path::Path;

use super::metadata::ImageMetadata;

pub struct ImageFile {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub data: Vec<u8>,
    pub extension: String,
    pub metadata: ImageMetadata,
    pub created: std::time::SystemTime,
    pub modified: std::time::SystemTime,
}

impl ImageFile {
    /// Loads the file and builds its metadata before constructing the value.
    pub fn new(file_path: &Path) -> image::ImageResult<Self> {
        let name = file_path
            .file_name()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidInput, "Path has no file name")
            })?
            .to_string_lossy()
            .into_owned();
        let path = file_path.to_string_lossy().into_owned();

        // Filesystem metadata and image metadata describe different things.
        let file_metadata = std::fs::metadata(file_path)?;
        let size = file_metadata.len();
        let created = file_metadata.created()?;
        let modified = file_metadata.modified()?;
        let data = std::fs::read(file_path)?;
        let metadata = ImageMetadata::from_bytes(&data)?;
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        Ok(Self {
            name,
            path,
            size,
            data,
            extension,
            metadata,
            created,
            modified,
        })
    }
}
