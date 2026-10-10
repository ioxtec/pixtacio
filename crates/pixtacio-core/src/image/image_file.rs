use super::metadata::ImageMetadata;
use std::{path::PathBuf, time::SystemTime};

/// Encoded file bytes and metadata, not decoded pixels.
pub struct ImageFile {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub data: Vec<u8>,
    pub extension: String,
    pub metadata: ImageMetadata,
    pub created: SystemTime,
    pub modified: SystemTime,
}
