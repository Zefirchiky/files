use crate::{define_file, define_image_file};

define_file!(Tiff, ["tiff", "tif"]);
define_image_file!(Tiff, image::ImageFormat::Tiff);
