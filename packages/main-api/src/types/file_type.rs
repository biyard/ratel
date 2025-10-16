use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    // Image
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
    MOV,

    // Etc
    PPTX,
}
