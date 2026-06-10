use crate::*;

pub fn use_origin() -> Signal<String> {
    use_signal(|| {
        // Tauri WebView: the page origin is the internal `tauri.localhost`
        // scheme, which is NOT a real, externally-reachable host. Anything we
        // surface to the user as a URL (the MCP `claude mcp add` endpoint, post
        // share links) must use the configured backend base URL instead.
        #[cfg(feature = "tauri-web")]
        {
            // Inline the same compile-time backend base URL the tauri-web
            // transport uses (`MOBILE_API_URL`, baked by the Makefile). Inlined
            // rather than calling `common::fullstack::api_base_url` because that
            // shim module is `#[cfg(not(feature = "fullstack"))]` and disappears
            // in fullstack builds — this expression has no such dependency.
            option_env!("MOBILE_API_URL")
                .unwrap_or("https://dev.ratel.foundation")
                .to_string()
        }

        #[cfg(all(not(feature = "server"), not(feature = "tauri-web")))]
        {
            let origin = web_sys::window()
                .and_then(|w| w.location().origin().ok())
                .unwrap_or_default();
            origin.to_string()
        }

        #[cfg(all(feature = "server", not(feature = "tauri-web")))]
        {
            use dioxus::fullstack::FullstackContext;
            let Some(ctx) = FullstackContext::current() else {
                return "".to_string();
            };

            let parts = ctx.parts_mut();
            let origin = parts
                .headers
                .get("host")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default();

            let is_https = parts
                .headers
                .get("referer")
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
                .starts_with("https://");

            format!("{}://{}", if is_https { "https" } else { "http" }, origin)
        }
    })
}
