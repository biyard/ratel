use super::*;
use std::collections::HashMap;

/// Label axes for a conditional panel row. Determines the swatch color
/// class (`--age`, `--gen`, or `--mix` for composite rows).
#[derive(Clone, Copy, PartialEq, Eq)]
enum RowAxis {
    Uni,
    Age,
    Gender,
    Mix,
}

impl RowAxis {
    fn group_class(self) -> &'static str {
        match self {
            Self::Uni => "spa-p-table__group spa-p-table__group--uni",
            Self::Age => "spa-p-table__group spa-p-table__group--age",
            Self::Gender => "spa-p-table__group spa-p-table__group--gen",
            Self::Mix => "spa-p-table__group spa-p-table__group--mix",
        }
    }
    fn ratio_value_class(self) -> &'static str {
        match self {
            Self::Uni => "spa-ratio-value spa-ratio-value--uni",
            Self::Age => "spa-ratio-value spa-ratio-value--age",
            Self::Gender => "spa-ratio-value spa-ratio-value--gen",
            Self::Mix => "spa-ratio-value spa-ratio-value--mix",
        }
    }
    fn ratio_bar_class(self) -> &'static str {
        match self {
            Self::Uni => "spa-ratio-bar__fill spa-ratio-bar__fill--uni",
            Self::Age => "spa-ratio-bar__fill spa-ratio-bar__fill--age",
            Self::Gender => "spa-ratio-bar__fill spa-ratio-bar__fill--gen",
            Self::Mix => "spa-ratio-bar__fill spa-ratio-bar__fill--mix",
        }
    }
}

fn row_axis(attrs: &[PanelAttribute]) -> RowAxis {
    let mut has_age = false;
    let mut has_gen = false;
    let mut has_uni = false;
    for attr in attrs {
        match attr {
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
            | PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => has_age = true,
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
            | PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => has_gen = true,
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => has_uni = true,
            _ => {}
        }
    }
    match (has_age, has_gen, has_uni) {
        (true, true, _) | (_, _, true) if has_age && has_gen => RowAxis::Mix,
        (true, false, false) => RowAxis::Age,
        (false, true, false) => RowAxis::Gender,
        (false, false, true) => RowAxis::Uni,
        (true, true, _) => RowAxis::Mix,
        _ => RowAxis::Mix,
    }
}

fn group_label(attrs: &[PanelAttribute], tr: &PanelsTranslate) -> String {
    let names: Vec<String> = attrs
        .iter()
        .filter_map(|attr| match attr {
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University)
            | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
                if matches!(attr, PanelAttribute::CollectiveAttribute(_)) =>
            {
                Some(tr.attr_university.to_string())
            }
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
                Some(tr.attr_university.to_string())
            }
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => {
                Some(tr.attr_age.to_string())
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)) => {
                Some(tr.attr_age.to_string())
            }
            PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => {
                Some(tr.attr_gender.to_string())
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)) => {
                Some(tr.attr_gender.to_string())
            }
            PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(_)) => {
                Some(tr.val_generation.to_string())
            }
            _ => None,
        })
        .collect();

    // Dedup while preserving order.
    let mut seen = std::collections::HashSet::new();
    let unique: Vec<String> = names
        .into_iter()
        .filter(|name| seen.insert(name.clone()))
        .collect();

    if unique.is_empty() {
        "-".to_string()
    } else {
        unique.join(", ")
    }
}

