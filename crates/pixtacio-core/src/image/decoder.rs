use super::image_file::ImageFile;

/// Decode encoded bytes into pixels.
pub fn decode_image(_file: &ImageFile) -> image::ImageResult<image::DynamicImage> {
    todo!("Decode ImageFile.data using the image crate")
}
