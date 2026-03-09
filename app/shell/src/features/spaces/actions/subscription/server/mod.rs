use crate::features::spaces::actions::subscription::*;

pub fn serve(app: fn() -> Element) {
    let config = config::get();

    dioxus::serve(move || {
        let cli = config.common.dynamodb();
        let session_layer = crate::common::middlewares::session_layer::get_session_layer(
            cli,
            config.common.env.to_string(),
        );
        async move {
            let dioxus_router = dioxus::server::router(app);
            Ok(dioxus_router.layer(session_layer))
        }
    });
}
