//! # Thumbnailer
//!
//! This crate can be used to generate thumbnails for all kinds of files.
//!
//! Example:
//! ```
//! use thumbnailer::{create_thumbnails, Thumbnail, ThumbnailSize};
//! use std::fs::File;
//! use std::io::BufReader;
//!
//! fn main() {
//!     let file = File::open("tests/assets/test.png").unwrap();
//!     let reader = BufReader::new(file);
//!     let mut  thumbnails = create_thumbnails(reader, mime::IMAGE_PNG, [ThumbnailSize::Small, ThumbnailSize::Medium]).unwrap();
//!     
//!     let thumbnail = thumbnails.pop().unwrap();
//!     let mut buf = Vec::new();
//!     thumbnail.write_png(&mut buf).unwrap();
//! }
//! ```

use crate::error::ThumbResult;
use image;
use image::imageops::FilterType;
use image::{DynamicImage, ImageOutputFormat};
use mime::Mime;
use rayon::prelude::*;
use std::io::{BufRead, Seek, Write};

use crate::formats::get_base_image;
pub use size::ThumbnailSize;

pub mod error;
mod formats;
mod size;

#[derive(Clone, Debug)]
pub struct Thumbnail {
    inner: DynamicImage,
}

impl Thumbnail {
    /// Writes the bytes of the image in a png format
    pub fn write_png<W: Write>(self, writer: &mut W) -> ThumbResult<()> {
        let image = DynamicImage::ImageRgba8(self.inner.into_rgba8());
        image.write_to(writer, ImageOutputFormat::Png)?;

        Ok(())
    }

    /// Writes the bytes of the image in a jpeg format
    pub fn write_jpeg<W: Write>(self, writer: &mut W, compression: u8) -> ThumbResult<()> {
        let image = DynamicImage::ImageRgb8(self.inner.into_rgb8());
        image.write_to(writer, ImageOutputFormat::Jpeg(compression))?;

        Ok(())
    }
}

/// Creates thumbnails of the requested sizes for the given reader providing the content as bytes and
/// the mime describing the contents type
pub fn create_thumbnails<R: BufRead + Seek, I: IntoIterator<Item = ThumbnailSize>>(
    reader: R,
    mime: Mime,
    sizes: I,
) -> ThumbResult<Vec<Thumbnail>> {
    let image = get_base_image(reader, mime)?;
    let sizes: Vec<ThumbnailSize> = sizes.into_iter().collect();
    let thumbnails = resize_images(image, &sizes)
        .into_iter()
        .map(|image| Thumbnail { inner: image })
        .collect();

    Ok(thumbnails)
}

fn resize_images(image: DynamicImage, sizes: &[ThumbnailSize]) -> Vec<DynamicImage> {
    sizes
        .into_par_iter()
        .map(|size| {
            let (width, height) = size.dimensions();
            image.resize(width, height, FilterType::Nearest)
        })
        .collect()
}
