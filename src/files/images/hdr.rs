use crate::{define_file, define_image_file};

define_file!(Hdr, ["hdr"]);
define_image_file!(Hdr, image::ImageFormat::Hdr);