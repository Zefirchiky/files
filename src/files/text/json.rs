use derive_more::{AsRef, Deref, DerefMut, From};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{FileBase, FileTrait};
#[cfg(feature = "serde")]
use crate::{ModelFile, model_file::ModelIoError};

#[derive(Debug, thiserror::Error)]
pub enum ModelJsonIoError {
    #[cfg(feature = "serde")]
    #[error("Seder Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(feature = "serde")]
impl ModelIoError for ModelJsonIoError {}

#[derive(Debug, Clone, Default, From, AsRef, Deref, DerefMut)]
#[from(forward)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Json {
    file: FileBase<Self>,
}

impl Json {
    /// Creates a new JsonHandler for the given file.
    ///
    /// If the file does not exist, it will be created. If the parent directories do not exist, they will be created.
    ///
    /// # Panics
    ///
    /// Panics if the path exists but is not a file, or if the file does not have the correct extension.
    pub fn new(path: impl AsRef<std::path::Path>) -> Self {
        Self { file: FileBase::new(path) }
    }
}

impl FileTrait for Json {
    fn file_init_bytes() -> Option<&'static [u8]> {
        Some(b"{}")
    }

    fn ext() -> &'static [&'static str] {
        &["json"]
    }
}

#[cfg(feature = "serde")]
impl ModelFile for Json {
    type Error = ModelJsonIoError;
    
    fn bytes_to_model<T: for<'de> Deserialize<'de>>(data: Vec<u8>) -> Result<T, Self::Error> {
        Ok(serde_json::from_slice(&data)?)
    }
    
    fn model_to_bytes(model: &impl Serialize) -> Result<Vec<u8>, Self::Error> {
        Ok(serde_json::to_vec_pretty(model)?)
    }
}

#[cfg(test)]
mod json_tests {
    use std::fs;

    use crate::{Json, Temporary};

    static FILE_NAME_EXT: &str = "dis/init.json";
    static TEMP_FILE_NAME_EXT: &str = "dis/init_temp.json";
    static FILE_NAME: &str = "dis/init";
    static TEMP_FILE_NAME: &str = "dis/init_temp";

    #[test]
    fn init() {
        Json::new(FILE_NAME_EXT);
        assert!(
            fs::exists(FILE_NAME_EXT).unwrap_or(false),
            "File {FILE_NAME_EXT} was not created"
        );
    }

    #[test]
    #[should_panic]
    fn init_wrong_ext() {
        Json::new(FILE_NAME);
    }

    #[test]
    fn init_temp() {
        {
            let _tis = Temporary::new(Json::new(TEMP_FILE_NAME_EXT));
            assert!(
                fs::exists(TEMP_FILE_NAME_EXT).unwrap_or(false),
                "File {TEMP_FILE_NAME_EXT} was not created"
            );
        }
        assert!(
            !fs::exists(TEMP_FILE_NAME_EXT).unwrap_or(false),
            "File {TEMP_FILE_NAME_EXT} was not deleted"
        );
    }

    #[test]
    #[should_panic]
    fn init_temp_wrong_ext() {
        Temporary::new(Json::new(TEMP_FILE_NAME));
    }
}
