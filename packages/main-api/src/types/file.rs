use bdk::prelude::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, JsonSchema, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
}

impl Default for File {
    fn default() -> Self {
        File {
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
    #[translate(ko = "JPG", en = "JPG")]
    JPG = 1,
    #[translate(ko = "PNG", en = "PNG")]
    PNG = 2,
    #[translate(ko = "PDF", en = "PDF")]
    PDF = 3,
    #[translate(ko = "ZIP", en = "ZIP")]
    ZIP = 4,
    #[translate(ko = "WORD", en = "WORD")]
    WORD = 5,
    #[translate(ko = "PPTX", en = "PPTX")]
    PPTX = 6,
    #[translate(ko = "EXCEL", en = "EXCEL")]
    EXCEL = 7,
    #[translate(ko = "MP4", en = "MP4")]
    MP4 = 8,
    #[translate(ko = "MOV", en = "MOV")]
    MOV = 9,
}
