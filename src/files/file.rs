use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{FileBase, FileTrait};

#[derive(Debug, Clone, Default, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct File {
    file: FileBase<Self>,
}

impl File {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for File {
    fn ext() -> &'static [&'static str] {
        &[""]
    }
}
