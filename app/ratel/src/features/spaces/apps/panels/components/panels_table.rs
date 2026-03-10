use super::*;
use std::collections::HashMap;

translate! {
    PanelsTableTranslate;

    panels: {
        en: "Panels",
        ko: "패널",
    },
    attribute_group: {
        en: "Attribute group",
        ko: "속성 그룹",
    },
    attributes: {
        en: "Attributes",
        ko: "속성",
    },
    ratio: {
        en: "Ratio",
        ko: "비율",
    },
    total_quotas: {
        en: "Total quotas",
        ko: "총 쿼터",
    },
    no_attributes: {
        en: "No attributes",
        ko: "속성이 없습니다",
    },
    university: {
        en: "University",
        ko: "대학교",
    },
    gender: {
        en: "Gender",
        ko: "성별",
    },
    age: {
        en: "Age",
        ko: "나이",
    },
    generation: {
        en: "Generation",
        ko: "세대",
    },
    adult: {
        en: "Adult",
        ko: "성인",
    },
    verified: {
        en: "Verified",
        ko: "인증됨",
    },
    male: {
        en: "Male",
        ko: "남성",
    },
    female: {
        en: "Female",
        ko: "여성",
    },
    minor: {
        en: "Minor",
        ko: "미성년자",
    },
}

fn panel_group_label(panel: &SpacePanelQuotaResponse, tr: &PanelsTableTranslate) -> String {
    match &panel.attributes {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
            tr.university.to_string()
        }
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => tr.age.to_string(),
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => tr.gender.to_string(),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)) => {
            tr.gender.to_string()
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_)) => tr.age.to_string(),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(_)) => {
            tr.generation.to_string()
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(_)) => {
            tr.adult.to_string()
        }
        _ => "-".to_string(),
    }
}

fn panel_value_label(panel: &SpacePanelQuotaResponse, tr: &PanelsTableTranslate) -> String {
    match &panel.attributes {
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::University) => {
            tr.verified.to_string()
        }
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Age) => tr.verified.to_string(),
        PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender) => tr.verified.to_string(),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Male)) => {
            tr.male.to_string()
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(Gender::Female)) => {
            tr.female.to_string()
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Specific(age),
        )) => format!("{age}"),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(
            crate::common::attribute::Age::Range {
                inclusive_min,
                inclusive_max,
            },
        )) => format!("{inclusive_min}-{inclusive_max}"),
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(generation)) => {
            format!("{generation:?}")
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(true)) => {
            tr.adult.to_string()
        }
        PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(false)) => {
            tr.minor.to_string()
        }
        _ => "-".to_string(),
    }
}

