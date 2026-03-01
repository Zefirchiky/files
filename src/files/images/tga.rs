use crate::{define_file, define_image_file};

define_file!(Tga, ["tga"]);
define_image_file!(Tga, image::ImageFormat::Tga);