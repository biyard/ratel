use crate::*;
use dioxus::prelude::*;

#[component]
pub fn ProfileImageSection(profile_url: String, on_pick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        if profile_url.is_empty() {
            button {
                class: "flex justify-center items-center w-40 h-40 text-sm font-semibold rounded-full bg-c-wg-80 text-text-primary",
                onclick: on_pick,
                "Upload Logo"
            }
        } else {
            img {
                src: "{profile_url}",
                class: "object-cover w-40 h-40 rounded-full cursor-pointer",
                onclick: on_pick,
            }
        }
    }
}

#[component]
pub fn SettingsForm(
    username: String,
    evm_value: String,
    wallet_visible: bool,
    wallet_address: Option<String>,
    wallet_connected: bool,
    nickname: String,
    description: String,
    saving: bool,
    message: Option<String>,
    save_blocked: bool,
    on_toggle_wallet: EventHandler<MouseEvent>,
    on_connect_wallet: EventHandler<MouseEvent>,
    on_save_wallet: EventHandler<MouseEvent>,
    on_nickname_input: EventHandler<FormEvent>,
    on_description_input: EventHandler<FormEvent>,
    on_save: EventHandler<MouseEvent>,
) -> Element {
    let metamask = asset!("/assets/meta-mask-icon.png");
    let wallet_label = if wallet_visible { "Hide" } else { "Change" };
    rsx! {
        div { class: "flex flex-col gap-2.5 w-full",
            div { class: "flex max-tablet:flex-col gap-2.5",
                label { class: "w-40 font-bold text-text-primary", "Username" }
                input {
                    class: "w-full text-text-primary bg-component-bg border border-card-border rounded-md px-3 py-2 disabled:opacity-70",
                    r#type: "text",
                    disabled: true,
                    value: "@{username}",
                }
            }

            div { class: "flex max-tablet:flex-col gap-2.5",
                label { class: "w-40 font-bold text-text-primary", "EVM Address" }
                div { class: "flex gap-2 w-full",
                    input {
                        class: "w-full text-text-primary bg-component-bg border border-card-border rounded-md px-3 py-2 disabled:opacity-70",
                        r#type: "text",
                        disabled: true,
                        value: "{evm_value}",
                    }
                    button {
                        class: "py-0 rounded-sm bg-enable-button-bg text-enable-button-white-text hover:bg-enable-button-bg/80 px-4",
                        onclick: on_toggle_wallet,
                        "{wallet_label}"
                    }
                }
            }

            if wallet_visible {
                div { class: "w-full",
                    div {
                        class: "flex items-center justify-between p-4 bg-card-bg rounded-lg hover:bg-component-bg/70 transition-colors cursor-pointer",
                        onclick: on_connect_wallet,
                        div { class: "flex items-center space-x-3",
                            img {
                                src: "{metamask}",
                                width: "40",
                                height: "40",
                            }
                            div { class: "font-semibold text-text-primary",
                                div { "MetaMask" }
                                div { class: "mt-1 flex items-center space-x-2 text-sm text-gray-400",
                                    if let Some(address) = &wallet_address {
                                        span { "{truncate_address(address)}" }
                                    } else {
                                        span { "Connect wallet" }
                                    }
                                }
                            }
                        }
                        if wallet_connected {
                            button {
                                class: "bg-enable-button-bg text-enable-button-white-text px-4 py-2 rounded-md",
                                onclick: on_save_wallet,
                                "Save"
                            }
                        }
                    }
                }
            }

            div { class: "flex max-tablet:flex-col gap-2.5",
                label { class: "w-40 font-bold text-text-primary", "Display Name" }
                input {
                    class: "w-full text-text-primary bg-component-bg border border-card-border rounded-md px-3 py-2",
                    r#type: "text",
                    placeholder: "Display name",
                    value: "{nickname}",
                    maxlength: 30,
                    oninput: on_nickname_input,
                }
            }

            div { class: "flex flex-col gap-2.5",
                label { class: "w-40 font-bold text-text-primary", "Description" }
                textarea {
                    class: "w-full text-text-primary bg-component-bg border border-card-border rounded-md px-3 py-2 min-h-[120px] resize-y",
                    placeholder: "Tell us about yourself",
                    value: "{description}",
                    oninput: on_description_input,
                }
            }

            div { class: "flex justify-end py-5",
                button {
                    class: if save_blocked { "cursor-not-allowed bg-disable-button-bg text-disable-button-white-text px-6 py-2 rounded-md" } else { "cursor-pointer bg-enable-button-bg text-enable-button-white-text px-6 py-2 rounded-md" },
                    disabled: saving,
                    onclick: on_save,
                    if saving {
                        "Saving..."
                    } else {
                        "Save"
                    }
                }
            }

            if let Some(msg) = message {
                div { class: "text-sm text-[var(--text-secondary)]", "{msg}" }
            }
        }
    }
}

fn truncate_address(address: &str) -> String {
    if address.len() <= 10 {
        return address.to_string();
    }
    format!(
        "{}...{}",
        &address[..6],
        &address[address.len().saturating_sub(4)..]
    )
}
