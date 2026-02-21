use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{FileBase, FileTrait};
#[cfg(feature = "image")]
use crate::{ImageFileEncoding, ImageFile};
#[cfg(all(feature = "image", feature = "async"))]
use crate::{ImageFileEncodingAsync, ImageFileAsync};

#[derive(Debug, Default, Clone, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Jpeg {
    file: FileBase,
}

impl Jpeg {
    pub fn new(file: impl AsRef<std::path::Path>) -> Self {
        Self::make_new(file)
    }
}

impl FileTrait for Jpeg {
    fn ext() -> &'static [&'static str] {
        &["jpeg"]
    }

    fn make_new(file: impl AsRef<std::path::Path>) -> Self {
        Self {
            file: FileBase::new_with_handler::<Self>(file),
        }
    }
}

#[cfg(feature = "image")]
impl ImageFile for Jpeg {
    fn image_format() -> image::ImageFormat {
        image::ImageFormat::Jpeg
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl ImageFileAsync for Jpeg {}

#[cfg(feature = "image")]
impl ImageFileEncoding for Jpeg {
    fn get_encoder_w_quality(
        w: impl std::io::Write,
        quality: u8,
    ) -> image::codecs::jpeg::JpegEncoder<impl std::io::Write> {
        image::codecs::jpeg::JpegEncoder::new_with_quality(w, quality)
    }
}

#[cfg(all(feature = "image", feature = "async"))]
impl ImageFileEncodingAsync for Jpeg {}

impl From<&str> for Jpeg {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}
