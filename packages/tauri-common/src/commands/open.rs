//! DTO for the `open_external_url` Tauri command.
//!
//! v1 demonstrative bridge. The native handler wraps `tauri-plugin-opener` so
//! the dioxus-web bundle can open a URL in the user's default browser instead
//! of inside the WebView (where target=_blank does nothing useful on Android).

use serde::{Deserialize, Serialize};

use crate::define_invoke_tauri;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalUrlRequest {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalUrlResponse {
    pub opened: bool,
}

define_invoke_tauri!(
    open,
    "open_external_url",
    ExternalUrlRequest,
    ExternalUrlResponse
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_url_request_roundtrip() {
        let req = ExternalUrlRequest {
            url: "https://example.com".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let back: ExternalUrlRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, back);
    }

    #[test]
    fn external_url_response_roundtrip() {
        let resp = ExternalUrlResponse { opened: true };
        let json = serde_json::to_string(&resp).unwrap();
        let back: ExternalUrlResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, back);
    }
}
