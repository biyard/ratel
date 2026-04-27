use super::*;

/// Collective attributes section — chip list + "move to conditional"
/// dropdown. Each chip shows the active collective attribute; hovering
/// the '+' button on the section header reveals a menu of options
/// eligible to be promoted into the conditional table.
#[component]
pub fn CollectivePanelSection(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelsTranslate = use_translate();
    let space = use_space();
    let UseSpacePanels {
        panels,
        mut move_to_conditional,
        ..
    } = use_space_panels(space_id)?;

    let panel_list = panels.read().clone();
    let mut show_menu = use_signal(|| false);

    let has_uni = is_collective_option(PanelOption::University, &panel_list);
    let has_age = is_collective_option(PanelOption::Age, &panel_list);
    let has_gen = is_collective_option(PanelOption::Gender, &panel_list);
    let has_any = has_uni || has_age || has_gen;

    let current_quota = space().quota;
    let can_move_age = has_age && current_quota > 0;
    let can_move_gender = has_gen && current_quota > 0;
    let has_movable = can_move_age || can_move_gender;

    rsx! {
        section { class: "spa-section", "data-testid": "section-collective",
            div { class: "spa-section__head",
                div { class: "spa-section__title",
                    span { class: "spa-section__label", "{tr.collective_title}" }
                }
                if has_movable {
                    div { class: "spa-section__actions",
                        div { class: "spa-dropdown",
                            button {
                                r#type: "button",
                                class: "spa-btn spa-btn--icon",
                                "aria-label": "{tr.move_to_conditional_aria}",
                                "data-testid": "collective-add-btn",
                                onclick: move |e: MouseEvent| {
                                    e.stop_propagation();
                                    show_menu.set(!show_menu());
                                },
                                svg {
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    "stroke-width": "2",
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    line {
                                        x1: "12",
                                        y1: "5",
                                        x2: "12",
                                        y2: "19",
                                    }
                                    line {
                                        x1: "5",
                                        y1: "12",
                                        x2: "19",
                                        y2: "12",
                                    }
                                }
                            }
                            div {
                                class: "spa-dropdown-menu",
                                "data-open": show_menu(),
                                "data-testid": "collective-menu",
                                if can_move_age {
                                    button {
                                        r#type: "button",
                                        class: "spa-dropdown-menu__item",
                                        "data-testid": "collective-move-age",
                                        onclick: move |e: MouseEvent| {
                                            e.stop_propagation();
                                            show_menu.set(false);
                                            move_to_conditional.call(PanelOption::Age);
                                        },
                                        "{tr.move_age_to_conditional}"
                                    }
                                }
                                if can_move_gender {
                                    button {
                                        r#type: "button",
                                        class: "spa-dropdown-menu__item",
                                        "data-testid": "collective-move-gender",
                                        onclick: move |e: MouseEvent| {
                                            e.stop_propagation();
                                            show_menu.set(false);
                                            move_to_conditional.call(PanelOption::Gender);
                                        },
                                        "{tr.move_gender_to_conditional}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
            span { class: "spa-section__hint", "{tr.collective_hint}" }

            if has_any {
                div {
                    class: "spa-collective-row",
                    "data-testid": "collective-list",
                    if has_uni {
                        span { class: "spa-collective-chip spa-collective-chip--uni",
                            span { class: "spa-collective-chip__dot" }
                            "{tr.attr_university}"
                        }
                    }
                    if has_age {
                        span { class: "spa-collective-chip spa-collective-chip--age",
                            span { class: "spa-collective-chip__dot" }
                            "{tr.attr_age}"
                            if can_move_age {
                                button {
                                    r#type: "button",
                                    class: "spa-collective-chip__action",
                                    "aria-label": "{tr.move_to_conditional_aria}",
                                    "data-testid": "collective-move-age-chip",
                                    onclick: move |_| move_to_conditional.call(PanelOption::Age),
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "2",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        path { d: "M5 12h14M13 6l6 6-6 6" }
                                    }
                                }
                            }
                        }
                    }
                    if has_gen {
                        span { class: "spa-collective-chip spa-collective-chip--gen",
                            span { class: "spa-collective-chip__dot" }
                            "{tr.attr_gender}"
                            if can_move_gender {
                                button {
                                    r#type: "button",
                                    class: "spa-collective-chip__action",
                                    "aria-label": "{tr.move_to_conditional_aria}",
                                    "data-testid": "collective-move-gender-chip",
                                    onclick: move |_| move_to_conditional.call(PanelOption::Gender),
                                    svg {
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        "stroke-width": "2",
                                        "stroke-linecap": "round",
                                        "stroke-linejoin": "round",
                                        path { d: "M5 12h14M13 6l6 6-6 6" }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                div { class: "spa-empty-hint", "{tr.collective_empty}" }
            }
        }
    }
}