#[component]
pub fn PanelsTable(
    space_id: SpacePartition,
    panels: Vec<SpacePanelQuotaResponse>,
    panels_query_key: Vec<String>,
) -> Element {
    let tr: PanelsTableTranslate = use_translate();
    let mut editing_quotas = use_signal(HashMap::<String, String>::new);

    let visible_panels = panels
        .into_iter()
        .filter(|panel| {
            matches!(
                panel.attributes,
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
                    | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
                    | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(_))
                    | PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(_))
            )
        })
        .collect::<Vec<_>>();
    let total_visible_quota = visible_panels
        .iter()
        .map(|panel| panel.quotas.max(0))
        .sum::<i64>();

    rsx! {
        SpaceCard { class: "!p-6".to_string(),
            div { class: "flex flex-col gap-4 min-w-0",
                h2 { class: "text-lg font-semibold text-panel-title", {tr.panels} }

                div { class: "overflow-x-auto min-w-0",
                    table { class: "w-full overflow-hidden rounded-xl border border-input-box-border text-sm",
                        thead { class: "bg-panel-container-bg text-panel-table-header",
                            tr {
                                th { class: "px-4 py-3 text-left font-medium", {tr.attribute_group} }
                                th { class: "px-4 py-3 text-left font-medium", {tr.attributes} }
                                th { class: "px-4 py-3 text-right font-medium", {tr.ratio} }
                                th { class: "px-4 py-3 text-center font-medium", {tr.total_quotas} }
                                th { class: "px-4 py-3 text-right font-medium", "" }
                            }
                        }
                        tbody {
                            if visible_panels.is_empty() {
                                tr {
                                    td {
                                        class: "px-4 py-8 text-center text-text-secondary",
                                        colspan: "5",
                                        {tr.no_attributes}
                                    }
                                }
                            } else {
                                for panel in visible_panels.iter() {
                                    {
                                        let panel = panel.clone();
                                        let input_key = panel.panel_id.to_string();
                                        let input_key_for_input = input_key.clone();
                                        let input_key_for_confirm = input_key.clone();
                                        let input_key_for_blur = input_key.clone();
                                        let space_id_for_confirm = space_id.clone();
                                        let space_id_for_blur = space_id.clone();
                                        let space_id_for_delete = space_id.clone();
                                        let panels_query_key_for_confirm = panels_query_key.clone();
                                        let panels_query_key_for_blur = panels_query_key.clone();
                                        let panels_query_key_for_delete = panels_query_key.clone();
                                        let panel_for_confirm = panel.clone();
                                        let panel_for_blur = panel.clone();
                                        let panel_for_delete = panel.clone();
                                        let displayed_value = editing_quotas
                                            .read()
                                            .get(&input_key)
                                            .cloned()
                                            .unwrap_or_else(|| panel.quotas.to_string());
                                        let ratio = if total_visible_quota > 0 {
                                            ((panel.quotas as f64 / total_visible_quota as f64)
                                                * 1000.0)
                                                .round()
                                                / 10.0
                                        } else {
                                            0.0
                                        };

                                        rsx! {
                                            tr { key: "{panel.panel_id}", class: "border-t border-input-box-border",
                                                td { class: "px-4 py-3 text-left font-medium text-text-primary",
                                                    {panel_group_label(&panel, &tr)}
                                                }
                                                td { class: "px-4 py-3 text-left text-text-primary", {panel_value_label(&panel, &tr)} }
                                                td { class: "px-4 py-3 text-right text-text-secondary", "{ratio}%" }
                                                td { class: "px-4 py-3 text-center",
                                                    Input {
                                                        class: "w-20 h-9 !px-3 text-center text-sm font-semibold".to_string(),
                                                        value: displayed_value,
                                                        oninput: move |evt: Event<FormData>| {
                                                            let digits = evt
                                                                .value()
                                                                .chars()
                                                                .filter(|ch| ch.is_ascii_digit())
                                                                .collect::<String>();
                                                            editing_quotas
                                                                .with_mut(|map| {
                                                                    map.insert(input_key_for_input.clone(), digits);
                                                                });
                                                        },
                                                        onconfirm: move |_| {
                                                            let next = editing_quotas
                                                                .read()
                                                                .get(&input_key_for_confirm)
                                                                .and_then(|value| value.parse::<i64>().ok())
                                                                .unwrap_or(panel_for_confirm.quotas);
                                                            editing_quotas
                                                                .with_mut(|map| {
                                                                    map.remove(&input_key_for_confirm);
                                                                });
                                                            let space_id = space_id_for_confirm.clone();
                                                            let panel_id = panel_for_confirm.panel_id.clone();
                                                            let panels_query_key = panels_query_key_for_confirm.clone();
                                                            spawn(async move {
                                                                match update_panel_quota(
                                                                        space_id,
                                                                        UpdatePanelQuotaRequest {
                                                                            panel_id,
                                                                            quota: next,
                                                                        },
                                                                    )
                                                                    .await
                                                                {
                                                                    Ok(_) => invalidate_query(&panels_query_key),
                                                                    Err(err) => error!("Failed to update panel row quota: {:?}", err),
                                                                }
                                                            });
                                                        },
                                                        onblur: move |_| {
                                                            let next = editing_quotas
                                                                .read()
                                                                .get(&input_key_for_blur)
                                                                .and_then(|value| value.parse::<i64>().ok())
                                                                .unwrap_or(panel_for_blur.quotas);
                                                            editing_quotas
                                                                .with_mut(|map| {
                                                                    map.remove(&input_key_for_blur);
                                                                });
                                                            let space_id = space_id_for_blur.clone();
                                                            let panel_id = panel_for_blur.panel_id.clone();
                                                            let panels_query_key = panels_query_key_for_blur.clone();
                                                            spawn(async move {
                                                                match update_panel_quota(
                                                                        space_id,
                                                                        UpdatePanelQuotaRequest {
                                                                            panel_id,
                                                                            quota: next,
                                                                        },
                                                                    )
                                                                    .await
                                                                {
                                                                    Ok(_) => invalidate_query(&panels_query_key),
                                                                    Err(err) => error!("Failed to update panel row quota: {:?}", err),
                                                                }
                                                            });
                                                        },
                                                    }
                                                }
                                                td { class: "px-4 py-3 text-right",
                                                    Button {
                                                        size: ButtonSize::Icon,
                                                        style: ButtonStyle::Text,
                                                        class: "flex items-center justify-center size-8 !p-0 rounded-full !text-text-secondary hover:!bg-hover hover:!text-text-primary"
                                                            .to_string(),
                                                        onclick: move |_| {
                                                            let space_id = space_id_for_delete.clone();
                                                            let panels_query_key = panels_query_key_for_delete.clone();
                                                            let keys = vec![
                                                                DeletePanelKey {
                                                                    panel_id: panel_for_delete.panel_id.clone(),
                                                                },
                                                            ];
                                                            spawn(async move {
                                                                match delete_panel_quotas(space_id, DeletePanelQuotaRequest { keys }).await {
                                                                    Ok(_) => invalidate_query(&panels_query_key),
                                                                    Err(err) => error!("Failed to delete panel row: {:?}", err),
                                                                }
                                                            });
                                                        },
                                                        icons::ratel::XMarkIcon { width: "16", height: "16", class: "h-4 w-4" }
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
