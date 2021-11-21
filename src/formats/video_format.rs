use crate::error::{ThumbError, ThumbResult};
use image::png::PngDecoder;
use image::DynamicImage;
use mime::Mime;
use std::fs;
use std::io::{BufRead, Seek};
use std::path::PathBuf;
use vid2img::FileSource;

pub fn get_video_frame<R: BufRead + Seek>(mut reader: R, mime: Mime) -> ThumbResult<DynamicImage> {
    let tempdir = tempfile::tempdir()?;
    tempdir.path();
    let path = PathBuf::from(tempdir.path())
        .join("video")
        .with_extension(mime.subtype().as_str());

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    fs::write(&path, buf)?;

    let img = extract_frame_from_video(&path)?;
    tempdir.close()?;

    Ok(img)
}

fn extract_frame_from_video(path: &PathBuf) -> ThumbResult<DynamicImage> {
    let source = FileSource::new(path, (2000, 2000))?;
    for frame in source.into_iter() {
        if let Ok(Some(data)) = frame {
            let decoder = PngDecoder::new(data.as_slice())?;
            let img = DynamicImage::from_decoder(decoder)?;
            return Ok(img);
        }
    }
    Err(ThumbError::NullVideo)
}
