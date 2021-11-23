use image::ImageError;
use mime::Mime;
use std::fmt::{Debug, Display, Formatter};
use std::io;

pub type ThumbResult<T> = Result<T, ThumbError>;

#[derive(Debug)]
pub enum ThumbError {
    IO(io::Error),

    Image(image::error::ImageError),

    Decode,

    Unsupported(Mime),

    NullVideo,

    #[cfg(feature = "ffmpeg")]
    FFMPEG(ffmpeg_next::Error),
}

impl Display for ThumbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThumbError::IO(_) => write!(f, "an io error occurred"),
            ThumbError::Image(e) => write!(f, "an image error occurred {}", e),
            ThumbError::Decode => write!(f, "failed to decode image"),
            ThumbError::Unsupported(mime) => write!(f, "Unsupported media type {}", mime),
            ThumbError::NullVideo => write!(f, "no video data found in file"),

            #[cfg(feature = "ffmpeg")]
            ThumbError::FFMPEG(e) => write!(f, "ffmpeg error: {}", e),
        }
    }
}

impl std::error::Error for ThumbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ThumbError::IO(e) => e.source(),
            ThumbError::Image(i) => i.source(),

            #[cfg(feature = "ffmpeg")]
            ThumbError::FFMPEG(e) => e.source(),

            _ => None,
        }
    }
}

impl From<io::Error> for ThumbError {
    fn from(e: io::Error) -> Self {
        Self::IO(e)
    }
}

impl From<image::error::ImageError> for ThumbError {
    fn from(e: ImageError) -> Self {
        Self::Image(e)
    }
}

#[cfg(feature = "ffmpeg")]
impl From<ffmpeg_next::Error> for ThumbError {
    fn from(e: ffmpeg_next::Error) -> Self {
        Self::FFMPEG(e)
    }
}
