use serde::{Deserialize, Deserializer, Serialize};

use super::ContentDocument;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(tag = "content_type", content = "data", rename_all = "snake_case")]
pub enum ContentBody {
    StructuredContent(ContentDocument),
    HtmlContent(String),
}

impl Default for ContentBody {
    fn default() -> Self {
        ContentBody::HtmlContent(String::new())
    }
}

impl ContentBody {
    pub fn html<S: Into<String>>(s: S) -> Self {
        ContentBody::HtmlContent(s.into())
    }

    pub fn structured(doc: ContentDocument) -> Self {
        ContentBody::StructuredContent(doc)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ContentBody::HtmlContent(s) => s.trim().is_empty(),
            ContentBody::StructuredContent(d) => d.blocks.is_empty(),
        }
    }
}

impl From<String> for ContentBody {
    fn from(s: String) -> Self {
        ContentBody::HtmlContent(s)
    }
}

impl From<&str> for ContentBody {
    fn from(s: &str) -> Self {
        ContentBody::HtmlContent(s.to_string())
    }
}

// Custom Deserialize: accept three shapes.
//   1. JSON string                                 -> HtmlContent
//   2. {"content_type":"html_content","data":...}  -> tagged
//   3. {"content_type":"structured_content",...}   -> tagged
impl<'de> Deserialize<'de> for ContentBody {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = serde_json::Value::deserialize(deserializer)?;

        match v {
            serde_json::Value::String(s) => Ok(ContentBody::HtmlContent(s)),
            other => {
                #[derive(Deserialize)]
                #[serde(tag = "content_type", content = "data", rename_all = "snake_case")]
                enum Tagged {
                    StructuredContent(ContentDocument),
                    HtmlContent(String),
                }
                let tagged: Tagged =
                    serde_json::from_value(other).map_err(serde::de::Error::custom)?;
                Ok(match tagged {
                    Tagged::StructuredContent(d) => ContentBody::StructuredContent(d),
                    Tagged::HtmlContent(s) => ContentBody::HtmlContent(s),
                })
            }
        }
    }
}
