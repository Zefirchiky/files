use crate::{define_file, define_image_file};

define_file!(Bmp, ["bmp", "dib"]);
define_image_file!(Bmp, image::ImageFormat::Bmp);