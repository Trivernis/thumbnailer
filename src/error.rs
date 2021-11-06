use std::io;
use mime::Mime;
use thiserror::Error;

pub type ThumbResult<T> = Result<T, ThumbError>;

#[derive(Debug, Error)]
pub enum ThumbError {
    #[error("IO Error {0}")]
    IO(#[from] io::Error),

    #[error("Image Error {0}")]
    Image(#[from] image::error::ImageError),

    #[error("Failed to decode image")]
    Decode,

    #[error("Unsupported media type {0}")]
    Unsupported(Mime),
}