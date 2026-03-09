use super::super::dto::EligibleAdminResponse;
use super::super::TeamDaoTranslate;
use super::super::*;
use dioxus::prelude::*;
use std::collections::HashSet;

#[component]
pub fn RegisterDaoPopup(
    eligible_admins: Vec<EligibleAdminResponse>,
    on_register: EventHandler<Vec<String>>,
    on_cancel: EventHandler<MouseEvent>,
    is_registering: bool,
) -> Element {
    let tr: TeamDaoTranslate = use_translate();
    let mut selected_addresses = use_signal(|| HashSet::<String>::new());

    let selected_count = selected_addresses.read().len();
    let min_required = 3;
    let can_confirm = selected_count >= min_required && !is_registering;

    let on_confirm = {
        let selected_addresses = selected_addresses.clone();
        let on_register = on_register.clone();
        move |_| {
            let list = selected_addresses
                .read()
                .iter()
                .cloned()
                .collect::<Vec<_>>();
            on_register.call(list);
        }
    };

    let truncate_address = |address: &str| {
        if address.len() <= 10 {
            address.to_string()
        } else {
            format!("{}...{}", &address[..6], &address[address.len() - 4..])
        }
    };

    let admin_rows = eligible_admins
        .into_iter()
        .map(|admin| {
            let selected = selected_addresses.read().contains(&admin.evm_address);
            let evm_address = admin.evm_address.clone();
            let address_label = truncate_address(&evm_address);
            let user_id = admin.user_id.clone();
            let display_name = admin.display_name.clone();
            let username = admin.username.clone();
            let is_owner = admin.is_owner;
            let profile_url = admin
                .profile_url
                .clone()
                .filter(|url| !url.is_empty())
                .unwrap_or_else(|| "/default-avatar.png".to_string());
            let base_class = if selected {
                "flex items-center gap-4 p-4 rounded-lg border-2 cursor-pointer transition-all border-primary bg-primary/10"
            } else {
                "flex items-center gap-4 p-4 rounded-lg border-2 cursor-pointer transition-all border-neutral-200/70 dark:border-neutral-700/60 hover:border-neutral-400/50 dark:hover:border-neutral-500/50"
            };
            let row_class = if is_registering {
                format!("{} opacity-50 cursor-not-allowed", base_class)
            } else {
                base_class.to_string()
            };
            let mut selected_addresses = selected_addresses.clone();

            rsx! {
                div {
                    key: "{user_id}",
                    class: row_class,
                    onclick: move |_| {
                        if !is_registering {
                            let mut next = selected_addresses();
                            if next.contains(&evm_address) {
                                next.remove(&evm_address);
                            } else {
                                next.insert(evm_address.clone());
                            }
                            selected_addresses.set(next);
                        }
                    },
                    div { class: if selected { "w-5 h-5 rounded border-2 flex items-center justify-center shrink-0 bg-primary border-primary" } else { "w-5 h-5 rounded border-2 flex items-center justify-center shrink-0 border-neutral-300/70 dark:border-neutral-600/70" },
                        if selected {
                            icons::ratel::CheckIcon {
                                width: "16",
                                height: "16",
                                class: "w-4 h-4 text-white",
                            }
                        }
                    }
                    img {
                        src: profile_url,
                        alt: display_name,
                        class: "w-12 h-12 rounded-full object-cover",
                    }
                    div { class: "flex-1 min-w-0",
                        div { class: "flex items-center gap-2 mb-1",
                            p { class: "font-semibold text-text-primary dark:text-neutral-100 truncate",
                                "{display_name}"
                            }
                            if is_owner {
                                span { class: "px-2 py-0.5 bg-yellow-100/80 dark:bg-yellow-900/60 text-yellow-800 dark:text-yellow-200 text-xs font-medium rounded",
                                    "Owner"
                                }
                            }
                        }
                        p { class: "text-sm text-neutral-500 dark:text-neutral-400 truncate",
                            {format!("@{}", username)}
                        }
                        p { class: "text-xs font-mono text-neutral-400 dark:text-neutral-500 mt-1",
                            {address_label}
                        }
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    rsx! {
        div { class: "fixed inset-0 z-50 flex items-center justify-center bg-black/60",
            div { class: "bg-background text-neutral-900 dark:text-neutral-100 rounded-lg shadow-xl w-full max-w-[600px] max-h-[80vh] flex flex-col border border-neutral-200/70 dark:border-neutral-700/60",
                div { class: "flex items-center justify-between p-6 border-b border-neutral-200/70 dark:border-neutral-700/60",
                    div {
                        h2 { class: "text-2xl font-bold text-text-primary", {tr.select_admins} }
                        p { class: "text-sm text-text-secondary mt-1",
                            {tr.select_admins_description}
                        }
                    }
                    button {
                        class: "p-2 hover:bg-neutral-100 dark:hover:bg-neutral-800 rounded-full transition-colors disabled:opacity-50",
                        onclick: on_cancel,
                        disabled: is_registering,
                        icons::ratel::XMarkIcon {
                            width: "24",
                            height: "24",
                            class: "w-6 h-6 text-neutral-500",
                        }
                    }
                }

                div { class: "flex-1 overflow-y-auto p-6",
                    div { class: "space-y-3",
                        for row in admin_rows {
                            {row}
                        }
                    }
                }

                div { class: "p-6 border-t border-neutral-200/70 dark:border-neutral-700/60",
                    div { class: "flex items-center justify-between mb-4",
                        p { class: "text-sm text-neutral-500 dark:text-neutral-400",
                            {tr.selected_count.replace("{{count}}", &selected_count.to_string())}
                        }
                        if selected_count < min_required {
                            p { class: "text-sm text-red-500", {tr.min_admins_required} }
                        }
                    }

                    div { class: "flex gap-3",
                        button {
                            class: "flex-1 px-6 py-3 bg-neutral-300 text-neutral-900 dark:bg-neutral-800 dark:text-neutral-100 rounded-md font-medium hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                            onclick: on_cancel,
                            disabled: is_registering,
                            {tr.cancel}
                        }
                        button {
                            class: "flex-1 px-6 py-3 bg-primary hover:bg-primary/80 text-white rounded-md font-medium hover:bg-primary-dark transition-colors disabled:opacity-50 disabled:cursor-not-allowed",
                            onclick: on_confirm,
                            disabled: !can_confirm,
                            if is_registering {
                                {tr.registering_dao}
                            } else {
                                {tr.confirm}
                            }
                        }
                    }
                }
            }
        }
    }
}
