use std::io::Write;

use image::{DynamicImage, ImageReader};

use crate::FileTrait;

#[derive(Debug, thiserror::Error)]
pub enum ImageIoError {
    #[error("Image Error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait ImageFileTrait: FileTrait {
    fn save_image(&self, img: &DynamicImage) -> Result<(), image::ImageError> {
        img.save(&self)
    }

    fn load_image(&self) -> Result<DynamicImage, image::ImageError> {
        Ok(ImageReader::open(&self)?.decode()?)
    }
    
    fn image_format() -> image::ImageFormat;
}

pub trait ImageFileEncodingTrait: FileTrait {
    fn get_encoder_w_quality(w: impl Write, quality: u8) -> impl image::ImageEncoder;

    fn save_image_custom(
        &self,
        img: &image::DynamicImage,
        quality: u8,
    ) -> Result<(), ImageIoError> {
        let mut buf = vec![];
        img.write_with_encoder(Self::get_encoder_w_quality(&mut buf, quality))?;
        self.save(&buf)?;
        Ok(())
    }
}
