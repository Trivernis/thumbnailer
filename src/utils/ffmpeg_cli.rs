use crate::error::ThumbError;
use crate::ThumbResult;
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::process::{Command, Stdio};

const FFMPEG: &str = "ffmpeg";

/// Runs ffmpeg to retrieve a png video frame
pub fn get_png_frame(video_file: &str, index: usize) -> ThumbResult<Vec<u8>> {
    ffmpeg([
        "-loglevel",
        "panic",
        "-i",
        video_file,
        "-vf",
        format!("select=eq(n\\,{})", index).as_str(),
        "-vframes",
        "1",
        "-c:v",
        "png",
        "-movflags",
        "empty_moov",
        "-f",
        "image2pipe",
        "pipe:1",
    ])
}

/// Runs ffmpeg with the given args
fn ffmpeg<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(args: I) -> ThumbResult<Vec<u8>> {
    let child = Command::new(FFMPEG)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;

    let output = child.wait_with_output()?;
    if output.status.success() && output.stdout.len() > 0 {
        Ok(output.stdout)
    } else {
        Err(ThumbError::FFMPEG(
            String::from_utf8_lossy(&output.stderr[..]).to_string(),
        ))
    }
}

pub fn is_ffmpeg_installed() -> bool {
    match Command::new("ffmpeg").args(["-loglevel", "quiet"]).spawn() {
        Ok(_) => true,
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                false
            } else {
                true
            }
        }
    }
}
