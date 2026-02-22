use derive_more::{AsRef, Deref, DerefMut, From};

use crate::{ImageQualityConfig, define_custom_quality_image, define_file, define_image_file};

#[cfg(feature = "image")]
#[derive(Debug, Clone, Copy)]
pub struct JpegConfig {
    quality: u8,
}

#[cfg(feature = "image")]
impl JpegConfig {
    pub fn new(quality: u8) -> Self {
        Self { quality }
    }
}

#[cfg(feature = "image")]
impl<'a> ImageQualityConfig<'a> for JpegConfig {
    type Encoder = image::codecs::jpeg::JpegEncoder<&'a mut Vec<u8>>;
    fn get_encoder(&self, w: &'a mut Vec<u8>) -> Self::Encoder {
        image::codecs::jpeg::JpegEncoder::new_with_quality(w, self.quality)
    }
}

define_file!(Jpeg, ["jpeg"], );
define_image_file!(Jpeg, image::ImageFormat::Jpeg);
define_custom_quality_image!(Jpeg, JpegConfig);

// #[cfg(test)]
// mod jpeg {
//     use crate::ImageQualityEncodingAsync;

//     use super::*;
    
//     #[test]
//     fn macros() {
//         Jpeg
//     }
// }
