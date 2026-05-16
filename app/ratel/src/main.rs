fn main() {
    // Print Rust panic location + message to the WebView console instead of
    // bubbling up as opaque `RuntimeError: unreachable`. Must run before any
    // code that might panic during init. `set_server_url` itself was a
    // duplicate of the call in `common::run`, which uses dioxus-fullstack's
    // `OnceLock`-backed setter — calling it twice panics on Err(value).
    // #[cfg(not(feature = "server"))]
    // std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    app_shell::common::run(app_shell::App);
}
