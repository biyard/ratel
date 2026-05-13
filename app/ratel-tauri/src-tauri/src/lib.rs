mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // Third-party cookie support for Android WebView is wired in
            // Task 7.3 once we have an APK to iterate on. For now this is
            // a no-op so the shell crate compiles cleanly on the host.
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::external_url::open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
