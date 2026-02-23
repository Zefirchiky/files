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
