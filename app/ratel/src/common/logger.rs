use tracing::{
    Level,
    subscriber::{SetGlobalDefaultError, set_global_default},
};

#[allow(unused_variables)]
pub fn init(level: Level) -> Result<(), SetGlobalDefaultError> {
    #[cfg(target_arch = "wasm32")]
    {
        use tracing_subscriber::Registry;
        use tracing_subscriber::layer::SubscriberExt;

        let layer_config = tracing_wasm::WASMLayerConfigBuilder::new()
            .set_max_level(level)
            .build();
        let layer = tracing_wasm::WASMLayer::new(layer_config);
        let reg = Registry::default().with(layer);

        set_global_default(reg)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use tracing_subscriber::EnvFilter;

        let sub = tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::from(level.to_string().to_ascii_uppercase().as_str())
                    .add_directive("rustls=info".parse().unwrap())
                    .add_directive("aws=info".parse().unwrap())
                    .add_directive("hyper_util=info".parse().unwrap())
                    .add_directive("tower_http=info".parse().unwrap()),
            )
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_ansi(
                option_env!("DISABLE_ANSI")
                    .map(|e| e.to_lowercase() != "true")
                    .unwrap_or(true),
            );

        if !dioxus_cli_config::is_cli_enabled() {
            return set_global_default(sub.finish());
        }

        // todo(jon): this is a small hack to clean up logging when running under the CLI
        // eventually we want to emit everything as json and let the CLI manage the parsing + display
        set_global_default(sub.without_time().with_target(false).finish())
    }
}
