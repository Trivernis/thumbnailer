# Thumbnailer

This crate can be used to create thumbnails for all kinds of files.

## Usage 

```rust
use thumbnailer::{create_thumbnails, Thumbnail, ThumbnailSize};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("tests/assets/test.png").unwrap();
    let reader = BufReader::new(file);
    let mut  thumbnails = create_thumbnails(reader, mime::IMAGE_PNG, [ThumbnailSize::Small, ThumbnailSize::Medium]).unwrap();
    
    let thumbnail = thumbnails.pop().unwrap();
    let mut buf = Vec::new();
    thumbnail.write_png(&mut buf).unwrap();
}
```

## Supported media types

| Type  | Subtype |
|-------|---------|
| Image | Png     |
| Image | Bmp     |
| image | Jpeg    |
| Image | Webp    |
| Image | Gif     |

## License

MIT