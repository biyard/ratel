fn main() {
    // Print Rust panic location + message to the WebView console instead of
    // bubbling up as opaque `RuntimeError: unreachable`. Must run before any
    // code that might panic during init.
    #[cfg(any(feature = "web", feature = "mobile", feature = "tauri-web"))]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    app_shell::common::run(app_shell::App);
}