/// Per-attribute pill. Returns (css modifier, value text).
fn attribute_pill(attr: &PanelAttribute, tr: &PanelsTranslate) -> Option<(&'static str, String)> {
    match attr {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
            Some(("spa-attr-pill spa-attr-pill--uni", tr.val_verified.to_string()))
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(age)) => {
            let label = match age {
                Age::Specific(n) => format!("{n}"),
                Age::Range {
                    inclusive_min,
                    inclusive_max,
                } => {
                    if *inclusive_min == 70 && *inclusive_max == u8::MAX {
                        "70+".to_string()
                    } else if *inclusive_min == 0 && *inclusive_max == 17 {
                        format!("{} (0-17)", tr.val_minor)
                    } else {
                        format!("{inclusive_min}-{inclusive_max}")
                    }
                }
            };
            Some(("spa-attr-pill spa-attr-pill--age", label))
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(g)) => {
            let label = match g {
                Gender::Male => tr.val_male.to_string(),
                Gender::Female => tr.val_female.to_string(),
            };
            Some(("spa-attr-pill spa-attr-pill--gen", label))
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(is_adult)) => {
            let label = if *is_adult {
                tr.val_adult.to_string()
            } else {
                tr.val_minor.to_string()
            };
            Some(("spa-attr-pill spa-attr-pill--age", label))
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(gen)) => {
            Some(("spa-attr-pill spa-attr-pill--age", format!("{gen:?}")))
        }
        _ => None,
    }
}

