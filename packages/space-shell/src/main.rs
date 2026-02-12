use dioxus::prelude::*;
use space_shell::*;

#[cfg(feature = "server")]
use common::utils::aws::{dynamo::DynamoClient, get_aws_config};

fn main() {
    let config = config::get();
    dioxus::logger::init(config.log_level).expect("logger failed to init");

    #[cfg(not(feature = "server"))]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    {
        let common::by_types::config::DatabaseConfig::DynamoDb { endpoint, .. } = &config.dynamodb
        else {
            panic!("Only DynamoDB is supported");
        };

        let aws_sdk_config = get_aws_config(
            config.aws.access_key_id.to_string(),
            config.aws.secret_access_key.to_string(),
            config.aws.region.to_string(),
        );
        let dynamo_client = DynamoClient::new(&aws_sdk_config, endpoint.map(|e| e.to_string()));
        let state = AppState {
            upstream_url: config.upstream_url.to_string(),
        };
        dioxus::serve(move || {
            let dynamo_client = dynamo_client.clone();
            let state = state.clone();
            let session_layer = common::middlewares::session_layer::get_session_layer(
                &dynamo_client,
                //FIXME: use "ENV"
                "local".to_string(),
            );
            async move {
                use common::axum::Extension;

                Ok(dioxus::server::router(App)
                    .layer(session_layer)
                    .layer(Extension(dynamo_client))
                    .layer(Extension(state)))
            }
        });
    }
}

pub const FAVICON: Asset = asset!("/assets/favicon.ico");
pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}
