//pbm, pgm, ppm and pam
use crate::{define_file, define_image_file};

define_file!(Pnm, ["pnm", "pbm", "ppm", "pam"]);
define_image_file!(Pnm, image::ImageFormat::Pnm);