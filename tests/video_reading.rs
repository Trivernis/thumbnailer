use mime::Mime;
use std::io::Cursor;
use std::str::FromStr;
use thumbnailer::{create_thumbnails, ThumbnailSize};

const VIDEO_BYTES: &'static [u8] = include_bytes!("assets/test.mp4");

#[test]
fn it_creates_thumbnails_for_mp4() {
    let reader = Cursor::new(VIDEO_BYTES);
    let result = create_thumbnails(
        reader,
        Mime::from_str("video/mp4").unwrap(),
        [
            ThumbnailSize::Small,
            ThumbnailSize::Medium,
            ThumbnailSize::Large,
        ],
    );
    #[cfg(feature = "ffmpeg")]
    result.unwrap();

    #[cfg(not(feature = "ffmpeg"))]
    assert!(result.is_err())
}
