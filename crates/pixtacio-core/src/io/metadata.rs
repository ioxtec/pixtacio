use std::io::Cursor;

use image::{ImageFormat, ImageReader, ImageResult};

pub struct ImageMetadata {
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
    pub exif: Option<ExifMetadata>,
}

pub struct ExifMetadata {
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub iso: Option<u32>,
    pub orientation: Option<u16>,
}

impl ImageMetadata {
    /// Reads dimensions and optional EXIF without decoding the full image.
    pub fn from_bytes(data: &[u8]) -> ImageResult<Self> {
        let format = image::guess_format(data)?;
        let (width, height) =
            ImageReader::with_format(Cursor::new(data), format).into_dimensions()?;

        // Missing, unsupported, or malformed EXIF does not reject the image.
        let exif = exif::Reader::new()
            .read_from_container(&mut Cursor::new(data))
            .ok()
            .map(|reader| {
                let camera_make = reader
                    .get_field(exif::Tag::Make, exif::In::PRIMARY)
                    .map(|field| field.display_value().to_string());
                let camera_model = reader
                    .get_field(exif::Tag::Model, exif::In::PRIMARY)
                    .map(|field| field.display_value().to_string());
                let iso = reader
                    .get_field(exif::Tag::PhotographicSensitivity, exif::In::PRIMARY)
                    .and_then(|field| field.value.get_uint(0));
                let orientation = reader
                    .get_field(exif::Tag::Orientation, exif::In::PRIMARY)
                    .and_then(|field| field.value.get_uint(0))
                    .filter(|value| (1..=8).contains(value))
                    .map(|value| value as u16);

                ExifMetadata {
                    camera_make,
                    camera_model,
                    iso,
                    orientation,
                }
            });

        Ok(Self {
            width,
            height,
            format,
            exif,
        })
    }
}
