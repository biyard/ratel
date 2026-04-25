use super::*;

/// Attribute group toggles — University / Age / Gender.
///
/// Each card is a pressable toggle: `aria-selected="true"` switches the
/// card into the "on" visual state (matched by `.spa-attr-toggle` CSS).
/// Flipping a card fires `toggle_attribute` which rebuilds the panel
/// rows server-side.
#[component]
pub fn AttributeGroupsSection(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelsTranslate = use_translate();
    let UseSpacePanels {
        panels,
        mut toggle_attribute,
        ..
    } = use_space_panels(space_id)?;

    let panel_list = panels.read().clone();
    let has_university = is_selected_option(PanelOption::University, &panel_list);
    let has_age = is_selected_option(PanelOption::Age, &panel_list);
    let has_gender = is_selected_option(PanelOption::Gender, &panel_list);

    rsx! {
        section { class: "spa-section", "data-testid": "section-attribute-groups",
            div { class: "spa-section__head",
                div { class: "spa-section__title",
                    span { class: "spa-section__label", "{tr.attribute_groups}" }
                }
                span { class: "spa-section__hint", "{tr.attribute_groups_hint}" }
            }
            div { class: "spa-attr-toggles", "data-testid": "attr-toggles",
                AttributeToggleCard {
                    modifier: "spa-attr-toggle--uni",
                    name: tr.attr_university.to_string(),
                    desc: tr.attr_university_desc.to_string(),
                    selected: has_university,
                    testid: "attr-university",
                    onclick: move |_| toggle_attribute.call(PanelOption::University),
                }
                AttributeToggleCard {
                    modifier: "spa-attr-toggle--age",
                    name: tr.attr_age.to_string(),
                    desc: tr.attr_age_desc.to_string(),
                    selected: has_age,
                    testid: "attr-age",
                    onclick: move |_| toggle_attribute.call(PanelOption::Age),
                }
                AttributeToggleCard {
                    modifier: "spa-attr-toggle--gen",
                    name: tr.attr_gender.to_string(),
                    desc: tr.attr_gender_desc.to_string(),
                    selected: has_gender,
                    testid: "attr-gender",
                    onclick: move |_| toggle_attribute.call(PanelOption::Gender),
                }
            }
        }
    }
}

#[component]
fn AttributeToggleCard(
    modifier: &'static str,
    name: String,
    desc: String,
    selected: bool,
    testid: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            r#type: "button",
            class: "spa-attr-toggle {modifier}",
            "aria-selected": selected,
            "data-testid": "{testid}",
            onclick: move |e| onclick.call(e),
            div { class: "spa-attr-toggle__head",
                span { class: "spa-attr-toggle__name",
                    span { class: "spa-attr-toggle__swatch" }
                    "{name}"
                }
                span { class: "spa-attr-toggle__check",
                    svg {
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "3",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        polyline { points: "20 6 9 17 4 12" }
                    }
                }
            }
            div { class: "spa-attr-toggle__desc", "{desc}" }
        }
    }
}
