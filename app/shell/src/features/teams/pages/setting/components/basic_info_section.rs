use super::{Input, TextArea};
use super::super::*;

#[component]
pub fn BasicInfoSection(
    username: String,
    nickname: String,
    html_contents: String,
    on_nickname_change: EventHandler<FormEvent>,
    on_description_change: EventHandler<FormEvent>,
    #[props(default)] is_editing: bool,
) -> Element {
    let tr: TeamSettingsTranslate = use_translate();

    rsx! {
        div { class: "flex max-tablet:flex-col gap-2.5",
            label { class: "w-40 font-bold text-text-primary", "{tr.username}" }
            Input {
                value: format!("@{username}"),
                disabled: Some(true),
                class: Some(
                    "w-full text-text-primary bg-input-box-bg border border-input-box-border rounded-md px-3 py-2 disabled:opacity-70"
                        .to_string(),
                ),
            }
        }
        div { class: "flex max-tablet:flex-col gap-2.5",
            label { class: "w-40 font-bold text-text-primary", "{tr.display_name}" }
            Input {
                value: nickname,
                placeholder: Some(tr.display_name_hint.to_string()),
                disabled: Some(!is_editing),
                class: Some(
                    "w-full text-text-primary bg-input-box-bg border border-input-box-border rounded-md px-3 py-2"
                        .to_string(),
                ),
                oninput: Some(on_nickname_change),
            }
        }
        div { class: "flex flex-col gap-2.5",
            label { class: "w-40 font-bold text-text-primary", "{tr.description}" }
            TextArea {
                value: html_contents,
                placeholder: Some(tr.team_description_hint.to_string()),
                disabled: Some(!is_editing),
                class: Some(
                    "w-full text-text-primary bg-input-box-bg border border-input-box-border rounded-md px-3 py-2 min-h-[120px] resize-y"
                        .to_string(),
                ),
                oninput: Some(on_description_change),
            }
        }
    }
}
