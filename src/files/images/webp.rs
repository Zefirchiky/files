use derive_more::{AsRef, Deref, DerefMut, From};

#[cfg(feature = "image")]
use crate::ImageFileTrait;
use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
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
            file: FileBase::new_with_handler::<Self>(file)
        }
    }
}

#[cfg(feature = "image")]
impl ImageFileTrait for Webp {}

impl From<&str> for Webp {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

