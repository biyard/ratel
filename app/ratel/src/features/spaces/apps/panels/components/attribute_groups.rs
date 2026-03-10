use super::*;

translate! {
    AttributeGroupsTranslate;

    attribute_groups: {
        en: "Attribute groups",
        ko: "속성 그룹",
    },
    university: {
        en: "University",
        ko: "대학교",
    },
    gender: {
        en: "Gender",
        ko: "성별",
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PanelOption {
    University,
    Gender,
}

impl PanelOption {
    fn label(self, tr: &AttributeGroupsTranslate) -> String {
        match self {
            Self::University => tr.university.to_string(),
            Self::Gender => tr.gender.to_string(),
        }
    }
}

fn is_selected_option(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> bool {
    panels
        .iter()
        .any(|panel| match (option, &panel.attributes) {
            (
                PanelOption::University,
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::University),
            ) => true,
            (
                PanelOption::Gender,
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender),
            ) => true,
            (
                PanelOption::Gender,
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)),
            ) => true,
            _ => false,
        })
}

fn option_keys(option: PanelOption, panels: &[SpacePanelQuotaResponse]) -> Vec<DeletePanelKey> {
    panels
        .iter()
        .filter(|panel| match (option, &panel.attributes) {
            (
                PanelOption::University,
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::University),
            ) => true,
            (
                PanelOption::Gender,
                PanelAttribute::CollectiveAttribute(CollectiveAttribute::Gender),
            ) => true,
            (
                PanelOption::Gender,
                PanelAttribute::VerifiableAttribute(VerifiableAttribute::Gender(_)),
            ) => true,
            _ => false,
        })
        .map(|panel| DeletePanelKey {
            panel_id: panel.panel_id.clone(),
        })
        .collect()
}

fn default_attributes(option: PanelOption, total_quota: i64) -> Vec<PanelAttributeWithQuota> {
    match option {
        PanelOption::University => {
            vec![PanelAttributeWithQuota::CollectiveAttribute(
                CollectiveAttribute::University,
            )]
        }
        PanelOption::Gender => {
            let default_quota = (total_quota.max(0) + 1) / 2;
            vec![
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Gender(Gender::Male),
                    quota: default_quota,
                }),
                PanelAttributeWithQuota::VerifiableAttribute(VerifiableAttributeWithQuota {
                    attribute: VerifiableAttribute::Gender(Gender::Female),
                    quota: default_quota,
                }),
            ]
        }
    }
}

#[component]
pub fn AttributeGroups(
    space_id: SpacePartition,
    panels: Vec<SpacePanelQuotaResponse>,
    current_quota: i64,
    panels_query_key: Vec<String>,
) -> Element {
    let tr: AttributeGroupsTranslate = use_translate();
    let has_university = is_selected_option(PanelOption::University, &panels);
    let has_gender = is_selected_option(PanelOption::Gender, &panels);
    let panels_query_key_for_university = panels_query_key.clone();
    let panels_query_key_for_gender = panels_query_key.clone();

    let on_toggle_university = {
        let space_id = space_id.clone();
        let keys = option_keys(PanelOption::University, &panels);
        move |_| {
            let space_id = space_id.clone();
            let panels_query_key = panels_query_key_for_university.clone();
            let keys = keys.clone();
            spawn(async move {
                let result = if has_university {
                    delete_panel_quotas(space_id.clone(), DeletePanelQuotaRequest { keys })
                        .await
                        .map(|_| ())
                } else {
                    create_panel_quotas(
                        space_id.clone(),
                        CreatePanelQuotaRequest {
                            attributes: default_attributes(PanelOption::University, current_quota),
                        },
                    )
                    .await
                    .map(|_| ())
                };

                match result {
                    Ok(_) => invalidate_query(&panels_query_key),
                    Err(err) => error!("Failed to toggle university panel: {:?}", err),
                }
            });
        }
    };
    let on_toggle_gender = {
        let space_id = space_id.clone();
        let keys = option_keys(PanelOption::Gender, &panels);
        move |_| {
            let space_id = space_id.clone();
            let panels_query_key = panels_query_key_for_gender.clone();
            let keys = keys.clone();
            spawn(async move {
                let result = if has_gender {
                    delete_panel_quotas(space_id.clone(), DeletePanelQuotaRequest { keys })
                        .await
                        .map(|_| ())
                } else {
                    create_panel_quotas(
                        space_id.clone(),
                        CreatePanelQuotaRequest {
                            attributes: default_attributes(PanelOption::Gender, current_quota),
                        },
                    )
                    .await
                    .map(|_| ())
                };

                match result {
                    Ok(_) => invalidate_query(&panels_query_key),
                    Err(err) => error!("Failed to toggle gender panels: {:?}", err),
                }
            });
        }
    };

    rsx! {
        div { class: "flex items-center gap-5 min-w-0 flex-1",
            div { class: "text-sm font-medium text-text-primary whitespace-nowrap",
                {tr.attribute_groups}
            }
            div { class: "flex flex-wrap items-center gap-2 min-w-0",
                AttributeButton {
                    label: PanelOption::University.label(&tr),
                    selected: has_university,
                    onclick: on_toggle_university,
                }
                AttributeButton {
                    label: PanelOption::Gender.label(&tr),
                    selected: has_gender,
                    onclick: on_toggle_gender,
                }
            }
        }
    }
}
