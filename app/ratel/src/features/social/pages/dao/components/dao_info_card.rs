use super::super::TeamDaoTranslate;
use super::super::*;
use crate::common::*;
#[cfg(feature = "web")]
use std::time::Duration;

#[component]
pub fn DaoInfoCard(dao_address: String, explorer_url: Option<String>) -> Element {
    let tr: TeamDaoTranslate = use_translate();
    let mut copied = use_signal(|| false);

    let on_copy = {
        let dao_address = dao_address.clone();
        let mut copied = copied.clone();
        move |_| {
            let dao_address = dao_address.clone();
            let mut copied = copied.clone();
            spawn(async move {
                if copy_to_clipboard(&dao_address).await.is_ok() {
                    copied.set(true);
                    after_copy_delay().await;
                    copied.set(false);
                }
            });
        }
    };

    rsx! {
        div { class: "p-6 rounded-lg border border-gray-200 shadow-md dark:bg-gray-800 dark:border-gray-700 bg-card-bg",
            div { class: "flex justify-between items-start mb-4",
                div {
                    h3 { class: "mb-1 text-xl font-semibold text-text-primary", {tr.dao_address} }
                    p { class: "text-sm text-text-secondary", {tr.dao_description} }
                }
                div { class: "py-1 px-3 text-sm font-medium text-green-800 bg-green-100 rounded-full dark:text-green-200 dark:bg-green-900",
                    {tr.active}
                }
            }

            div { class: "p-4 mb-4 rounded-md bg-card-bg-secondary dark:bg-modal-card-bg",
                div { class: "flex gap-3 justify-between items-center",
                    code { class: "font-mono text-sm break-all text-text-primary", {dao_address} }
                    button {
                        class: "p-2 rounded transition-colors hover:bg-gray-200 shrink-0 dark:hover:bg-gray-700",
                        title: {tr.copy_address},
                        onclick: on_copy,
                        if copied() {
                            icons::ratel::CheckIcon {
                                width: "20",
                                height: "20",
                                class: "w-5 h-5 text-green-600",
                            }
                        } else {
                            icons::ratel::ClipboardIcon {
                                width: "20",
                                height: "20",
                                class: "w-5 h-5 text-text-secondary",
                            }
                        }
                    }
                }
            }

            if let Some(url) = explorer_url {
                a {
                    href: "{url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-flex gap-2 items-center py-2 px-4 text-white rounded-md transition-colors bg-primary hover:bg-primary-dark",
                    onclick: move |evt| {
                        // Under tauri-web: intercept and open via the native bridge so the URL
                        // opens in the user's default external browser instead of the WebView.
                        // Under plain web: the anchor's default behaviour is preserved.
                        #[cfg(feature = "tauri-web")]
                        {
                            let url = url.clone();
                            evt.prevent_default();
                            spawn(async move {
                                use crate::tauri::ExternalUrlRequest;
                                use crate::tauri::open;
                                if let Err(e) = open(&ExternalUrlRequest { url }).await {
                                    crate::error!("open_external_url failed: {e}");
                                }
                            });
                        }
                        #[cfg(not(feature = "tauri-web"))]
                        let _ = evt;
                    },
                    {tr.view_on_explorer}
                    icons::ratel::ExternalLinkIcon { width: "16", height: "16", class: "w-4 h-4" }
                }
            }
        }
    }
}

#[cfg(all(feature = "web", not(feature = "server")))]
async fn copy_to_clipboard(text: &str) -> std::result::Result<(), String> {
    use wasm_bindgen_futures::JsFuture;

    let promise = super::super::interop::copy_text(text).map_err(|e| format!("{:?}", e))?;
    JsFuture::from(promise)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[cfg(any(not(feature = "web"), feature = "server"))]
async fn copy_to_clipboard(_text: &str) -> std::result::Result<(), String> {
    Err("Clipboard is only available on web".to_string())
}

#[cfg(feature = "web")]
async fn after_copy_delay() {
    gloo_timers::future::sleep(Duration::from_millis(2000)).await;
}

#[cfg(not(feature = "web"))]
async fn after_copy_delay() {}
