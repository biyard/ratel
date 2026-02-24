use crate::Result as AppResult;
use crate::components::{ProfileImageSection, SettingsForm};
use crate::controllers::{UpdateUserRequest, get_user_detail_handler, update_user_handler};
use crate::*;
#[cfg(not(feature = "server"))]
use common::{wasm_bindgen, wasm_bindgen_futures, web_sys};
use dioxus::prelude::*;
use ratel_auth::hooks::use_user_context;
#[cfg(not(feature = "server"))]
use web_sys::js_sys::{JSON, Reflect};

#[component]
pub fn Home(username: String) -> Element {
    let user_ctx = use_user_context();
    let user = user_ctx.read().user.clone();

    let Some(user) = user else {
        return rsx! {
            div { class: "flex flex-col items-center justify-center w-full h-full py-10",
                p { class: "text-gray-500", "Please log in to access settings." }
            }
        };
    };

    let detail_resource =
        use_server_future(move || async move { get_user_detail_handler().await })?;
    let detail_state = detail_resource.value();

    let mut profile_url = use_signal(|| user.profile_url.clone());
    let mut nickname = use_signal(|| user.display_name.clone());
    let mut description = use_signal(|| user.description.clone());
    let mut evm_address = use_signal(String::new);
    let mut wallet_address = use_signal(|| Option::<String>::None);
    let mut show_wallet_connect = use_signal(|| false);
    let mut saving = use_signal(|| false);
    let mut message = use_signal(|| Option::<String>::None);
    let mut detail_loaded = use_signal(|| false);

    {
        let detail_state = detail_state.clone();
        let mut evm_address = evm_address.clone();
        let mut detail_loaded = detail_loaded.clone();
        use_effect(move || {
            let detail_state = detail_state.read();
            let Some(state) = detail_state.as_ref() else {
                return;
            };

            if let Ok(detail) = state {
                evm_address.set(detail.evm_address.clone().unwrap_or_default());
                detail_loaded.set(true);
            }
        });
    }

    let on_save = {
        let mut nickname = nickname.clone();
        let mut profile_url = profile_url.clone();
        let mut description = description.clone();
        let mut saving = saving.clone();
        let mut message = message.clone();
        move |_evt: MouseEvent| {
            let nick = nickname();
            let profile = profile_url();
            let desc = description();
            if is_blocked_text(&nick) || is_blocked_text(&desc) {
                message.set(Some("Invalid words detected.".to_string()));
                return;
            }
            spawn(async move {
                saving.set(true);
                message.set(None);
                let result = update_user_handler(UpdateUserRequest::Profile {
                    nickname: nick,
                    profile_url: profile,
                    description: desc,
                })
                .await;
                saving.set(false);
                match result {
                    Ok(_) => {
                        message.set(Some("Profile updated successfully.".to_string()));
                    }
                    Err(e) => {
                        message.set(Some(format!("Failed to update profile: {}", e)));
                    }
                }
            });
        }
    };

    let on_toggle_wallet = {
        let mut show_wallet_connect = show_wallet_connect.clone();
        move |_evt: MouseEvent| show_wallet_connect.set(!show_wallet_connect())
    };

    let on_connect_wallet = {
        let mut wallet_address = wallet_address.clone();
        let mut message = message.clone();
        move |_evt: MouseEvent| {
            spawn(async move {
                match connect_wallet_address().await {
                    Ok(Some(address)) => {
                        wallet_address.set(Some(address));
                    }
                    Ok(None) => {
                        message.set(Some("No wallet address found.".to_string()));
                    }
                    Err(err) => {
                        message.set(Some(format!("Wallet error: {}", err)));
                    }
                }
            });
        }
    };

    let on_save_wallet = {
        let mut evm_address = evm_address.clone();
        let mut wallet_address = wallet_address.clone();
        let mut show_wallet_connect = show_wallet_connect.clone();
        let mut message = message.clone();
        move |_evt: MouseEvent| {
            let Some(address) = wallet_address().clone() else {
                return;
            };
            spawn(async move {
                let result = update_user_handler(UpdateUserRequest::EvmAddress {
                    evm_address: address.clone(),
                })
                .await;
                match result {
                    Ok(_) => {
                        evm_address.set(address);
                        show_wallet_connect.set(false);
                        message.set(Some("Wallet address updated.".to_string()));
                    }
                    Err(err) => {
                        message.set(Some(format!("Failed to update wallet: {}", err)));
                    }
                }
            });
        }
    };

    let profile_img = profile_url();
    let evm_value = if evm_address().is_empty() {
        "-".to_string()
    } else {
        evm_address()
    };
    let wallet_visible = *show_wallet_connect.read();
    let wallet_connected = wallet_address().is_some();
    let save_blocked = is_blocked_text(&nickname()) || is_blocked_text(&description());

    let on_profile_pick = {
        let mut profile_url = profile_url.clone();
        move |_evt: MouseEvent| {
            // if let Some(url) = prompt_profile_url() {
            //     profile_url.set(url);
            // }
        }
    };

    rsx! {
        div { class: "flex flex-col gap-10 items-center w-full max-tablet:w-full",
            ProfileImageSection { profile_url: profile_img, on_pick: on_profile_pick }

            SettingsForm {
                username: user.username.clone(),
                evm_value,
                wallet_visible,
                wallet_address: wallet_address(),
                wallet_connected,
                nickname: nickname(),
                description: description(),
                saving: saving(),
                message: message(),
                save_blocked,
                on_toggle_wallet,
                on_connect_wallet,
                on_save_wallet,
                on_nickname_input: move |e: FormEvent| nickname.set(e.value()),
                on_description_input: move |e: FormEvent| description.set(e.value()),
                on_save,
            }
        }
    }
}

#[cfg(not(feature = "server"))]
async fn connect_wallet_address() -> AppResult<Option<String>> {
    let promise =
        crate::interop::connect_wallet().map_err(|e| Error::Unknown(format_js_error(e)))?;
    let value = wasm_bindgen_futures::JsFuture::from(promise)
        .await
        .map_err(|e| Error::Unknown(format_js_error(e)))?;
    Ok(value.as_string())
}

#[cfg(feature = "server")]
async fn connect_wallet_address() -> AppResult<Option<String>> {
    Err(Error::NotSupported(
        "Wallet connection is only available on web".to_string(),
    ))
}

#[cfg(not(feature = "server"))]
fn format_js_error(err: wasm_bindgen::JsValue) -> String {
    if let Some(msg) = err.as_string() {
        msg
    } else {
        if err.is_object() {
            if let Ok(message) = Reflect::get(&err, &wasm_bindgen::JsValue::from_str("message")) {
                if let Some(msg) = message.as_string() {
                    return msg;
                }
            }
        }
        if let Ok(json) = JSON::stringify(&err) {
            if let Some(msg) = json.as_string() {
                return msg;
            }
        }
        "Unknown error".to_string()
    }
}

fn is_blocked_text(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("test") || value.contains("테스트")
}
