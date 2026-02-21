use crate::{File, Image, Jpeg, Json, Md, Png, Toml, webp::WebP};

#[derive(Debug)]
pub enum FileType {
    File(File),
    Image(Image),
    Json(Json),
    Toml(Toml),
    Md(Md),
    Jpeg(Jpeg),
    Png(Png),
    WebP(WebP),
}

pub enum TextTypes {
    File(File),
    Json(Json),
    Toml(Toml),
    Md(Md),
}

#[cfg(feature = "serde")]
pub enum ModelTypes {
    Json(Json),
    Toml(Toml),
}

#[cfg(feature = "image")]
pub enum ImageType {
    Image(Image),
    Jpeg(Jpeg),
    Png(Png),
    WebP(WebP),
}
