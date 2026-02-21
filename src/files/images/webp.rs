use derive_more::{AsRef, Deref, DerefMut, From};

#[cfg(feature = "image")]
use crate::ImageFile;
use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Webp {
    file: FileBase,
}

impl Webp {
    pub fn new(file: impl AsRef<std::path::Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for Webp {
    fn ext() -> &'static [&'static str] {
        &["webp"]
    }

    fn make_new(file: impl AsRef<std::path::Path>) -> Self {
        Self {
            file: FileBase::new_with_handler::<Self>(file),
        }
    }
}

#[cfg(feature = "image")]
impl ImageFile for Webp {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::WebP
    }
}

impl From<&str> for Webp {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
