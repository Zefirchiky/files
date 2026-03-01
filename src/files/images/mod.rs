#[cfg(feature = "jpeg")]
mod jpeg;
#[cfg(feature = "png")]
mod png;
#[cfg(feature = "webp")]
mod webp;
#[cfg(feature = "avif")]
mod avif;
#[cfg(feature = "tiff")]
mod tiff;
#[cfg(feature = "gif")]
mod gif;
#[cfg(feature = "bmp")]
mod bmp;
#[cfg(feature = "exr")]
mod exr;
#[cfg(feature = "ff")]
mod ff;
#[cfg(feature = "hdr")]
mod hdr;
#[cfg(feature = "ico")]
mod ico;
#[cfg(feature = "pnm")]
mod pnm;
#[cfg(feature = "qoi")]
mod qoi;
#[cfg(feature = "tga")]
mod tga;

#[cfg(feature = "jpeg")]
pub use jpeg::*;
#[cfg(feature = "png")]
pub use png::*;
#[cfg(feature = "webp")]
pub use webp::*;
#[cfg(feature = "avif")]
pub use avif::*;
#[cfg(feature = "tiff")]
pub use tiff::*;
#[cfg(feature = "gif")]
pub use gif::*;
#[cfg(feature = "bmp")]
pub use bmp::*;
#[cfg(feature = "exr")]
pub use exr::*;
#[cfg(feature = "ff")]
pub use ff::*;
#[cfg(feature = "hdr")]
pub use hdr::*;
#[cfg(feature = "ico")]
pub use ico::*;
#[cfg(feature = "pnm")]
pub use pnm::*;
#[cfg(feature = "qoi")]
pub use qoi::*;
#[cfg(feature = "tga")]
pub use tga::*;
