use std::path::{Path, PathBuf};

/// Lists candidate images without reading their contents.
pub fn list_images(folder: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(folder)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_file() && is_supported_extension(&path) {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}

/// This is only a filename filter; decoding still validates the contents.
pub fn is_supported_extension(path: &Path) -> bool {
    image::ImageFormat::from_path(path)
        .map(|format| format.reading_enabled())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_sorted_image_paths_without_decoding_files() -> std::io::Result<()> {
        let directory = std::env::temp_dir().join(format!(
            "pixtacio-catalog-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory)?;
        let result = (|| -> std::io::Result<()> {
            // Empty files prove listing does not attempt decoding.
            std::fs::write(directory.join("b.png"), [])?;
            std::fs::write(directory.join("a.jpg"), [])?;
            std::fs::write(directory.join("notes.txt"), [])?;
            std::fs::create_dir(directory.join("nested.png"))?;
            assert_eq!(
                list_images(&directory)?,
                vec![directory.join("a.jpg"), directory.join("b.png")]
            );
            assert!(list_images(&directory.join("missing")).is_err());
            Ok(())
        })();
        // Remove only the specific temporary entries created by this test.
        std::fs::remove_file(directory.join("a.jpg"))?;
        std::fs::remove_file(directory.join("b.png"))?;
        std::fs::remove_file(directory.join("notes.txt"))?;
        std::fs::remove_dir(directory.join("nested.png"))?;
        std::fs::remove_dir(directory)?;
        result
    }
}
