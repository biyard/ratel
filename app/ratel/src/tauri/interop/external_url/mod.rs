//! Web-side caller for the `open_external_url` Tauri command.

use dioxus::document::eval as dx_eval;

use crate::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};

/// Open `url` in the user's default external browser via the Tauri host.
///
/// Returns `Err` if the URL is malformed or `tauri-plugin-opener` fails.
/// Only callable from a tauri-web build — there's no fallback path here
/// because the web build compiles a different module.
pub async fn open(req: ExternalUrlRequest) -> Result<ExternalUrlResponse, ExternalUrlError> {
    let mut runner = dx_eval(include_str!("open.js"));
    runner
        .send(serde_json::to_value(&req).map_err(|e| {
            ExternalUrlError::OpenerFailed(format!("serialize request: {e}"))
        })?)
        .map_err(|e| ExternalUrlError::OpenerFailed(format!("eval send: {e}")))?;

    // The JS driver sends one of two shapes:
    //   { "ok": ExternalUrlResponse }
    //   { "err": ExternalUrlError }
    // We deserialize to a tagged enum and convert into a Result.
    let outcome: Outcome = runner
        .recv::<Outcome>()
        .await
        .map_err(|e| ExternalUrlError::OpenerFailed(format!("eval recv: {e}")))?;
    outcome.into()
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Outcome {
    Ok(ExternalUrlResponse),
    Err(ExternalUrlError),
}

impl From<Outcome> for Result<ExternalUrlResponse, ExternalUrlError> {
    fn from(o: Outcome) -> Self {
        match o {
            Outcome::Ok(r) => Ok(r),
            Outcome::Err(e) => Err(e),
        }
    }
}
