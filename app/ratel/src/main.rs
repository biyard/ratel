fn main() {
    #[cfg(feature = "mobile")]
    {
        use app_shell::common::CommonConfig;

        let endpoint = CommonConfig::default().env.mobile_endpoint();
        dioxus::fullstack::set_server_url(endpoint);

        // Force `Accept: application/json` on every server-function call from
        // the mobile binary. Without this, dioxus-server's PRG (Post-Redirect-Get)
        // middleware in `dioxus-server-0.7.6/src/serverfn.rs:120-128` sees a
        // request with `Accept: text/html` + a `Referer` header and rewrites
        // the successful 200 response into a 302 pointing at Referer, turning
        // every server function call into an infinite redirect loop on mobile.
        // The PRG path is intended for HTML form posts; mobile reqwest must
        // opt out by advertising JSON only.
        let mut headers = dioxus::fullstack::HeaderMap::new();
        headers.insert(
            dioxus::fullstack::http::header::ACCEPT,
            dioxus::fullstack::HeaderValue::from_static("application/json"),
        );
        dioxus::fullstack::set_request_headers(headers);
    }

    app_shell::common::run(app_shell::App);
}
