use crate::{define_file, define_image_file};

define_file!(Exr, ["exr"]);
define_image_file!(Exr, image::ImageFormat::OpenExr);