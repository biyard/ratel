use crate::*;
use common::utils::aws::{dynamo::DynamoClient, get_aws_config};

pub fn serve(app: fn() -> Element) {
    use common::utils::aws::dynamo::DynamoBuilder;
    let config = config::get();

    let common::by_types::config::DatabaseConfig::DynamoDb { endpoint, .. } = &config.dynamodb
    else {
        panic!("Only DynamoDB is supported");
    };

    let aws_sdk_config = get_aws_config(
        config.aws.access_key_id.to_string(),
        config.aws.secret_access_key.to_string(),
        config.aws.region.to_string(),
    );
    let dynamo_client = DynamoBuilder::new(&aws_sdk_config, endpoint.map(|e| e.to_string()));
    let app_state = AppState {
        upstream_url: config.upstream_url.to_string(),
    };
    dioxus::serve(move || {
        use common::middlewares::client_state::ClientState;
        let app_state = app_state.clone();
        let dynamo_client = dynamo_client.clone();
        let state = ClientState {
            dynamo: dynamo_client.clone(),
        };
        let session_layer = common::middlewares::session_layer::get_session_layer(
            &dynamo_client,
            //FIXME: use "ENV"
            "local".to_string(),
        );
        async move {
            use common::axum::Extension;

            Ok(dioxus::server::router(app)
                .layer(session_layer)
                .layer(Extension(app_state))
                .layer(Extension(state)))
        }
    });
}
