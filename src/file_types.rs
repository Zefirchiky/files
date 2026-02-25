use std::path::Path;

use crate::{FileTrait, define_file_types};

define_file_types! {
    FileType,
    File,
    Image,
    #[cfg(feature = "json")] Json,
    #[cfg(feature = "toml")] Toml,
    #[cfg(feature = "md")]   Md,
    #[cfg(feature = "jpeg")] Jpeg,
    #[cfg(feature = "png")]  Png,
    #[cfg(feature = "webp")] WebP,
}

define_file_types! {
    TextTypes,
    File,
    #[cfg(feature = "json")] Json,
    #[cfg(feature = "toml")] Toml,
    #[cfg(feature = "md")]   Md,
}

#[cfg(feature = "serde")]
#[derive(Debug)]
pub enum ModelTypes {
    #[cfg(feature = "json")]
    Json(crate::Json),
    #[cfg(feature = "toml")]
    Toml(crate::Toml),
}

#[cfg(feature = "serde")]
impl ModelTypes {
    pub fn from_ext(path: &impl AsRef<Path>) -> Option<Self> {
        let path_ref = path.as_ref();
        
        if let Some(ext) = path_ref.extension().and_then(|s| s.to_str()) {
            if crate::Json::ext().contains(&ext) {
                return Some(Self::Json(crate::Json::new(&path_ref)));
            } else if crate::Toml::ext().contains(&ext) {
                return Some(Self::Toml(crate::Toml::new(&path_ref)));
            }
        }
        None
    }
}

// impl crate::ModelFile for ModelFile {
    
// }

#[cfg(feature = "image")]
pub enum ImageType {
    Image(crate::Image),
    #[cfg(feature = "jpeg")]
    Jpeg(crate::Jpeg),
    #[cfg(feature = "png")]
    Png(crate::Png),
    #[cfg(feature = "webp")]
    WebP(crate::WebP),
}

#[cfg(test)]
mod file_types {
    use crate::{FileType, Json};

    #[test]
    fn from_ext() {
        let file = FileType::from_ext("file.json");
        assert_eq!(file, FileType::Json(Json::new(&"file.json")))
    }
}
