use bdk::prelude::*;

use crate::Error;

#[api_model(base = "/v1/assets", database = skip)]
pub struct AssetPresignedUris {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
    #[api_model(read_action = get_presigned_uris)]
    pub total_count: usize,

    #[api_model(read_action = get_presigned_uris)]
    pub file_type: FileType,
}

#[derive(
    Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Default, PartialEq, Translate,
)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum FileType {
    // Image
    #[default]
    None,
    PNG,
    JPG,
    GIF,
    WEBM,
    SVG,
    AI,

    // 3D Model
    GLB,
    GLTF,

    // Audio
    MP3,
    WAV,

    // Video
    MP4,

    // Etc
    PPTX,
}

impl FileType {
    pub fn from_str(s: &str) -> Result<FileType, Error> {
        match s {
            "jpg" | "jpeg" => Ok(FileType::JPG),
            "png" => Ok(FileType::PNG),
            _ => Err(Error::InvalidType),
        }
    }
}
