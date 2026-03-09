use crate::common::*;

pub fn run(app: fn() -> Element) {
    let cfg = crate::common::CommonConfig::default();

    crate::common::logger::init(cfg.log_level.into()).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    launch(app);

    #[cfg(feature = "server")]
    serve(app);
}

#[cfg(not(feature = "server"))]
fn launch(app: fn() -> Element) {
    dioxus::launch(app);
}

#[cfg(feature = "server")]
fn serve(app: fn() -> Element) {
    let cfg = crate::common::CommonConfig::default();

    let cli = cfg.dynamodb();
    let session_layer =
        crate::common::middlewares::session_layer::get_session_layer(cli, cfg.env.to_string());

    let dioxus_router = dioxus::server::router(app);
    let app = dioxus_router.layer(session_layer);

    #[cfg(not(feature = "lambda"))]
    dioxus::serve(move || {
        let app = app.clone();

        async move { Ok(app) }
    });

    #[cfg(feature = "lambda")]
    {
        use lambda_http::run;

        let app_future = async move { run(app).await };

        info!("Starting server in Lambda environment");
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let _ = handle.block_on(app_future);
        } else {
            let _ = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(app_future);
        }
    }
}
