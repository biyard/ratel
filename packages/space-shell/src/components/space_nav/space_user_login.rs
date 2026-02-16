use crate::{
    controllers::user::{oauth_login, OAuthLoginRequest},
    *,
};
use dioxus::fullstack::Form;

const FIREBASE_API_KEY: &str = match option_env!("FIREBASE_API_KEY") {
    Some(v) => v,
    None => "",
};

const FIREBASE_AUTH_DOMAIN: &str = match option_env!("FIREBASE_AUTH_DOMAIN") {
    Some(v) => v,
    None => "",
};

const FIREBASE_PROJECT_ID: &str = match option_env!("FIREBASE_PROJECT_ID") {
    Some(v) => v,
    None => "",
};

#[component]
pub fn SpaceUserLogin() -> Element {
    let mut logging_in = use_signal(|| false);
    let mut error_msg = use_signal(|| Option::<String>::None);

    let mut login_action = use_action(oauth_login);

    rsx! {
        FirebaseAuth {
            api_key: FIREBASE_API_KEY.to_string(),
            auth_domain: FIREBASE_AUTH_DOMAIN.to_string(),
            project_id: FIREBASE_PROJECT_ID.to_string(),
            on_login: move |evt: FirebaseLoginEvent| {
                logging_in.set(true);
                error_msg.set(None);
                spawn(async move {
                    login_action
                        .call(Form(OAuthLoginRequest {
                            access_token: evt.access_token.clone(),
                        }))
                        .await;
                    logging_in.set(false);
                });
            },
            on_error: move |msg: String| {
                error_msg.set(Some(msg));
            },
            button {
                class: "w-full flex justify-end items-center cursor-pointer hover:opacity-80 p-4",
                disabled: *logging_in.read(),
                if *logging_in.read() {
                    "Signing in..."
                } else {
                    "Sign In"
                }
            }
        }
    }
}
