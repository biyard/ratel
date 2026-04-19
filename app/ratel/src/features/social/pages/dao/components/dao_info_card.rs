use super::super::TeamDaoTranslate;
use super::super::*;
use dioxus::prelude::*;
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
        div { class: "bg-card-bg dark:bg-gray-800 rounded-lg shadow-md p-6 border border-gray-200 dark:border-gray-700",
            div { class: "flex items-start justify-between mb-4",
                div {
                    h3 { class: "text-xl font-semibold text-text-primary mb-1", {tr.dao_address} }
                    p { class: "text-sm text-text-secondary", {tr.dao_description} }
                }
                div { class: "px-3 py-1 bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200 rounded-full text-sm font-medium",
                    {tr.active}
                }
            }

            div { class: "bg-card-bg-secondary dark:bg-modal-card-bg rounded-md p-4 mb-4",
                div { class: "flex items-center justify-between gap-3",
                    code { class: "text-sm font-mono text-text-primary break-all", {dao_address} }
                    button {
                        class: "shrink-0 p-2 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors",
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
                    href: url,
                    target: "_blank",
                    rel: "noopener noreferrer",
                    class: "inline-flex items-center gap-2 px-4 py-2 bg-primary text-white rounded-md hover:bg-primary-dark transition-colors",
                    {tr.view_on_explorer}
                    icons::ratel::ExternalLinkIcon { width: "16", height: "16", class: "w-4 h-4" }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
async fn copy_to_clipboard(text: &str) -> std::result::Result<(), String> {
    use wasm_bindgen_futures::JsFuture;

    let promise = super::super::interop::copy_text(text).map_err(|e| format!("{:?}", e))?;
    JsFuture::from(promise)
        .await
        .map_err(|e| format!("{:?}", e))?;
    Ok(())
}

#[cfg(not(feature = "web"))]
async fn copy_to_clipboard(_text: &str) -> std::result::Result<(), String> {
    Err("Clipboard is only available on web".to_string())
}

#[cfg(feature = "web")]
async fn after_copy_delay() {
    gloo_timers::future::sleep(Duration::from_millis(2000)).await;
}

#[cfg(not(feature = "web"))]
async fn after_copy_delay() {}
