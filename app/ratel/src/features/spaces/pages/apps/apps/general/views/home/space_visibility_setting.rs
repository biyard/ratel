use super::*;

#[component]
pub fn SpaceVisibilitySetting(space_id: ReadSignal<SpacePartition>) -> Element {
    let space = use_space();
    let tr: GeneralTranslate = use_translate();
    let UseSpaceGeneralSettings {
        mut update_visibility,
        ..
    } = use_space_general_settings(space_id)?;

    let is_public = space().visibility == SpaceVisibility::Public;
    let pending = update_visibility.pending();

    rsx! {
        section { class: "sga-section", "data-testid": "section-visibility",
            div { class: "sga-section__head",
                span { class: "sga-section__label", "{tr.space_visibility}" }
                span { class: "sga-section__hint", "{tr.space_visibility_description}" }
            }
            div { class: "sga-radio-cards",
                button {
                    r#type: "button",
                    class: "sga-radio-card",
                    "aria-selected": is_public,
                    "data-testid": "visibility-public",
                    disabled: pending || is_public,
                    onclick: move |_| update_visibility.call(SpaceVisibility::Public),
                    span { class: "sga-radio-card__title",
                        icons::internet_script::Internet { class: "[&>path]:stroke-current [&>path]:fill-none [&>circle]:stroke-current [&>circle]:fill-none [&>ellipse]:stroke-current [&>ellipse]:fill-none" }
                        "{tr.public_space}"
                    }
                    span { class: "sga-radio-card__desc", "{tr.public_space_desc}" }
                }
                button {
                    r#type: "button",
                    class: "sga-radio-card",
                    "aria-selected": !is_public,
                    "data-testid": "visibility-private",
                    disabled: pending || !is_public,
                    onclick: move |_| update_visibility.call(SpaceVisibility::Private),
                    span { class: "sga-radio-card__title",
                        icons::security::Lock1 { class: "[&>path]:stroke-current [&>path]:fill-none" }
                        "{tr.private_space}"
                    }
                    span { class: "sga-radio-card__desc", "{tr.private_space_desc}" }
                }
            }
        }
    }
}
