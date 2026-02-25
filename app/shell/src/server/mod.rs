use crate::*;

pub fn serve(app: fn() -> Element) {
    let config = config::get();

    #[cfg(not(feature = "lambda"))]
    dioxus::serve(move || {
        let cli = config.common.dynamodb();
        let session_layer = common::middlewares::session_layer::get_session_layer(
            cli,
            config.common.env.to_string(),
        );

        async move {
            let dioxus_router = dioxus::server::router(app);
            let app = dioxus_router.layer(session_layer);

            Ok(app)
        }
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
