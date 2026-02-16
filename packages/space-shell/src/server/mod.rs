use crate::*;
use common::utils::aws::{dynamo::DynamoClient, get_aws_config};

pub fn serve(app: fn() -> Element) {
    use common::utils::aws::dynamo::DynamoBuilder;
    let config = config::get();

    dioxus::serve(move || {
        let cli = config.common.dynamodb();
        let session_layer = common::middlewares::session_layer::get_session_layer(
            cli,
            config.common.env.to_string(),
        );
        async move {
            use common::axum::Extension;

            Ok(dioxus::server::router(app).layer(session_layer))
        }
    });
}
