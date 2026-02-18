use dioxus::prelude::*;
use web_sys::wasm_bindgen::JsCast;

pub const FIREBASE_AUTH_JS: Asset = asset!("/assets/firebase-auth.js", AssetOptions::js());

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, PartialEq)]
pub struct FirebaseLoginEvent {
    pub access_token: String,
    pub id_token: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
enum FirebaseAuthResult {
    Ok { ok: FirebaseLoginEvent },
    Err { err: String },
}

#[derive(Debug, Props, Clone, PartialEq)]
pub struct FirebaseAuthProps {
    pub api_key: String,
    pub auth_domain: String,
    pub project_id: String,
    #[props(default)]
    pub on_login: Option<EventHandler<FirebaseLoginEvent>>,
    #[props(default)]
    pub on_error: Option<EventHandler<String>>,
    #[props(default)]
    pub class: String,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn FirebaseAuth(props: FirebaseAuthProps) -> Element {
    rsx! {
        Fragment {
            document::Script { src: FIREBASE_AUTH_JS }
            firebase-auth {
                class: "{props.class}",
                "api-key": "{props.api_key}",
                "auth-domain": "{props.auth_domain}",
                "project-id": "{props.project_id}",
                onchange: move |evt| {
                    if let Some(raw_event) = evt.data().downcast::<web_sys::Event>() {
                        if let Some(custom_event) = raw_event.dyn_ref::<web_sys::CustomEvent>() {
                            if let Some(val) = custom_event.detail().as_string() {
                                match serde_json::from_str::<FirebaseAuthResult>(&val) {
                                    Ok(FirebaseAuthResult::Ok { ok: login_event }) => {
                                        if let Some(handler) = &props.on_login {
                                            handler.call(login_event);
                                        }
                                    }
                                    Ok(FirebaseAuthResult::Err { err: message }) => {
                                        if let Some(handler) = &props.on_error {
                                            handler.call(message);
                                        }
                                    }
                                    Err(_) => {}
                                }
                            }
                        }
                    }
                },
                {props.children}
            }
        }
    }
}
