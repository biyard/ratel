use super::*;

/// Total-quota section. Mirrors the mockup's "Total quotas" card:
/// numeric input on the left, an "allocated / unassigned" meter on the
/// right. Auto-saves on blur and Enter (no explicit save button).
#[component]
pub fn TotalQuota(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelsTranslate = use_translate();
    let space = use_space();
    let UseSpacePanels {
        panels,
        mut update_total_quota,
        ..
    } = use_space_panels(space_id)?;

    let current_quota = space().quota;
    let panel_list = panels.read().clone();

    // Keep local input state so the user can type freely; server sync
    // fires on Enter / blur. An effect re-hydrates when the upstream
    // `space().quota` changes (e.g. after a restart).
    let mut input_value = use_signal(|| current_quota.to_string());
    let mut synced_quota = use_signal(|| current_quota);
    use_effect(move || {
        let next = space().quota;
        if synced_quota() != next {
            synced_quota.set(next);
            input_value.set(next.to_string());
        }
    });

    // Allocated = sum of conditional (quota-carrying) rows. The HTML
    // mockup uses it to show `allocated / unassigned`.
    let allocated: i64 = panel_list
        .iter()
        .filter(|panel| {
            panel_attributes(panel).into_iter().any(|attribute| {
                matches!(
                    attribute,
                    PanelAttribute::VerifiableAttribute(_)
                )
            })
        })
        .map(|panel| panel.quotas.max(0))
        .sum();

    let unassigned = (current_quota - allocated).max(0);
    let fill_pct = if current_quota > 0 {
        ((allocated as f64 / current_quota as f64) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let fill_style = format!("width:{fill_pct:.1}%");

    let mut commit = move || {
        let next = input_value().parse::<i64>().unwrap_or_default();
        if next != space().quota {
            update_total_quota.call(next);
        }
    };

    rsx! {
        section { class: "spa-section", "data-testid": "section-total-quotas",
            div { class: "spa-section__head",
                div { class: "spa-section__title",
                    span { class: "spa-section__label", "{tr.total_quotas}" }
                }
                span { class: "spa-section__hint", "{tr.total_quotas_hint}" }
            }
            div { class: "spa-quota-row",
                span { class: "spa-quota-label", "{tr.total_label}" }
                input {
                    class: "spa-quota-input",
                    r#type: "text",
                    value: "{input_value()}",
                    "data-testid": "total-quota-input",
                    oninput: move |e: FormEvent| {
                        let digits = e
                            .value()
                            .chars()
                            .filter(|c| c.is_ascii_digit())
                            .collect::<String>();
                        input_value.set(digits);
                    },
                    onkeydown: move |e: KeyboardEvent| {
                        if e.key() == Key::Enter {
                            e.stop_propagation();
                            commit();
                        }
                    },
                    onfocusout: move |_| commit(),
                }
                div { class: "spa-quota-meter",
                    div { class: "spa-quota-meter__bar",
                        div {
                            class: "spa-quota-meter__fill",
                            style: "{fill_style}",
                        }
                    }
                    div { class: "spa-quota-meter__labels",
                        span {
                            strong { "{allocated}" }
                            " {tr.allocated_label}"
                        }
                        span { "{unassigned} {tr.unassigned_label}" }
                    }
                }
            }
        }
    }
}
