use crate::error::{ThumbError, ThumbResult};
use crate::utils::ffmpeg_cli::{get_png_frame, is_ffmpeg_installed};
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use mime::Mime;
use std::fs;
use std::io::{BufRead, Cursor, Seek};
use std::path::PathBuf;

pub fn get_video_frame<R: BufRead + Seek>(mut reader: R, mime: Mime) -> ThumbResult<DynamicImage> {
    lazy_static::lazy_static! { static ref FFMPEG_INSTALLED: bool = is_ffmpeg_installed(); }

    if !*FFMPEG_INSTALLED {
        return Err(ThumbError::Unsupported(mime));
    }

    let tempdir = tempfile::tempdir()?;
    let path = PathBuf::from(tempdir.path())
        .join("video")
        .with_extension(mime.subtype().as_str());

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    fs::write(&path, buf)?;

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let png_bytes = get_png_frame(
        path.to_str()
            .expect("path to tmpdir contains invalid characters"),
        16,
    )?; // take the 16th frame
    tempdir.close()?;
    let img = ImageReader::with_format(Cursor::new(png_bytes), ImageFormat::Png).decode()?;

    Ok(img)
}
