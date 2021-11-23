use crate::error::{ThumbError, ThumbResult};
use crate::formats::image_format::read_image;
use image::DynamicImage;
use mime::Mime;
use std::io::{BufRead, Seek};

#[cfg(feature = "ffmpeg")]
use crate::formats::video_format::get_video_frame;

pub mod image_format;
#[cfg(feature = "ffmpeg")]
pub mod video_format;

/// Reads the buffer content into an image that can be used for thumbnail generation
pub fn get_base_image<R: BufRead + Seek>(reader: R, mime: Mime) -> ThumbResult<DynamicImage> {
    match mime.type_() {
        mime::IMAGE => read_image(reader, mime),
        #[cfg(feature = "ffmpeg")]
        mime::VIDEO => get_video_frame(reader, mime),
        _ => Err(ThumbError::Unsupported(mime)),
    }
}
