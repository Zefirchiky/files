use derive_more::{AsRef, Deref, DerefMut, From};

#[cfg(feature = "image")]
use crate::ImageFile;
use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Png {
    file: FileBase<Self>,
}

impl Png {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Png {
    fn ext() -> &'static [&'static str] {
        &["png"]
    }
}

#[cfg(feature = "image")]
impl ImageFile for Png {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Png
    }
}
