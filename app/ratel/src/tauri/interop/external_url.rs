//! Web-side caller for the `open_external_url` Tauri command.

use crate::tauri::invoke::{InvokeError, invoke};
use crate::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};

/// Open `url` in the user's default external browser via the Tauri host.
///
/// Returns `Err` if the URL is malformed or `tauri-plugin-opener` fails.
/// Only callable from a tauri-web build — there's no fallback path here
/// because the web build compiles a different module.
pub async fn open(req: ExternalUrlRequest) -> Result<ExternalUrlResponse, ExternalUrlError> {
    invoke::<_, ExternalUrlResponse>("open_external_url", req)
        .await
        .map_err(invoke_err_into)
}

fn invoke_err_into(e: InvokeError) -> ExternalUrlError {
    match e {
        InvokeError::CommandFailed(msg) => {
            // The native command may return a typed ExternalUrlError; try to
            // recover it from the message, otherwise wrap as OpenerFailed.
            serde_json::from_str::<ExternalUrlError>(&msg)
                .unwrap_or(ExternalUrlError::OpenerFailed(msg))
        }
        other => ExternalUrlError::OpenerFailed(other.to_string()),
    }
}
