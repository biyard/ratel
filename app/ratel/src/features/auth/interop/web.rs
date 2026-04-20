use super::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

#[wasm_bindgen(module = "/src/features/auth/interop/web.js")]
extern "C" {
    pub fn init_firebase(conf: &JsValue);

    #[wasm_bindgen(js_name = signIn)]
    fn sign_in_promise() -> Promise;

    #[wasm_bindgen(js_name = detectInAppBrowser)]
    fn detect_in_app_browser_js() -> String;

    #[wasm_bindgen(js_name = escapeKakaoTalkInApp)]
    fn escape_kakaotalk_inapp_js();
}

pub async fn sign_in() -> crate::common::Result<super::UserInfo> {
    let js_value = JsFuture::from(sign_in_promise())
        .await
        .map_err(|_e| AuthError::UserInfoParseFailed)?;
    serde_wasm_bindgen::from_value(js_value).map_err(|_e| AuthError::UserInfoParseFailed.into())
}

/// In-app browser classification returned by the JS detector. Only
/// browsers that Google OAuth blocks with `disallowed_useragent` (or that
/// would break `signInWithPopup`) are enumerated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InAppBrowser {
    KakaoTalk,
    Instagram,
    Facebook,
    Line,
    Naver,
    Daum,
    Other,
}

impl InAppBrowser {
    fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "kakaotalk" => Some(Self::KakaoTalk),
            "instagram" => Some(Self::Instagram),
            "facebook" => Some(Self::Facebook),
            "line" => Some(Self::Line),
            "naver" => Some(Self::Naver),
            "daum" => Some(Self::Daum),
            "other-inapp" => Some(Self::Other),
            _ => None,
        }
    }
}

/// Returns `Some(kind)` if the current page is rendered inside a known
/// in-app browser that blocks Google OAuth.
pub fn detect_in_app_browser() -> Option<InAppBrowser> {
    let tag = detect_in_app_browser_js();
    InAppBrowser::from_tag(&tag)
}

/// Triggers KakaoTalk's `openExternal` scheme to move the user to their
/// default browser. Must be called from a user-gesture handler.
pub fn escape_kakaotalk_inapp() {
    escape_kakaotalk_inapp_js();
}
