use derive_more::{AsRef, Deref, DerefMut, From};

#[cfg(feature = "image")]
use crate::ImageFile;
use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Png {
    file: FileBase,
}

impl Png {
    pub fn new(file: impl AsRef<std::path::Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for Png {
    fn ext() -> &'static [&'static str] {
        &["png"]
    }

    fn make_new(file: impl AsRef<std::path::Path>) -> Self {
        Self {
            file: FileBase::new_with_handler::<Self>(file),
        }
    }
}

#[cfg(feature = "image")]
impl ImageFile for Png {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Png
    }
}

impl From<&str> for Png {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
