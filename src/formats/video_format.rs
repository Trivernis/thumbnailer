use crate::error::{ThumbError, ThumbResult};
use ffmpeg_next::codec::decoder::Video as VideoDecoder;
use ffmpeg_next::filter;
use ffmpeg_next::filter::Graph;
use ffmpeg_next::frame::Video;
use ffmpeg_next::media::Type as MediaType;
use ffmpeg_next::threading::Config;
use image::{DynamicImage, RgbaImage};
use mime::Mime;
use std::fs;
use std::io::{BufRead, Seek};
use std::path::PathBuf;

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
    let mut input = ffmpeg_next::format::input(path)?;
    let stream = input
        .streams()
        .best(MediaType::Video)
        .ok_or_else(|| ThumbError::NullVideo)?;

    let mut decoder = stream.codec().decoder().video()?;
    decoder.set_threading(Config::count(1));

    let mut filter = get_format_filter(&mut decoder)?;

    let stream_index = stream.index();

    let packets = input
        .packets()
        .filter(|(s, _)| s.index() == stream_index)
        .map(|(_, p)| p);

    let mut frame = Video::empty();
    let mut output_frame = Video::empty();
    let mut count = 0;

    for packet in packets {
        decoder.send_packet(&packet)?;
        while let Err(ffmpeg_next::Error::DecoderNotFound) = decoder.receive_frame(&mut frame) {}

        if decode_single_frame(&mut filter, &mut frame, &mut output_frame).is_ok() {
            count += 1;
        }
        if count > 2 {
            // take the second frame because the first one often is just blank
            break;
        }
    }
    decoder.send_eof()?;

    convert_frame_to_image(&decoder, output_frame)
}

fn get_format_filter(decoder: &mut VideoDecoder) -> ThumbResult<Graph> {
    let args = format!(
        "width={w}:height={h}:video_size={w}x{h}:pix_fmt={fmt}:time_base={base}",
        w = decoder.width(),
        h = decoder.height(),
        fmt = decoder
            .format()
            .descriptor()
            .ok_or_else(|| ThumbError::NullVideo)?
            .name(),
        base = decoder.time_base(),
    );
    let mut filter = Graph::new();
    filter.add(
        &filter::find("buffer").ok_or_else(|| ThumbError::NullVideo)?,
        "in",
        &args,
    )?;
    filter.add(
        &filter::find("buffersink").ok_or_else(|| ThumbError::NullVideo)?,
        "out",
        "",
    )?;
    filter
        .output("in", 0)?
        .input("out", 0)?
        .parse("format=rgba")?;
    filter.validate()?;

    Ok(filter)
}

fn decode_single_frame(
    filter: &mut Graph,
    frame: &mut Video,
    mut output_frame: &mut Video,
) -> ThumbResult<()> {
    filter
        .get("in")
        .ok_or_else(|| ThumbError::NullVideo)?
        .source()
        .add(&frame)?;
    let mut ctx = filter.get("out").ok_or_else(|| ThumbError::NullVideo)?;
    let mut out = ctx.sink();
    out.frame(&mut output_frame)?;

    Ok(())
}

fn convert_frame_to_image(
    decoder: &VideoDecoder,
    output_frame: Video,
) -> ThumbResult<DynamicImage> {
    let image = RgbaImage::from_raw(
        decoder.width(),
        decoder.height(),
        output_frame.data(0).to_vec(),
    )
    .ok_or_else(|| ThumbError::NullVideo)?;
    let image = DynamicImage::ImageRgba8(image);

    Ok(image)
}
