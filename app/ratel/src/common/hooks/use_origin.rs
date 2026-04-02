use crate::*;

pub fn use_origin() -> Signal<String> {
    #[cfg(not(feature = "server"))]
    {
        let origin = web_sys::window()
            .and_then(|w| w.location().origin().ok())
            .unwrap_or_default();
        use_signal(move || origin.to_string())
    }

    #[cfg(feature = "server")]
    {
        use dioxus::fullstack::FullstackContext;
        let Some(ctx) = FullstackContext::current() else {
            return use_signal(|| "".to_string());
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

        use_signal(move || format!("{}://{}", if is_https { "https" } else { "http" }, origin))
    }
}
