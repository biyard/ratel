use super::super::*;

#[component]
pub fn DeleteTeamPopup(
    on_confirm: EventHandler<MouseEvent>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: TeamSettingsTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col w-[480px] max-w-full gap-6 p-6",
            div { class: "flex flex-col gap-2",
                div { class: "text-lg font-bold text-text-primary text-center",
                    "{tr.delete_team_title}"
                }
                div { class: "text-sm text-text-secondary leading-6", "{tr.delete_team_description}" }
            }

            div { class: "flex items-center justify-end gap-3",
                button {
                    r#type: "button",
                    class: "h-10 px-4 rounded-lg border border-neutral-300 text-text-primary hover:bg-neutral-100 disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: on_cancel,
                    "{tr.cancel}"
                }
                button {
                    r#type: "button",
                    class: "h-10 px-4 rounded-lg bg-red-600 text-white font-semibold hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: on_confirm,
                    "{tr.confirm}"
                }
            }
        }
    }
}
