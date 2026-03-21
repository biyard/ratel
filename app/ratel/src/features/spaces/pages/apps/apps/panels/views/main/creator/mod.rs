use super::*;

const PANELS_QUERY_KEY: &str = "Panels";

translate! {
    PanelPageTranslate;

    unsaved_changes: {
        en: "Unsaved changes",
        ko: "저장되지 않은 변경사항",
    },
    changes_saved: {
        en: "Changes saved",
        ko: "변경사항 저장됨",
    },
}

fn panels_key(space_id: &SpacePartition) -> Vec<String> {
    vec![
        "Space".to_string(),
        space_id.to_string(),
        PANELS_QUERY_KEY.to_string(),
    ]
}

#[derive(Clone, Copy, PartialEq)]
enum SaveStatus {
    Idle,
    Saving,
    Saved,
}

#[component]
pub fn PanelPage(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: PanelPageTranslate = use_translate();
    let panels_query_key = panels_key(&space_id());
    let panels_loader = use_query(&panels_query_key, { move || list_panels(space_id()) })?;

    let panels: Vec<SpacePanelQuotaResponse> = panels_loader.read().clone();
    let space = use_space();
    let current_quota = space().quota;

    let has_collective = is_collective_option(PanelOption::University, &panels)
        || is_collective_option(PanelOption::Age, &panels)
        || is_collective_option(PanelOption::Gender, &panels);

    let has_conditional = panels.iter().any(|panel| {
        panel_attributes(panel).into_iter().any(|attr| {
            matches!(
                attr,
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_))
                    | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Age(_))
                    | PanelAttribute::VerifiableAttribute(VerifiableAttribute::Generation(_))
                    | PanelAttribute::VerifiableAttribute(VerifiableAttribute::IsAdult(_))
            )
        })
    });

    let save_status = use_memo(move || {
        if panels_loader.loading() {
            SaveStatus::Saving
        } else {
            SaveStatus::Saved
        }
    });

    rsx! {
        div { class: "flex flex-col w-full min-h-full",
            div { class: "flex w-full flex-col gap-5 flex-1",
                div { class: "flex flex-col md:flex-row md:items-center gap-5 min-w-0",
                    TotalQuotas { space_id, quota: current_quota }
                    AttributeGroups {
                        space_id,
                        panels: panels.clone(),
                        current_quota,
                        panels_query_key: panels_query_key.clone(),
                    }
                }

                if has_collective {
                    CollectivePanel {
                        space_id,
                        panels: panels.clone(),
                        current_quota,
                        panels_query_key: panels_query_key.clone(),
                    }
                }

                if has_conditional {
                    PanelsTable { space_id, panels, panels_query_key }
                }
            }

            div { class: "sticky -bottom-5 -mx-5 -mb-5 flex flex-row justify-end items-center h-20 py-5 px-4 bg-card-bg max-tablet:-bottom-3 max-tablet:-mx-3 max-tablet:-mb-3 max-mobile:-bottom-2 max-mobile:-mx-2 max-mobile:-mb-2",
                match save_status() {
                    SaveStatus::Saving => rsx! {
                        span { class: "text-sm text-text-secondary animate-pulse", {tr.unsaved_changes} }
                    },
                    SaveStatus::Saved => rsx! {
                        div { class: "flex items-center gap-1 text-sm text-green-500",
                            icons::validations::Check { class: "w-[13px] h-[13px] min-w-[13px] [&>path]:stroke-green-500" }
                            {tr.changes_saved}
                        }
                    },
                    SaveStatus::Idle => rsx! {},
                }
            }
        }
    }
}

#[component]
pub fn CreatorPage(space_id: ReadSignal<SpacePartition>) -> Element {
    rsx! {
        PanelPage { space_id }
    }
}
