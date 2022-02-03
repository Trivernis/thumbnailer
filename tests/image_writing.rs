use mime::Mime;
use std::io::Cursor;
use std::str::FromStr;
use thumbnailer::error::ThumbResult;
use thumbnailer::{create_thumbnails, ThumbnailSize};

const PNG_BYTES: &'static [u8] = include_bytes!("assets/test.png");
const JPG_BYTES: &'static [u8] = include_bytes!("assets/test.jpg");
const WEBP_BYTES: &'static [u8] = include_bytes!("assets/test.webp");

enum SourceFormat {
    Png,
    Jpeg,
    Webp,
}

enum TargetFormat {
    Png,
    Jpeg,
}

#[test]
fn it_converts_png_thumbnails_for_png() {
    write_thumbnail(SourceFormat::Png, TargetFormat::Png).unwrap();
}

#[test]
fn it_converts_jpeg_thumbnails_for_png() {
    write_thumbnail(SourceFormat::Png, TargetFormat::Jpeg).unwrap();
}

#[test]
fn it_converts_png_thumbnails_for_jpeg() {
    write_thumbnail(SourceFormat::Jpeg, TargetFormat::Png).unwrap();
}

#[test]
fn it_converts_jpeg_thumbnails_for_jpeg() {
    write_thumbnail(SourceFormat::Jpeg, TargetFormat::Jpeg).unwrap();
}

#[test]
fn it_converts_png_thumbnails_for_webp() {
    write_thumbnail(SourceFormat::Webp, TargetFormat::Png).unwrap();
}

#[test]
fn it_converts_jpeg_thumbnails_for_webp() {
    write_thumbnail(SourceFormat::Webp, TargetFormat::Jpeg).unwrap();
}

fn write_thumbnail(
    source_format: SourceFormat,
    target_format: TargetFormat,
) -> ThumbResult<Vec<u8>> {
    let thumb = match source_format {
        SourceFormat::Png => {
            let reader = Cursor::new(PNG_BYTES);
            create_thumbnails(reader, mime::IMAGE_PNG, [ThumbnailSize::Medium]).unwrap()
        }
        SourceFormat::Jpeg => {
            let reader = Cursor::new(JPG_BYTES);
            create_thumbnails(reader, mime::IMAGE_JPEG, [ThumbnailSize::Medium]).unwrap()
        }
        SourceFormat::Webp => {
            let reader = Cursor::new(WEBP_BYTES);
            let webp_mime = Mime::from_str("image/webp").unwrap();
            create_thumbnails(reader, webp_mime, [ThumbnailSize::Medium]).unwrap()
        }
    }
    .pop()
    .unwrap();

    let mut buf = Cursor::new(Vec::new());
    match target_format {
        TargetFormat::Png => thumb.write_png(&mut buf)?,
        TargetFormat::Jpeg => thumb.write_jpeg(&mut buf, 8)?,
    }

    Ok(buf.into_inner())
}
