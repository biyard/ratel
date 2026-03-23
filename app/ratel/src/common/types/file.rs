use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub struct File {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub size: String,
    pub ext: FileExtension,
    pub url: Option<String>,
    #[serde(default)]
    pub uploader_name: Option<String>,
    #[serde(default)]
    pub uploader_profile_url: Option<String>,
    #[serde(default)]
    pub uploaded_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum FileExtension {
    #[default]
    #[serde(alias = "jpg", alias = "jpeg", alias = "JPEG")]
    JPG,
    #[serde(alias = "png", alias = "PNG")]
    PNG,
    #[serde(alias = "pdf", alias = "PDF")]
    PDF,
    #[serde(alias = "zip", alias = "ZIP")]
    ZIP,
    #[serde(alias = "doc", alias = "docx", alias = "word", alias = "WORD")]
    WORD,
    #[serde(alias = "ppt", alias = "pptx", alias = "PPTX")]
    PPTX,
    #[serde(alias = "xls", alias = "xlsx", alias = "excel", alias = "EXCEL")]
    EXCEL,
    #[serde(alias = "mp4", alias = "MP4")]
    MP4,
    #[serde(alias = "mov", alias = "MOV")]
    MOV,
    #[serde(alias = "mkv", alias = "MKV")]
    MKV,
}

impl FileExtension {
    /// Parse a file extension string (e.g., "pdf", "docx") into a `FileExtension`.
    /// Returns `FileExtension::default()` (JPG) for unrecognised extensions.
    pub fn from_ext_str(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" => Self::JPG,
            "png" => Self::PNG,
            "pdf" => Self::PDF,
            "zip" => Self::ZIP,
            "doc" | "docx" => Self::WORD,
            "ppt" | "pptx" => Self::PPTX,
            "xls" | "xlsx" => Self::EXCEL,
            "mp4" => Self::MP4,
            "mov" => Self::MOV,
            "mkv" => Self::MKV,
            _ => Self::default(),
        }
    }

    /// Derive a `FileExtension` from the original filename, falling back to the
    /// URL when the filename has no extension. Uses `std::path::Path::extension()`
    /// for robust parsing (ignores dot-less names, hidden files like `.gitignore`, etc.).
    pub fn from_name_or_url(name: &str, url: &str) -> Self {
        // Try the filename first
        if let Some(ext) = std::path::Path::new(name).extension().and_then(|e| e.to_str()) {
            return Self::from_ext_str(ext);
        }
        // Fall back to URL (strip query string, then extract extension)
        let path = url.split('?').next().unwrap_or(url);
        if let Some(ext) = std::path::Path::new(path).extension().and_then(|e| e.to_str()) {
            return Self::from_ext_str(ext);
        }
        Self::default()
    }
}

/// Extract a human-readable filename from a URL, falling back to `"untitled"`.
pub fn extract_filename_from_url(url: &str) -> String {
    let path = url.split('?').next().unwrap_or(url);
    // Trim trailing slashes so URLs like "https://example.com/path/" fall back correctly.
    let trimmed = path.trim_end_matches('/');

    if trimmed.is_empty() {
        return "untitled".to_string();
    }

    trimmed
        .rsplit('/')
        .next()
        .filter(|segment| !segment.is_empty())
        .unwrap_or("untitled")
        .to_string()
}
