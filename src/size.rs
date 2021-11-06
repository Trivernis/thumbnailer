
/// Represents fixed sizes of a thumbnail
#[derive(Clone, Copy, Debug)]
pub enum ThumbnailSize {
    Icon,
    Small,
    Medium,
    Large,
    Larger,
    Custom((u32, u32))
}

impl ThumbnailSize {
    pub fn dimensions(&self) -> (u32, u32) {
        match self {
            ThumbnailSize::Icon => (64, 64),
            ThumbnailSize::Small => (128, 128),
            ThumbnailSize::Medium => (256, 256),
            ThumbnailSize::Large => (512, 512),
            ThumbnailSize::Larger => (1024, 1024),
            ThumbnailSize::Custom(size) => *size,
        }
    }
}