use super::super::*;

#[component]
pub fn SpecBox(
    left_text: String,
    action_text: String,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex justify-between items-center py-8 px-4 rounded-md border border-setting-card-border",
            p { class: "text-base font-bold text-text-primary", "{left_text}" }
            button {
                class: "flex gap-2 items-center cursor-pointer text-primary",
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
        div { class: "flex flex-col gap-6 px-4 mx-auto w-full md:px-0 max-w-[800px]",
            section { class: "p-4 rounded-lg border md:p-6 bg-card-bg border-card-border",
                h2 { class: "mb-4 text-lg font-bold text-text-primary", "Appearance" }
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
