use std::io::{BufRead, Read, Seek};
use image::{DynamicImage, ImageFormat};
use mime::Mime;
use image::io::Reader as ImageReader;
use webp::Decoder as WebpDecoder;
use crate::error::{ThumbError, ThumbResult};

const IMAGE_WEBP_MIME: &str = "image/webp";

/// Reads an image with a known mime type
pub fn read_image<R: BufRead + Seek>(reader: R, mime: Mime) -> ThumbResult<DynamicImage> {
    match mime.essence_str() {
        IMAGE_WEBP_MIME => read_webp_image(reader),
        _ => read_generic_image(reader, mime_to_image_format(mime)),
    }
}

/// Reads a webp image
fn read_webp_image<R: Read>(mut reader: R) -> ThumbResult<DynamicImage> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    let webp_image = WebpDecoder::new(&buf).decode().ok_or_else(|| ThumbError::Decode)?;

    Ok(webp_image.to_image())
}

/// Reads a generic image
fn read_generic_image<R: BufRead + Seek>(reader: R, format: Option<ImageFormat>) -> ThumbResult<DynamicImage> {
    let reader = if let Some(format) = format {
        ImageReader::with_format(reader, format)
    } else {
        ImageReader::new(reader).with_guessed_format()?
    };
    let image = reader.decode()?;

    Ok(image)
}

fn mime_to_image_format(mime: Mime) -> Option<ImageFormat> {
    match mime.subtype().as_str() {
        "png" => Some(ImageFormat::Png),
        "jpeg" => Some(ImageFormat::Jpeg),
        "bmp" => Some(ImageFormat::Bmp),
        "gif" => Some(ImageFormat::Gif),
        _ => None,
    }
}