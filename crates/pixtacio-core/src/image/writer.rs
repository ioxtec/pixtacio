use std::path::Path;

/// Encode and save an image in the explicitly selected format.
pub fn save_image(
    _image: &image::DynamicImage,
    _path: &Path,
    _format: image::ImageFormat,
) -> image::ImageResult<()> {
    todo!("Encode and save the image")
}
