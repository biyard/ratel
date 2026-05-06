use super::super::*;

#[component]
pub fn DeleteTeamPopup(
    on_confirm: EventHandler<MouseEvent>,
    on_cancel: EventHandler<MouseEvent>,
) -> Element {
    let tr: TeamSettingsTranslate = use_translate();

    rsx! {
        div { class: "flex flex-col gap-6 p-6 max-w-full w-[480px]",
            div { class: "flex flex-col gap-2",
                div { class: "text-lg font-bold text-center text-text-primary",
                    "{tr.delete_team_title}"
                }
                div { class: "text-sm text-text-secondary leading-6", "{tr.delete_team_description}" }
            }

            div { class: "flex gap-3 justify-end items-center",
                button {
                    r#type: "button",
                    class: "px-4 h-10 rounded-lg border disabled:opacity-50 disabled:cursor-not-allowed border-neutral-300 text-text-primary hover:bg-neutral-100",
                    onclick: on_cancel,
                    "{tr.cancel}"
                }
                button {
                    r#type: "button",
                    class: "px-4 h-10 font-semibold text-white bg-red-600 rounded-lg hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed",
                    onclick: on_confirm,
                    "{tr.confirm}"
                }
            }
        }
    }
}
