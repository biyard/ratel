//! `open_external_url` Tauri command. Real implementation in Task 5.1.

use app_shell::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};

#[tauri::command]
pub fn open_external_url(req: ExternalUrlRequest) -> Result<ExternalUrlResponse, ExternalUrlError> {
    let _ = req;
    Ok(ExternalUrlResponse { opened: false })
}
