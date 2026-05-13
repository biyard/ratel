//! `open_external_url` Tauri command.

use app_shell::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn open_external_url(
    app: tauri::AppHandle,
    req: ExternalUrlRequest,
) -> Result<ExternalUrlResponse, ExternalUrlError> {
    // Basic URL sanity check — reject anything that doesn't look like an
    // http(s) URL. The plugin itself does some validation but surface a
    // typed error to the web caller.
    if !(req.url.starts_with("http://") || req.url.starts_with("https://")) {
        return Err(ExternalUrlError::InvalidUrl(req.url));
    }

    app.opener()
        .open_url(&req.url, None::<&str>)
        .map_err(|e| ExternalUrlError::OpenerFailed(e.to_string()))?;

    Ok(ExternalUrlResponse { opened: true })
}
