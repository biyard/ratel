use bdk::prelude::*;

use crate::Error;

// #[api_model(base = "/v1/assets", database = skip)]
// pub struct AssetPresignedUris {
//     pub presigned_uris: Vec<String>,
//     pub uris: Vec<String>,
//     #[api_model(read_action = get_presigned_uris)]
//     pub total_count: usize,

//     #[api_model(read_action = get_presigned_uris)]
//     pub file_type: FileType,
// }

#[api_model(base = "/v1/assets", database = skip)]
pub struct AssetPresignedUris {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,

    #[api_model(read_action = get_presigned_uris)]
    pub total_count: usize,

    #[api_model(read_action = get_presigned_uris)]
    pub file_type: FileType,

    pub upload_id: Option<String>,
    pub key: Option<String>,
}

#[api_model(base = "/v1/assets/complete", database = skip)]
pub struct CompleteMultipartUploadRequest {
    pub upload_id: String,
    pub key: String,
    pub parts: Vec<UploadedPart>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UploadedPart {
    pub part_number: i32,
    pub etag: String,
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

    PDF,
    XLSX,

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
            "svg" => Ok(FileType::SVG),
            "pdf" => Ok(FileType::PDF),
            "xlsx" | "xls" => Ok(FileType::XLSX),
            "pptx" => Ok(FileType::PPTX),
            _ => Err(Error::InvalidType),
        }
    }
}
