use bdk::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema, PartialEq, Eq)]
pub struct File {
    #[serde(default = "generate_file_id")]
    pub id: String,
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

fn generate_file_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

impl Default for File {
    fn default() -> Self {
        File {
            id: generate_file_id(),
            name: String::new(),
            size: String::new(),
            ext: FileExtension::JPG,
            url: None,
        }
    }
}

#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema, PartialEq, Eq, Translate,
)]
pub enum FileExtension {
    #[serde(alias = "jpg", alias = "jpeg", alias = "JPEG")]
    #[translate(ko = "JPG", en = "JPG")]
    JPG = 1,

    #[serde(alias = "png", alias = "PNG")]
    #[translate(ko = "PNG", en = "PNG")]
    PNG = 2,

    #[serde(alias = "pdf", alias = "PDF")]
    #[translate(ko = "PDF", en = "PDF")]
    PDF = 3,

    #[serde(alias = "zip", alias = "ZIP")]
    #[translate(ko = "ZIP", en = "ZIP")]
    ZIP = 4,

    #[serde(alias = "doc", alias = "docx", alias = "word", alias = "WORD")]
    #[translate(ko = "WORD", en = "WORD")]
    WORD = 5,

    #[serde(alias = "ppt", alias = "pptx", alias = "PPTX")]
    #[translate(ko = "PPTX", en = "PPTX")]
    PPTX = 6,

    #[serde(alias = "xls", alias = "xlsx", alias = "excel", alias = "EXCEL")]
    #[translate(ko = "EXCEL", en = "EXCEL")]
    EXCEL = 7,

    #[serde(alias = "mp4", alias = "MP4")]
    #[translate(ko = "MP4", en = "MP4")]
    MP4 = 8,

    #[serde(alias = "mov", alias = "MOV")]
    #[translate(ko = "MOV", en = "MOV")]
    MOV = 9,

    #[serde(alias = "mkv", alias = "MKV")]
    #[translate(ko = "MKV", en = "MKV")]
    MKV = 10,
}
