use super::*;

#[component]
pub fn JoinAnytimeSetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();
    let UseSpaceGeneralSettings {
        mut update_join_anytime,
        ..
    } = use_space_general_settings(space_id)?;

    let enabled = space().join_anytime;
    let pending = update_join_anytime.pending();

    rsx! {
        section { class: "sga-section", "data-testid": "section-join-anytime",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.join_anytime_setting}" }
            }
            button {
                r#type: "button",
                class: "sga-switch",
                role: "switch",
                tabindex: "0",
                "aria-checked": enabled,
                "aria-disabled": pending,
                "data-testid": "join-anytime-switch",
                onclick: move |_| {
                    if !pending {
                        update_join_anytime.call(!enabled);
                    }
                },
                span { class: "sga-switch__track",
                    span { class: "sga-switch__thumb" }
                }
                span { class: "sga-switch__body",
                    span { class: "sga-switch__label", "{tr.join_anytime_setting}" }
                    span { class: "sga-switch__sub", "{tr.join_anytime_description}" }
                }
            }
        }
    }
}
