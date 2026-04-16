fn main() {
    #[cfg(feature = "mobile")]
    {
        use app_shell::common::CommonConfig;

        let endpoint = CommonConfig::default().env.mobile_endpoint();
        dioxus::fullstack::set_server_url(endpoint);
    }

    app_shell::common::run(app_shell::App);
}
