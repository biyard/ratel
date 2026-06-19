//! Android push wiring (tauri-web only): registers the native-injected FCM
//! token with the backend once the user is logged in, and routes
//! notification-tap deep links into the app router. No-op on web/server.

#[cfg(feature = "tauri-web")]
pub fn use_push() {
    use crate::common::*;
    use crate::features::auth::hooks::use_user_context;
    use crate::features::notifications::controllers::{
        register_device_handler, RegisterDeviceRequest,
    };
    use dioxus::document::eval as dx_eval;

    #[derive(serde::Deserialize, Clone, PartialEq)]
    struct FcmInfo {
        token: String,
        #[serde(rename = "deviceId")]
        device_id: String,
        platform: String,
    }

    let user_ctx = use_user_context();
    let nav = use_navigator();
    let mut fcm = use_signal(|| Option::<FcmInfo>::None);

    // Read the token MainActivity injected (or wait for `ratel-fcm-ready`).
    use_effect(move || {
        spawn(async move {
            let mut runner = dx_eval(include_str!("web/push_register.js"));
            if let Ok(info) = runner.recv::<FcmInfo>().await {
                fcm.set(Some(info));
            }
        });
    });

    // Register whenever we have a token AND are logged in. Reads `is_logged_in`
    // reactively, so it re-runs (and registers) right after a login. The
    // backend upsert is idempotent, so a duplicate call is harmless.
    use_effect(move || {
        let logged_in = user_ctx().is_logged_in();
        if let Some(info) = fcm() {
            if logged_in {
                spawn(async move {
                    if let Err(e) = register_device_handler(RegisterDeviceRequest {
                        device_id: info.device_id,
                        token: info.token,
                        platform: info.platform,
                    })
                    .await
                    {
                        tracing::error!("push: register_device failed: {e}");
                    }
                });
            }
        }
    });

    // Deep-link: navigate on each tapped-notification url.
    use_effect(move || {
        spawn(async move {
            let mut runner = dx_eval(include_str!("web/push_deeplink.js"));
            loop {
                match runner.recv::<String>().await {
                    Ok(url) if !url.is_empty() => {
                        nav.push(url);
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
        });
    });
}

#[cfg(not(feature = "tauri-web"))]
pub fn use_push() {}
