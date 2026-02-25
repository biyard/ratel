mod admin_routes;
mod proxy_middleware;
mod proxy_registry;

use crate::*;
use common::utils::aws::dynamo::DynamoClient;
use proxy_registry::ProxyRegistry;

pub fn serve(app: fn() -> Element) {
    let config = config::get();
    let registry = ProxyRegistry::new();

    dioxus::serve(move || {
        let cli = config.common.dynamodb();
        let session_layer = common::middlewares::session_layer::get_session_layer(
            cli,
            config.common.env.to_string(),
        );
        let registry = registry.clone();
        async move {
            use common::axum::{middleware, Extension};

            let dioxus_router = dioxus::server::router(app);
            let admin = admin_routes::admin_router(registry.clone());

            Ok(dioxus_router
                .merge(admin)
                .layer(middleware::from_fn(proxy_middleware::proxy_middleware))
                .layer(Extension(registry))
                .layer(session_layer))
        }
    });
}
