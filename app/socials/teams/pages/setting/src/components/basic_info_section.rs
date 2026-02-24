use crate::*;

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
            input {
                class: "w-full text-text-primary bg-input-box-bg border border-input-box-border rounded-md px-3 py-2 disabled:opacity-70",
                r#type: "text",
                disabled: true,
                value: "@{username}",
            }
        }
        div { class: "flex max-tablet:flex-col gap-2.5",
            label { class: "w-40 font-bold text-text-primary", "{tr.display_name}" }
            input {
                class: "w-full text-text-primary bg-input-box-bg border border-input-box-border rounded-md px-3 py-2",
                r#type: "text",
                placeholder: "{tr.display_name_hint}",
                value: "{nickname}",
                disabled: !is_editing,
                oninput: on_nickname_change,
            }
        }
        div { class: "flex flex-col gap-2.5",
            label { class: "w-40 font-bold text-text-primary", "{tr.description}" }
            textarea {
                class: "w-full text-text-primary bg-input-box-bg border border-input-box-border rounded-md px-3 py-2 min-h-[120px] resize-y",
                placeholder: "{tr.team_description_hint}",
                value: "{html_contents}",
                disabled: !is_editing,
                oninput: on_description_change,
            }
        }
    }
}
