use askama::Template;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use bdk::prelude::*;

use crate::AppState;

#[derive(Debug, Clone, Default)]
pub enum IndexType {
    #[default]
    IndexOnlyThisPage, // index, nofollow
    IndexAllPages,   // index, follow
    NoIndexThisPage, // noindex, follow
    NoIndexAllPages, // noindex, nofollow
    Block,           // disallow
}

#[derive(Debug, Template)]
#[template(path = "index.html")]
pub struct IndexTmpl {
    title: String,
    index_js: &'static str,
    index_css: &'static str,
    boot_json: String,
    index_type: IndexType,
    canonical_url: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
}

impl IndexTmpl {
    pub fn new(title: impl Into<String>) -> Self {
        IndexTmpl {
            title: title.into(),
            index_js: option_env!("WEB_INDEX_JS").unwrap_or("index.js"),
            index_css: option_env!("WEB_INDEX_CSS").unwrap_or("index.css"),
            boot_json: "{}".to_string(),
            index_type: IndexType::IndexOnlyThisPage,
            canonical_url: None,
            description: None,
            image_url: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_image_url(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    pub fn with_boot_json(mut self, boot_json: impl Into<String>) -> Self {
        self.boot_json = boot_json.into();
        self
    }

    pub fn with_index_type(mut self, index_type: IndexType) -> Self {
        self.index_type = index_type;
        self
    }

    pub fn with_canonical_url(mut self, canonical_url: impl Into<String>) -> Self {
        self.canonical_url = Some(canonical_url.into());
        self
    }
}

impl FromRequestParts<AppState> for IndexTmpl {
    type Rejection = crate::Error2;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("extracting IndexTmpl from request parts");
        // extract `path` from parts
        let mut path = parts.uri.path().to_string();

        if let Some(q) = parts.uri.query() {
            path = format!("{}?{}", path, q);
        }

        let host = if crate::config::get().env == "prod" {
            "ratel.foundation"
        } else {
            "dev.ratel.foundation"
        };

        Ok(Self::new("Ratel").with_canonical_url(format!("https://{host}{path}")))
    }
}
