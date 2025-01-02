#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_oauth::component::OAuthPopup;

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},

    #[route("/oauth/kakao")]
    OAuthPopup {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::DEBUG).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        KakaoLogin {}
    }
}

#[component]
pub fn KakaoLogin() -> Element {
    rsx! {
        button {
            onclick: move |_| async move {
                let client = dioxus_oauth::prelude::OAuthClient::new(
                        env!("CLIENT_ID"),
                        option_env!("REDIRECT_URI")
                            .unwrap_or("http://localhost:8000/oauth/kakao"),
                        "https://kauth.kakao.com/oauth/authorize",
                        "https://kauth.kakao.com/oauth/token",
                    )
                    .set_openid_url("https://kauth.kakao.com/oauth/tokeninfo");
                let code: String = match client.get_auth_code().await {
                    Ok(code) => code,
                    Err(e) => {
                        tracing::error!("Auth code failed: {:?}", e);
                        return;
                    }
                };
                let token_response: dioxus_oauth::prelude::TokenResponse = match client
                    .get_token(code.as_str())
                    .await
                {
                    Ok(token_response) => token_response,
                    Err(e) => {
                        tracing::error!("Token response failed: {:?}", e);
                        return;
                    }
                };
                tracing::debug!("Token response: {:?}", token_response);
                let oid_response: dioxus_oauth::prelude::OpenIdResponse = match client
                    .get_openid(&token_response.id_token)
                    .await
                {
                    Ok(oid_response) => oid_response,
                    Err(e) => {
                        tracing::error!("Token response failed: {:?}", e);
                        return;
                    }
                };
                tracing::debug!("OID response: {:?}", oid_response);
            },
            "Kakao Login"
        }
    }
}
