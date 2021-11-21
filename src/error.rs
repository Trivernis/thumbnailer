use image::ImageError;
use mime::Mime;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use vid2img::{CaptureError, StreamError};

pub type ThumbResult<T> = Result<T, ThumbError>;

#[derive(Debug)]
pub enum ThumbError {
    IO(io::Error),

    Image(image::error::ImageError),

    Decode,

    Unsupported(Mime),

    NullVideo,

    CaptureError(vid2img::CaptureError),

    StreamError(vid2img::StreamError),
}

impl Display for ThumbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThumbError::IO(_) => write!(f, "an io error occurred"),
            ThumbError::Image(e) => write!(f, "an image error occurred {}", e),
            ThumbError::Decode => write!(f, "failed to decode image"),
            ThumbError::Unsupported(mime) => write!(f, "Unsupported media type {}", mime),
            ThumbError::NullVideo => write!(f, "no video data found in file"),
            ThumbError::CaptureError(c) => {
                write!(f, "capture error when creating video thumbnail: {:?}", c)
            }
            ThumbError::StreamError(s) => {
                write!(f, "stream error when creating video thumbnail: {:?}", s)
            }
        }
    }
}

impl std::error::Error for ThumbError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ThumbError::IO(e) => e.source(),
            ThumbError::Image(i) => i.source(),
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

impl From<vid2img::CaptureError> for ThumbError {
    fn from(e: CaptureError) -> Self {
        Self::CaptureError(e)
    }
}

impl From<vid2img::StreamError> for ThumbError {
    fn from(s: StreamError) -> Self {
        Self::StreamError(s)
    }
}
