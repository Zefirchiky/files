use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{FileBase, FileTrait};

#[derive(Debug, Clone, Default, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Md {
    file: FileBase<Self>,
}

impl Md {
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Md {
    fn ext() -> &'static [&'static str] {
        &["md"]
    }
}