#[component]
pub fn ConditionalTableSection(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelsTranslate = use_translate();
    let space = use_space();
    let UseSpacePanels {
        panels,
        mut update_row_quota,
        mut delete_row,
        ..
    } = use_space_panels(space_id)?;

    let panel_list = panels.read().clone();

    // Filter to rows that actually belong in the conditional table
    // (those carrying VerifiableAttribute constraints).
    let visible: Vec<SpacePanelQuotaResponse> = panel_list
        .iter()
        .filter(|panel| {
            panel_attributes(panel).into_iter().any(|attribute| {
                matches!(
                    attribute,
                    PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
                        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
                        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(_))
                        | PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(_))
                )
            })
        })
        .cloned()
        .collect();

    let total_visible: i64 = visible.iter().map(|p| p.quotas.max(0)).sum();
    let space_quota = space().quota;
    let over_allocated = space_quota > 0 && total_visible > space_quota;

    // Local pending edits per row (id → digits string). Writes fire on
    // blur / Enter; the map is cleared after each commit so the next
    // render picks up the server-confirmed value.
    let mut editing: Signal<HashMap<String, String>> = use_signal(HashMap::new);

    rsx! {
        section { class: "spa-section", "data-testid": "section-conditional",
            div { class: "spa-section__head",
                div { class: "spa-section__title",
                    span { class: "spa-section__label", "{tr.conditional_title}" }
                }
                span { class: "spa-section__hint", "{tr.conditional_hint}" }
            }

            if over_allocated {
                div { class: "spa-warn", "data-testid": "ratio-warning",
                    span { class: "spa-warn__icon",
                        svg {
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            "stroke-width": "2",
                            "stroke-linecap": "round",
                            "stroke-linejoin": "round",
                            path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                            line {
                                x1: "12",
                                y1: "9",
                                x2: "12",
                                y2: "13",
                            }
                            line {
                                x1: "12",
                                y1: "17",
                                x2: "12.01",
                                y2: "17",
                            }
                        }
                    }
                    span {
                        strong { "{tr.conditional_over_allocated}" }
                        "{tr.conditional_over_allocated_detail}"
                    }
                }
            }

            div { class: "spa-table-wrap",
                table { class: "spa-p-table", "data-testid": "conditional-table",
                    thead {
                        tr {
                            th { "{tr.th_attribute_group}" }
                            th { "{tr.th_attributes}" }
                            th { "{tr.th_ratio}" }
                            th { class: "spa-num", "{tr.th_max_quotas}" }
                            th { class: "spa-action", "" }
                        }
                    }
                    tbody {
                        if visible.is_empty() {
                            tr {
                                td { class: "spa-table-empty", colspan: "5",
                                    "{tr.conditional_empty}"
                                }
                            }
                        } else {
                            for panel in visible.iter() {
                                {
                                    let attrs = panel_attributes(panel);
                                    let axis = row_axis(&attrs);
                                    let group_text = group_label(&attrs, &tr);
                                    let row_id = panel.panel_id.to_string();
                                    let ratio = if total_visible > 0 {
                                        ((panel.quotas as f64 / total_visible as f64) * 1000.0).round() / 10.0
                                    } else {
                                        0.0
                                    };
                                    let displayed = editing
                                        .read()
                                        .get(&row_id)
                                        .cloned()
                                        .unwrap_or_else(|| panel.quotas.to_string());
                                    let panel_id_for_input = panel.panel_id.clone();
                                    let panel_id_for_input_clear = panel.panel_id.clone();
                                    let panel_id_for_input_key = row_id.clone();
                                    let panel_id_for_delete = panel.panel_id.clone();
                                    let row_id_for_commit = row_id.clone();
                                    let row_id_for_key = row_id.clone();
                                    let panel_quotas_fallback = panel.quotas;
                                    let ratio_style = format!("width:{ratio}%");

                                    rsx! {
                                        tr { key: "{row_id_for_key}",
                                            td {
                                                span { class: "{axis.group_class()}",
                                                    span { class: "spa-p-table__swatch" }
                                                    "{group_text}"
                                                }
                                            }
                                            td {
                                                for attr in attrs.iter() {
                                                    if let Some((cls, label)) = attribute_pill(attr, &tr) {
                                                        span { class: "{cls}", "{label}" }
                                                    }
                                                }
                                            }
                                            td {
                                                span { class: "{axis.ratio_value_class()}", "{ratio}%" }
                                                div { class: "spa-ratio-bar",
                                                    span { class: "{axis.ratio_bar_class()}", style: "{ratio_style}" }
                                                }
                                            }
                                            td { class: "spa-num",
                                                div { class: "spa-quota-cell",
                                                    input {
                                                        class: "spa-num-input",
                                                        r#type: "text",
                                                        value: "{displayed}",
                                                        "data-testid": "conditional-row-input",
                                                        oninput: move |e: FormEvent| {
                                                            let digits = e
                                                                .value()
                                                                .chars()
                                                                .filter(|c| c.is_ascii_digit())
                                                                .collect::<String>();
                                                            editing
                                                                .with_mut(|map| {
                                                                    map.insert(panel_id_for_input_key.clone(), digits);
                                                                });
                                                        },
                                                        onkeydown: {
                                                            let row_id = row_id_for_commit.clone();
                                                            let panel_id = panel_id_for_input.clone();
                                                            move |e: KeyboardEvent| {
                                                                if e.key() == Key::Enter {
                                                                    e.stop_propagation();
                                                                    let next = editing
                                                                        .read()
                                                                        .get(&row_id)
                                                                        .and_then(|v| v.parse::<i64>().ok())
                                                                        .unwrap_or(panel_quotas_fallback);
                                                                    editing
                                                                        .with_mut(|map| {
                                                                            map.remove(&row_id);
                                                                        });
                                                                    update_row_quota.call(panel_id.clone(), next);
                                                                }
                                                            }
                                                        },
                                                        onfocusout: {
                                                            let row_id = row_id.clone();
                                                            let panel_id = panel_id_for_input_clear.clone();
                                                            move |_| {
                                                                let next = editing
                                                                    .read()
                                                                    .get(&row_id)
                                                                    .and_then(|v| v.parse::<i64>().ok())
                                                                    .unwrap_or(panel_quotas_fallback);
                                                                editing
                                                                    .with_mut(|map| {
                                                                        map.remove(&row_id);
                                                                    });
                                                                update_row_quota.call(panel_id.clone(), next);
                                                            }
                                                        },
                                                    }
                                                }
                                            }
                                            td { class: "spa-action",
                                                button {
                                                    r#type: "button",
                                                    class: "spa-row-delete",
                                                    "aria-label": "Delete row",
                                                    "data-testid": "conditional-row-delete",
                                                    onclick: move |_| delete_row.call(panel_id_for_delete.clone()),
                                                    svg {
                                                        view_box: "0 0 24 24",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        "stroke-width": "2",
                                                        "stroke-linecap": "round",
                                                        "stroke-linejoin": "round",
                                                        line {
                                                            x1: "18",
                                                            y1: "6",
                                                            x2: "6",
                                                            y2: "18",
                                                        }
                                                        line {
                                                            x1: "6",
                                                            y1: "6",
                                                            x2: "18",
                                                            y2: "18",
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
