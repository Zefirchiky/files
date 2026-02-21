use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{FileBase, FileTrait};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[as_ref(forward)]
#[from(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image {
    file: FileBase<Self>,
}

impl Image {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Image {
    fn ext() -> &'static [&'static str] {
        &[""]
    }
}
