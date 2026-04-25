use super::*;

#[component]
pub fn AnonymousSetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();
    let UseSpaceGeneralSettings {
        mut update_anonymous,
        ..
    } = use_space_general_settings(space_id)?;

    let enabled = space().anonymous_participation;
    let pending = update_anonymous.pending();

    rsx! {
        section { class: "sga-section", "data-testid": "section-anonymous",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.anonymous_setting}" }
            }
            button {
                r#type: "button",
                class: "sga-switch",
                role: "switch",
                tabindex: "0",
                "aria-checked": enabled,
                "aria-disabled": pending,
                "data-testid": "anonymous-switch",
                onclick: move |_| {
                    if !pending {
                        update_anonymous.call(!enabled);
                    }
                },
                span { class: "sga-switch__track",
                    span { class: "sga-switch__thumb" }
                }
                span { class: "sga-switch__body",
                    span { class: "sga-switch__label", "{tr.anonymous_setting}" }
                    span { class: "sga-switch__sub", "{tr.anonymous_setting_description}" }
                }
            }
        }
    }
}
