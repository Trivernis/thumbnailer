const PNG_BYTES: &'static [u8] = include_bytes!("assets/test.png");
const JPG_BYTES: &'static [u8] = include_bytes!("assets/test.jpg");
const WEBP_BYTES: &'static [u8] = include_bytes!("assets/test.webp");

use crate::ImageType::{Jpeg, Png, Webp};
use mime::Mime;
use std::io::Cursor;
use std::str::FromStr;
use thumbnailer::error::ThumbResult;
use thumbnailer::{create_thumbnails, Thumbnail, ThumbnailSize};

enum ImageType {
    Png,
    Jpeg,
    Webp,
}

#[test]
fn it_creates_small_thumbnails_for_png() {
    create_thumbnail(Png, ThumbnailSize::Small).unwrap();
}

#[test]
fn it_creates_medium_thumbnails_for_png() {
    create_thumbnail(Png, ThumbnailSize::Medium).unwrap();
}

#[test]
fn it_creates_large_thumbnails_for_png() {
    create_thumbnail(Png, ThumbnailSize::Large).unwrap();
}

#[test]
fn it_creates_small_thumbnails_for_jpeg() {
    create_thumbnail(Jpeg, ThumbnailSize::Small).unwrap();
}

#[test]
fn it_creates_medium_thumbnails_for_jpeg() {
    create_thumbnail(Jpeg, ThumbnailSize::Medium).unwrap();
}

#[test]
fn it_creates_large_thumbnails_for_jpeg() {
    create_thumbnail(Jpeg, ThumbnailSize::Large).unwrap();
}

#[test]
fn it_creates_small_thumbnails_for_webp() {
    create_thumbnail(Webp, ThumbnailSize::Small).unwrap();
}

#[test]
fn it_creates_medium_thumbnails_for_webp() {
    create_thumbnail(Webp, ThumbnailSize::Medium).unwrap();
}

#[test]
fn it_creates_large_thumbnails_for_webp() {
    create_thumbnail(Webp, ThumbnailSize::Large).unwrap();
}

fn create_thumbnail(image_type: ImageType, size: ThumbnailSize) -> ThumbResult<Vec<Thumbnail>> {
    match image_type {
        ImageType::Png => {
            let reader = Cursor::new(PNG_BYTES);
            create_thumbnails(reader, mime::IMAGE_PNG, [size])
        }
        ImageType::Jpeg => {
            let reader = Cursor::new(JPG_BYTES);
            create_thumbnails(reader, mime::IMAGE_JPEG, [size])
        }
        ImageType::Webp => {
            let reader = Cursor::new(WEBP_BYTES);
            let webp_mime = Mime::from_str("image/webp").unwrap();
            create_thumbnails(reader, webp_mime, [size])
        }
    }
}
