use super::super::*;

#[component]
pub fn SpecBox(
    left_text: String,
    action_text: String,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex items-center justify-between border border-setting-card-border px-4 py-8 rounded-md",
            p { class: "text-base font-bold text-text-primary", "{left_text}" }
            button {
                class: "flex items-center gap-2 text-primary cursor-pointer",
                onclick: on_click,
                span { "{action_text}" }
                crate::common::lucide_dioxus::ChevronRight { size: 16 }
            }
        }
    }
}

#[component]
pub fn MySettingsTab(
    language_label: String,
    theme_label: String,
    on_language_click: EventHandler<MouseEvent>,
    on_theme_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "w-full max-w-[800px] mx-auto flex flex-col gap-6 px-4 md:px-0",
            section { class: "bg-card-bg border border-card-border p-4 md:p-6 rounded-lg",
                h2 { class: "text-lg font-bold mb-4 text-text-primary", "Appearance" }
                div { class: "flex flex-col gap-4",
                    SpecBox {
                        left_text: "Language".to_string(),
                        action_text: language_label,
                        on_click: on_language_click,
                    }
                    SpecBox {
                        left_text: "Theme".to_string(),
                        action_text: theme_label,
                        on_click: on_theme_click,
                    }
                }
            }
        }
    }
}
